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
                    self.get(params).codegen(prog, &mut func);
                }

                // Write return type
                match r_type {
                    Type::Int => func.prefix("(result i64)".to_owned()),
                    Type::Void => (),
                    Type::String => panic!("String output not implemented")
                }               

                // Block
                self.get(block).codegen(prog, &mut func);

                prog.add_func(func);
            },
            NodeData::Params{ v }=> {
                for param in v {
                    func.prefix(format!("(param ${param} i64)"))
                }
            },
            NodeData::Block => {
                for child in &self.children {
                    child.codegen(prog, func);
                }
            }
            NodeData::Declaration{ ident, expr } => {

                let expr = self.get(expr);

                // Declare local variable
                func.prefix(format!("(local ${ident} i64)"));

                // Expand Expression
                func.push_s(format!("(local.set ${}", ident));
                expr.codegen(prog, func);
                func.push(")");
            },
            NodeData::Return{ expr } => {
                self.get(expr).codegen(prog, func);
            },
            NodeData::Invoke{ ident, args } => {
                func.push_s(format!("(call ${}", ident));

                if let Some(args) = args {
                    self.get(args).codegen(prog, func);
                }

                func.push(")");
            },
            NodeData::Args => {
                // Expand each children 
                for elem in &self.children {
                    elem.codegen(prog, func);
                }
            },
            NodeData::Expr{ op, t1, t2 } => {
                let a = match &op[..] {
                    "+" => "(i64.add",
                    "-" => "(i64.sub",
                    "*" => "(i64.mul",
                    "/" => "(i64.div",
                    _ => panic!()
                };
                func.push(a);
                self.get(t1).codegen(prog, func);
                self.get(t2).codegen(prog, func);
                func.push(")");
            },
            _ => panic!("codegen unimplemented for {}", self.data)
        }
    }
}

impl Token {
    fn codegen (&self, prog: &mut Prog, func: &mut Func) {
        match &self.data {
            TokenData::INT(n) => {
                func.push_s(format!("(i64.const {})", n));
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
