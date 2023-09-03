use super::*;


impl Node {
    pub fn extract (self) -> LNode {
        let mut children = vec![];
        let data = match self.t {
            NodeT::Declaration => {
                // Declare local variable
                let ident = if let Elem::Token(t) = &self.children[0] {
                    if let TokenData::IDENT(s) = &t.data {
                        s.clone()
                    } else { panic!() }
                } else { panic!() };

                let expr_n = match self.children[1].clone() {
                    Elem::Node(n)  => {
                        assert!(n.t.is_expr());
                        LElem::Node(n.extract())
                    },
                    Elem::Token(t) => LElem::Token(t)
                };
                children.push(expr_n);
                let expr = ChildRef::new(0);
                
                NodeData::Declaration { 
                    ident,
                    expr
                }
            },
            /*
            NodeT::Invoke => {
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
            NodeT::Args => {
                // Expand each children 
                for c in &self.children {
                    c.codegen(prog, func);
                }
            },
            NodeT::Expr => {
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
        };
        LNode { 
            start: self.start,
            end: self.end,
            data,
            children
        }
    }
}
