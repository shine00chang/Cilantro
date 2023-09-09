use super::*;

impl LElem {
    pub fn codegen (&self, prog: &mut Prog, func: &mut Func) {
        match self {
            LElem::Node(n)  => n.codegen(prog, func),
            LElem::Token(t) => t.codegen(prog, func)
        }
    }
}
impl LNode {
    pub fn codegen (&self, prog: &mut Prog, func: &mut Func) {
        match &self.data {
            NodeData::Declaration{ ident, expr } => {
                // Declare local variable
                func.prefix(format!("(local ${} i32)", ident));

                // Expand Expression
                func.push_s(format!("(local.set ${}", ident));
                self.get(expr).codegen(prog, func);
                func.push(")");
            },
            NodeData::Invoke{ ident, args } => {
                func.push_s(format!("(call ${}", ident));

                self.get(args).codegen(prog, func);

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
                    "+" => "(i32.add",
                    "-" => "(i32.sub",
                    "*" => "(i32.mul",
                    "/" => "(i32.div",
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
    pub fn codegen (&self, prog: &mut Prog, func: &mut Func) {
        match &self.data {
            TokenData::INT(n) => {
                func.push_s(format!("(i32.const {})", n));
            },
            TokenData::IDENT(ident) => {
                func.push_s(format!("(local.get ${})", ident));
            }
            _ => panic!("codegen unimplemented for {}", self.data)
        }
    }
}
