use super::*;


impl Node {
    /// Extracts values from children, labeling and storing them into the NodeData enumeration
    /// structures. 
    /// Creates ChildRef elements to refer to child elements that cannot be extracted into a
    /// primitive value.
    /// Removes any extracted children. New children array should only include those refered to by
    /// a ChildRef 
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

                let expr = match self.children[1].clone() {
                    Elem::Node(n)  => {
                        assert!(n.t.is_expr());
                        LElem::Node(n.extract())
                    },
                    Elem::Token(t) => LElem::Token(t)
                };
                children.push(expr);
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
                let args = if let Elem::Node(n) = self.children[1].clone() {
                    assert!(n.t.is_args());
                    LElem::Node(n.extract())
                } else { panic!() };
                children.push(args);
                let args = ChildRef::new(0);

                NodeData::Invoke { ident, args }
            },
            NodeT::Args => {
                for child in self.children.into_iter() {
                    let n = match child {
                        Elem::Node(n)  => {
                            assert!(n.t.is_expr());
                            LElem::Node(n.extract())
                        },
                        Elem::Token(t) => LElem::Token(t)
                    };
                    children.push(n);
                }
                NodeData::Args
            },
            NodeT::Params => {
                let mut v = vec![];
                for child in self.children.into_iter() {
                    if let Elem::Token(t) = child {
                        if let TokenData::IDENT(s) = t.data {
                            v.push(s); 
                        } else { panic!() }
                    } else { panic!() }
                }
                NodeData::Params{ v }
            },
            NodeT::Expr => {
                let op = if let Elem::Token(t) = &self.children[1] {
                    match &t.data {
                        TokenData::NUMOP_1(op) => op,
                        TokenData::NUMOP_2(op) => op,
                        _ => panic!()
                    }.clone()
                } else { panic!() };

                let t1 = match self.children[0].clone() {
                    Elem::Node(n)  => {
                        assert!(n.t.is_expr());
                        LElem::Node(n.extract())
                    },
                    Elem::Token(t) => LElem::Token(t)
                };
                let t2 = match self.children[2].clone() {
                    Elem::Node(n)  => {
                        assert!(n.t.is_expr());
                        LElem::Node(n.extract())
                    },
                    Elem::Token(t) => LElem::Token(t)
                };
                children.push(t1);
                children.push(t2);
                let t1 = ChildRef::new(0);
                let t2 = ChildRef::new(1);

                NodeData::Expr{
                    t1,
                    t2,
                    op
                }
            },
            NodeT::Function => {
                // Get symbol
                let ident = if let Elem::Token(t) = &self.children[0] {
                    if let TokenData::IDENT(s) = &t.data {
                        s.clone()
                    } else { panic!() }
                } else { panic!() };

                // Get Params 
                let params = match self.children[1].clone() {
                    Elem::Node(n)  => {
                        assert!(n.t.is_params());
                        LElem::Node(n.extract())
                    },
                    Elem::Token(t) => LElem::Token(t)
                };

                // Get block
                let block = match self.children[2].clone() {
                    Elem::Node(n)  => {
                        assert!(n.t.is_block());
                        LElem::Node(n.extract())
                    },
                    Elem::Token(t) => LElem::Token(t)
                };

                children.push(params);
                children.push(block);
                let params = ChildRef::new(0);
                let block = ChildRef::new(1);
                NodeData::Function {
                    ident,
                    params,
                    block,
                }
            },
            NodeT::Block => {
                children = self.children
                    .into_iter()
                    .map(|elem| {
                        if let Elem::Node(stmt) = elem {
                            LElem::Node(stmt.extract())
                        } else { panic!() }
                    })
                    .collect();
                NodeData::Block
            }
            _ => panic!("extract unimplemented for node {}", self.t)
        };
        LNode { 
            start: self.start,
            end: self.end,
            data,
            children
        }
    }
}
