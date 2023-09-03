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
                func.push(format!("(local.set ${}", ident));
                self.get(expr).codegen(prog, func);
                func.push(")".to_owned());
            },
            /*
            NodeData::Invoke => {
                // Call
                let ident = if let Elem::Token(t) = &self.children[0] {
                    if let TokenData::IDENT(s) = &t.data {
                        s
                    } else { panic!() }
                } else { panic!() };
                func.push(format!("(call ${}", ident));

                // Gen Args 
                if let Elem::Node(n) = &self.children[1] {
                    n.codegen(prog, func);
                }
                func.push(")".to_owned());
            },
            NodeData::Args => {
                // Expand each children 
                for c in &self.children {
                    c.codegen(prog, func);
                }
            },
            NodeData::Expr => {
                let op = if let Elem::Token(t) = &self.children[1] {
                    match &t.data {
                        TokenData::NUMOP_1(op) => op,
                        TokenData::NUMOP_2(op) => op,
                        _ => panic!()
                    }
                } else { panic!() };

                let a = match &op[..] {
                    "+" => "(i32.add",
                    "-" => "(i32.sub",
                    "*" => "(i32.mul",
                    "/" => "(i32.div",
                    _ => panic!()
                };
            },
            */
            _ => panic!("codegen unimplemented")
        }
    }
}

impl Token {
    pub fn codegen (&self, prog: &mut Prog, func: &mut Func) {
        match self.data {
            TokenData::INT(n) => {
                func.push(format!("(i32.const {})", n));
            }
            _ => panic!("codegen unimplemented")
        }
    }
}
