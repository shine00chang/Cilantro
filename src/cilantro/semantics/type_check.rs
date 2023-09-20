use super::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TypeError {
    pub start: usize,
    pub msg: String,
    pub expected: Option<Type>,
    pub found: Option<Type>,
}

impl TypeError {
    fn new (start: usize, msg: String, expected: Type, found: Type) -> Self {
        Self { start, msg, expected: Some(expected), found: Some(found) }
    }
    fn msg (start: usize, msg: String) -> Self {
        Self { start, msg, expected: None, found: None }
    }
}

impl LElem {
    pub fn node_data (&self) -> &NodeData {
        if let Self::Node(n) = self {
            &n.data
        } else {
            panic!("downcasting to node failed. Check callstack");
        }
    }
}

pub type FuncSig = (Vec<Type>, Type);
#[derive(Debug, Default)]
pub struct TypeTable {
    pub vars: HashMap<String, Type>,
    pub funcs: HashMap<String, FuncSig>
}
impl TypeTable {
    fn define_f (&mut self, ident: &String, t: (Vec<Type>, Type)) {
        if self.funcs.insert(ident.clone(), t).is_some() {
            panic!("overwriting of function type: {}", ident);
        }
    }
    fn define_v (&mut self, ident: &String, t: Type) {
        if self.vars.insert(ident.clone(), t).is_some() {
            panic!("overwriting of variable type: {}", ident);
        }
    }

    fn get_f (&self, ident: &String) -> &FuncSig {
        if let Some(t) = self.funcs.get(ident) {
            t
        } else {
            panic!("function type not found for '{}'. Should've been caught in scope annotations", ident);
        }
    }
    fn get_v (&self, ident: &String) -> &Type {
        if let Some(t) = self.vars.get(ident) {
            t
        } else {
            panic!("variable type not found for '{}'. Should've been caught in scope annotations", ident);
        }    
    }
}

/// Type checking for nodes
pub fn type_check (nodes: Vec<LNode>) -> Result<Vec<LNode>, TypeError> { 
    let mut table = TypeTable::with_std();

    nodes
        .into_iter()
        .map(|node| 
            node.type_check(&mut table).map(|(a, _)| a)
        )
        .collect::<Result<_, _>>()
}

impl LElem {
    fn type_check (mut self, table: &mut TypeTable) -> Result<(LElem, Type), TypeError> {
        match self {
            LElem::Token(tok) => {
                let (tok, t) = tok.type_check(table);
                Ok((LElem::Token(tok), t))
            },
            LElem::Node(node) => {
                node
                    .type_check(table)
                    .map(|(node, t)| (LElem::Node(node), t))
            }
        }
    }
}

static mut CURRENT_FUNC: Option<String> = None;
impl LNode {
    /// Uses a type table to ensure type correctness of program.
    /// Does not need to bother with scoping issues. Resolved already.
    fn type_check (self, table: &mut TypeTable) -> Result<(LNode, Type), TypeError> {
        let (data, t) = match self.data {
            NodeData::Expr { t1, t2, op } => {
                let (t1, t1_t) = t1.type_check(table)?;
                let (t2, t2_t) = t2.type_check(table)?;
                println!("expr terms: {}, {}", t1_t, t2_t);
                if t1_t != t2_t {
                    return Err( TypeError::new(
                        t2.start(),
                        "Expression terms not of same type".to_owned(),
                        t1_t,
                        t2_t
                    ));
                }

                (
                NodeData::Expr{
                    t1: Box::new(t1),
                    t2: Box::new(t2),
                    op,
                },
                t1_t
                )
            },
            NodeData::Return { expr } => {
                let (expr, t) = expr.type_check(table)?;

                // Check type with function signature
                unsafe { 
                    let ident = CURRENT_FUNC.clone()
                        .expect("current func undefined, yet a 'return' statement found. Should've been caught on scope checking.");

                    let out_t = &table.get_f(&ident).1;
                    if *out_t != t {
                        return Err( TypeError::new(
                            expr.start(),
                            "Return expression does not match function signature".to_owned(),
                            out_t.clone(),
                            t 
                        ));
                    }
                }
                (
                NodeData::Return { 
                    expr: Box::new(expr)
                },
                t
                )
            }
            NodeData::Invoke { ident, args } => {

                // Check Args
                if let Some(args) = args.clone() {

                    let sig = table.get_f(&ident);

                    // Extract Args
                    let args = args.node_data();
                    let args = if let NodeData::Args{ v } = &args { v } else { panic!() };

                    // Check arg length
                    if args.len() != sig.0.len() {
                        return Err( TypeError::msg(
                            self.start,
                            format!("Argument lengths mismatched. expected {}, found {}", sig.0.len(), args.len())
                        ))
                    }

                    /*
                    // TODO: Parameter/Argument Typing
                    for (i, arg) in args.iter_mut().enumerate() {
                        let (arg, t) = arg.type_check(table)?; 
                        if t != sig.0[i] {
                            return Err( TypeError::msg(
                                self.start,
                                format!("Argument lengths mismatched. expected {}, found {}", sig.0.len(), args.len())
                            ))
                        }
                    }
                    */
                }

                // Return function signature
                let sig = table.get_f(&ident);
                println!("{:?}", sig);
                (
                NodeData::Invoke { ident, args },
                sig.1.clone()
                )
            }
            NodeData::Block { v } => {
                // TODO: Iterate through statements. Last one determines type.
                let mut nv = Vec::new();
                nv.reserve(v.len());
                for stmt in v {
                    let (stmt, _) = stmt.type_check(table)?;
                    nv.push(Box::new(stmt));
                }
                (
                NodeData::Block{ v: nv },
                Type::Void
                )
            }
            NodeData::Declaration { ident, expr } => {
                // Set type for ident
                let (expr, expr_t) = expr.type_check(table)?;
                println!("declaration type: {}", expr_t);
                table.define_v(&ident, expr_t.clone());
                let expr = Box::new(expr);

                (
                NodeData::Declaration { 
                    ident,
                    expr,
                },
                Type::Void
                )
            },
            NodeData::Function { ident, params, r_type, block } => {
                
                unsafe {
                    CURRENT_FUNC = Some(ident.clone());
                }

                // TODO: parameter Typing
                let param_t = 
                    if let Some(ref params) = params {
                        let params = params.node_data();
                        if let NodeData::Params { v } = &params {
                            v.iter().map(|(_, t)| t.clone()).collect()
                            //vec![Type::Void; v.len()]
                        } else { panic!() }
                    } else { vec![] };

                // Set signature
                let sig = (param_t, r_type.clone());
                table.define_f(&ident, sig);

                // Recurse into block
                let (block, _) = block.type_check(table)?;
                let block = Box::new(block);

                (
                NodeData::Function { 
                    ident,
                    params,
                    r_type,
                    block
                },
                Type::Void
                )
            }
            data @ _ => panic!("Typechecking unimplemented for {}", NodeT::from(data))
        };
        Ok((
            LNode {
                data,
                t: t.clone(),
                ..self
            },
            t
        ))
        //Err(TypeError::new(0, "".to_owned(), Type::Int, Type::Void))
    }
}

impl LToken {
    fn type_check (self, table: &mut TypeTable) -> (LToken, Type) {
        let t = match &self.data {
            TokenData::INT(_)       => Type::Int,
            TokenData::STR_LIT(_)   => Type::String,
            TokenData::IDENT(ident) => table.get_v(ident).clone(),
            data @ _ => panic!("Typing unimplemented for token {}", TokenT::from(data))
        };
        (
            LToken {
                t: t.clone(),
                ..self
            },
            t
        )
    }
}
