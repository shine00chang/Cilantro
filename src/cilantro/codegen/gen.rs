use super::*;

impl LElem {
    fn codegen (&self, prog: &mut Prog, func: &mut Func) {
        match self {
            LElem::Node(n)  => n.codegen(prog, func),
            LElem::Token(t) => t.codegen(prog, func)
        }
    }
}
impl LNode {
    pub fn codegen (&self, prog: &mut Prog, func: &mut Func) {
        match &self.data {
            NodeData::Function { ident, params, r_type, block } => {
                // Create function
                let mut func = Func::new(format!("func ${}", ident));
                
                // Params
                if let Some(params) = params { 
                    params.codegen(prog, &mut func);
                }

                // Write return type
                match r_type {
                    Type::Int    => func.prefix("(result i64)".to_owned()),
                    Type::Bool   => func.prefix("(result i32)".to_owned()),
                    Type::String => func.prefix("(result i64)".to_owned()),
                    Type::Void   => ()
                }               

                // Block
                block.codegen(prog, &mut func);

                prog.add_func(func);
            },
            NodeData::Params{ v } => {
                for (ident, t) in v {
                    func.prefix(format!("(param ${ident} {})", t.gen()));
                }
            },
            NodeData::Block { v } => {
                for child in v {
                    child.codegen(prog, func);
                }
            },
            NodeData::Declaration{ ident, expr } => {

                let expr_t = expr.t();

                // Declare local variable
                func.prefix(format!("(local ${ident} {})", expr_t.gen()));

                // Expand Expression
                func.push_s(format!("(local.set ${}", ident));
                expr.codegen(prog, func);
                func.push(")");
            },
            NodeData::If{ expr, block } => {

                if let LElem::Node(expr) = &**expr {
                    expr.codegen(prog, func);
                } else { panic!() };

                func.push("(if");
                func.push("(then");
                block.codegen(prog, func);
                func.push(")");
                func.push(")");
            },
            NodeData::Return { expr } => {
                expr.codegen(prog, func);
            },
            NodeData::Invoke { ident, args } => {
                func.push_s(format!("(call ${}", ident));

                for arg in args {
                    arg.codegen(prog, func);
                }

                func.push(")");
            },
            NodeData::UExpr { op, t } => {
                match op.as_str() {
                    "!" => {
                        func.push("(i32.ne");
                        func.push("(i32.const 1)");
                        t.codegen(prog, func);
                        func.push(")");
                    },
                    op @ _ => panic!("found unimplemented unary operator: {op}")
                }
            },
            NodeData::Expr{ op, t1, t2 } => {

                // Equality 
                if matches!(op.as_str(), "==" | "!=") {
                    func.push_s(format!(
                        "({}.{}",
                        t1.t().gen(),
                        if &op[..] == "==" { "eq" } else { "ne" }
                    ));
                    t1.codegen(prog, func);
                    t2.codegen(prog, func);
                    func.push(")");
                    
                    return
                }

                match self.t {
                    Type::Int => {
                        let a = match op.as_str() {
                            "+" => "(i64.add",
                            "-" => "(i64.sub",
                            "*" => "(i64.mul",
                            "/" => "(i64.div",
                            op @ _ => panic!("found unimplemented integer operator: {op}")
                        };
                        func.push(a);
                        t1.codegen(prog, func);
                        t2.codegen(prog, func);
                        func.push(")");
                    },
                    Type::Bool => {
                        let a = match op.as_str() {
                            "||" => Some("(i32.or"),
                            "&&" => Some("(i32.and"),
                            op @ _ => panic!("found unimplemented boolean operator: {op}")
                        };
                        func.push("(i32.ge_u");
                        if let Some(a) = a { func.push(a) } 
                        t1.codegen(prog, func);
                        t2.codegen(prog, func);
                        if let Some(_) = a { 
                            func.push("(i32.const 1)");
                            func.push(")");
                        } 
                        func.push(")");
                    },
                    _ => panic!("Expressions not implemented for type {}", self.t)
                }
            },
            _ => panic!("codegen unimplemented for {}", self.data)
        }
    }
}

impl LToken {
    fn codegen (&self, prog: &mut Prog, func: &mut Func) {
        match &self.data {
            TokenData::INT(n) => {
                func.push_s(format!("(i64.const {})", n));
            },
            TokenData::BOOL(b) => {
                func.push_s(format!("(i32.const {})", if *b { 1 } else { 0 }));
            },

            TokenData::IDENT(ident) => {
                func.push_s(format!("(local.get ${})", ident));
            }
            TokenData::STR_LIT(str) => {
                // Make literal in linear memory
                let ptr = prog.add_str_lit(str);

                // Write string pointer representation
                func.push(&format!("(i64.const {})", str.len()));
                func.push("(i64.const 32)");
                func.push("(i64.rotr)");
                func.push(&format!("(i64.const {})", ptr));
                func.push("(i64.add)");
            }
            _ => panic!("codegen unimplemented for {}", self.data)
        }
    }
}

impl Type {
    fn gen (&self) -> &str {
        match self {
            Type::Int    => "i64",
            Type::Bool   => "i32",
            Type::String => "i64",
            _ => panic!("codegen unimplemented for type {}", self)
        }
    }
}
