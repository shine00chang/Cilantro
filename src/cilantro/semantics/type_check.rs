use super::*;
use std::collections::HashMap;

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
            panic!("function type not found for '{}'. Should've been caught in scope annotations", ident);
        }    
    }
}

/// Type checking for nodes
pub fn type_check (nodes: &Vec<LNode>) -> Result<(), TypeError> { 
    let mut table = TypeTable::with_std();

    for node in nodes {
        node.type_check(&mut table).map(|_| ())?
    }
    Ok(())
}

impl LElem {
    fn type_check (&self, table: &mut TypeTable) -> Result<Type, TypeError> {
        match self {
            LElem::Token(t) => Ok(t.type_check(table)),
            LElem::Node(n)  => n.type_check(table)
        }
    }
}

static mut CURRENT_FUNC: Option<String> = None;
impl LNode {
    /// Uses a type table to ensure type correctness of program.
    /// Does not need to bother with scoping issues. Resolved already.
    fn type_check (&self, table: &mut TypeTable) -> Result<Type, TypeError> {
        match &self.data {
            NodeData::Expr { t1, t2, op: _ } => {
                let t1_t = self.get(t1).type_check(table)?;
                let t2_t = self.get(t2).type_check(table)?;
                if t1_t != t2_t {
                    return Err( TypeError::new(
                        self.get(t2).start(),
                        "Expression terms not of same type".to_owned(),
                        t1_t,
                        t2_t
                    ));
                }
                Ok(t1_t)
            },
            NodeData::Return { expr } => {
                let t = self.get(expr).type_check(table)?;

                // Check type with function signature
                unsafe { 
                    let ident = CURRENT_FUNC.clone()
                        .expect("current func undefined, yet a 'return' statement found. Should've been caught on scope checking.");

                    let out_t = &table.get_f(&ident).1;
                    if *out_t != t {
                        return Err( TypeError::new(
                            self.get(expr).start(),
                            "Return expression does not match function signature".to_owned(),
                            out_t.clone(),
                            t 
                        ));
                    }
                }
                Ok(t)
            }
            NodeData::Invoke { ident, args } => {

                // Check Args
                if let Some(args) = args {

                    // Check arg length
                    let args = self.get(args).downcast_node();
                    let sig = table.get_f(ident);
                    if args.children.len() != sig.0.len() {
                        return Err( TypeError::msg(
                            self.start,
                            format!("Argument lengths mismatched. expected {}, found {}", sig.0.len(), args.children.len())
                        ))
                    }

                    // Check each arg's type
                    for (i, arg) in args.children.iter().enumerate() {
                        let arg_t = arg.type_check(table)?;
                        let sig = table.get_f(ident);
                        if arg_t != sig.0[i] {
                            return Err( TypeError::new(
                                arg.start(),
                                "Argument type unexpected".to_owned(),
                                sig.0[i].clone(),
                                arg_t
                            ));
                        }
                    }
                }
                // Return function signature
                let sig = table.get_f(ident);
                Ok(sig.1.clone())
            }
            NodeData::Block => {
                // TODO: Iterate through statements. Last one determines type.
                Ok(Type::Void)
            }
            NodeData::Declaration { ident, expr } => {
                // Set type for ident
                let t = self.get(expr).type_check(table)?;
                table.define_v(ident, t);

                Ok(Type::Void)
            },
            NodeData::Function { ident, params, r_type, block } => {

                // Get parameter types 
                let param_t = if let Some(params) = params {
                    let params = self.get(params).downcast_node();
                    params.children.iter().map(|e| e.type_check(table)).collect::<Result<_, _>>()?
                } else { vec![] };

                // Set signature
                let sig = (param_t, r_type.clone());
                table.define_f(ident, sig);

                // Recurse into block
                self.get(block).type_check(table)?;

                Ok(Type::Void)
            }
            data @ _ => panic!("Typechecking unimplemented for {}", NodeT::from(data))
        }
    }
}

impl Token {
    fn type_check (&self, table: &mut TypeTable) -> Type {
        match &self.data {
            TokenData::INT(_) => {
                Type::Int
            },
            TokenData::IDENT(ident) => {
                table.get_v(ident).clone()
            }
            data @ _ => panic!("Typing unimplemented for token {}", TokenT::from(data))
        }
    }
}
