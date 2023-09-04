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
            NodeT::Invoke => {
                // Get Function name
                let ident = if let Elem::Token(t) = &self.children[0] {
                    if let TokenData::IDENT(s) = &t.data {
                        s.clone()
                    } else { panic!() }
                } else { panic!() };

                // Gen Args 
                let args_n = if let Elem::Node(n) = self.children[1].clone() {
                    assert!(n.t.is_args());
                    LElem::Node(n.extract())
                } else { panic!() };
                children.push(args_n);
                let args = ChildRef::new(0);

                NodeData::Invoke { ident, args }
            },
            NodeT::Args => {
                let mut v = vec![];
                for (i, child) in self.children.into_iter().enumerate() {
                    let n = match child {
                        Elem::Node(n)  => {
                            assert!(n.t.is_expr());
                            LElem::Node(n.extract())
                        },
                        Elem::Token(t) => LElem::Token(t)
                    };
                    children.push(n);
                    v.push(ChildRef::new(i))
                }
                NodeData::Args{ v }
           },
            NodeT::Expr => {
                let op = if let Elem::Token(t) = &self.children[1] {
                    match &t.data {
                        TokenData::NUMOP_1(op) => op,
                        TokenData::NUMOP_2(op) => op,
                        _ => panic!()
                    }.clone()
                } else { panic!() };

                let t1_n = match self.children[0].clone() {
                    Elem::Node(n)  => {
                        assert!(n.t.is_expr());
                        LElem::Node(n.extract())
                    },
                    Elem::Token(t) => LElem::Token(t)
                };
                let t2_n = match self.children[2].clone() {
                    Elem::Node(n)  => {
                        assert!(n.t.is_expr());
                        LElem::Node(n.extract())
                    },
                    Elem::Token(t) => LElem::Token(t)
                };
                children.push(t1_n);
                children.push(t2_n);
                let t1 = ChildRef::new(0);
                let t2 = ChildRef::new(1);

                NodeData::Expr{
                    t1,
                    t2,
                    op
                }
            },
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
