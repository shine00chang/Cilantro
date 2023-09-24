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
    fn type_check (self, table: &mut TypeTable) -> Result<(LElem, Type), TypeError> {
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
                //println!("expr terms: {}, {}", t1_t, t2_t);
                if t1_t != t2_t {
                    return Err( TypeError::new(
                        t2.start(),
                        "Expression terms not of same type".to_owned(),
                        t1_t,
                        t2_t
                    ));
                }

                let t = match (t1_t, op.as_str(), t2_t) {
                    (_, "==" | "!=",_) => Type::Bool,
                    (Type::Bool, "||" | "&&"  ,Type::Bool) => Type::Bool,
                    (Type::Int, "*" | "+" | "-" | "/", Type::Int) => Type::Int,
                    (t1 @ _, op @ _, t2 @ _) => panic!("invalid operands. Cannot apply operator '{op}' on terms {t1} and {t2}")
                };

                (
                NodeData::Expr{
                    t1: Box::new(t1),
                    t2: Box::new(t2),
                    op,
                },
                t 
                )
            },
            NodeData::UExpr { t: term, op } => {
                let (term, t) = term.type_check(table)?;
                
                let t = match (t, op.as_str()) {
                    (Type::Bool, "!") => Type::Bool,
                    (t @ _, op @ _) => panic!("invalid operands. Cannot apply unary operator '{op}' on term {t}")
                };

                (
                NodeData::UExpr{
                    t: Box::new(term),
                    op,
                },
                t 
                )
            },
            NodeData::If { expr, block } => {
                let (expr, t) = expr.type_check(table)?;
                
                // Expression type must be boolean
                if t != Type::Bool {
                    return Err( TypeError::new(
                        expr.start(),
                        "If statement expression does not evaluate to a boolean".to_owned(),
                        Type::Bool,
                        t 
                    ));
                }
                
                (
                NodeData::If { 
                    expr: Box::new(expr),
                    block
                },
                t
                )
            }
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
                let sig = table.get_f(&ident);

                // Check arg length
                if args.len() != sig.0.len() {
                    return Err( TypeError::msg(
                        self.start,
                        format!("Argument lengths mismatched. expected {}, found {}", sig.0.len(), args.len())
                    ))
                }

                let args = args.into_iter().enumerate().map(|(i, arg)| {
                    let (arg, t) = arg.type_check(table)?; 
                    let sig = table.get_f(&ident);
                    if t != sig.0[i] {
                        return Err( TypeError::new(
                            self.start,
                            format!("Argument no.{i} has mismatched type."),
                            sig.0[i].clone(),
                            t
                        ))
                    }
                    Ok(Box::new(arg))
                }).collect::<Result<_, _>>()?;

                // Return function signature
                let sig = table.get_f(&ident);

                (
                NodeData::Invoke { ident, args },
                sig.1.clone()
                )
            }
            NodeData::Block { v } => {
                // TODO: Last one determines type.
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

                // Extract Parameter Types, Add to signature and type table.
                let param_t = 
                    if let Some(ref params) = params {
                        if let NodeData::Params { v } = &params.node_data() {
                            v.iter().map(|(ident, t)| {
                                table.define_v(ident, t.clone());
                                t.clone()
                            }).collect()
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
            TokenData::BOOL(_)      => Type::Bool,
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
