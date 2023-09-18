use super::*;


impl Node {
    /// Extracts values from children, labeling and storing them into the NodeData enumeration
    /// structures. 
    /// Creates ChildRef elements to refer to child elements that cannot be extracted into a
    /// primitive value.
    /// Removes any extracted children. New children array should only include those refered to by
    /// a ChildRef 
    pub fn extract (mut self) -> LNode {
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
                        assert!(n.t.is_evaluable());
                        LElem::Node(n.extract())
                    },
                    Elem::Token(t) => LElem::Token(LToken::from(t))
                };
                let expr = Box::new(expr);
                
                NodeData::Declaration { 
                    ident,
                    expr,
                }
            },
            NodeT::Return => {
                // Get Expression
                let expr = self.children.pop().unwrap();
                assert!(expr.t().is_evaluable());
                let expr = match expr { 
                    Elem::Node(n)  => LElem::Node(n.extract()),
                    Elem::Token(t) => LElem::Token(LToken::from(t))
                };
                let expr = Box::new(expr);

                NodeData::Return { expr }
            }
            NodeT::Invoke => {
                // Get Function name
                let ident = if let Elem::Token(t) = &self.children[0] {
                    if let TokenData::IDENT(s) = &t.data {
                        s.clone()
                    } else { panic!() }
                } else { panic!() };

                // Gen Args 
                let args = if self.children.len() > 1 {
                    if let Elem::Node(n) = self.children[1].clone() {
                        assert!(n.t.is_args());
                        let elem = LElem::Node(n.extract());
                        Some(Box::new(elem))
                    } else { panic!() }
                } else { None };
            
                NodeData::Invoke { ident, args }
            },
            NodeT::Args => {
                let v = self.children
                    .into_iter()
                    .map(|child| {
                        let n = match child {
                            Elem::Node(n)  => {
                                n.t.is_evaluable();
                                LElem::Node(n.extract())
                            },
                            Elem::Token(t) => LElem::Token(LToken::from(t))
                        };
                        Box::new(n)
                    })
                    .collect::<Vec<_>>();

                NodeData::Args{ v }
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
                        assert!(n.t.is_evaluable());
                        LElem::Node(n.extract())
                    },
                    Elem::Token(t) => LElem::Token(LToken::from(t))
                };
                let t2 = match self.children[2].clone() {
                    Elem::Node(n)  => {
                        assert!(n.t.is_evaluable());
                        LElem::Node(n.extract())
                    },
                    Elem::Token(t) => LElem::Token(LToken::from(t))
                };
                let t1 = Box::new(t1);
                let t2 = Box::new(t2);


                NodeData::Expr{
                    t1,
                    t2,
                    op,
                }
            },
            NodeT::Function => {
                let mut i = 0;
                let mut ref_i = 0;

                // Get symbol
                let ident = if let Elem::Token(t) = &self.children[i] {
                    if let TokenData::IDENT(s) = &t.data {
                        s.clone()
                    } else { panic!() }
                } else { panic!() };
                i += 1;

                // Get Params 
                let params = if let Elem::Node(n) = self.children[i].clone() {
                    if n.t.is_params() {
                        i += 1;
                        let elem = LElem::Node(n.extract());
                        Some( Box::new( elem ) )
                    } else { None }
                } else { None };

                // Get Return Type 
                let r_type = if let Elem::Token(t) = self.children[i].clone() {
                    if let TokenData::TYPE(t) = t.data {
                        t
                    } else { panic!() }
                } else { panic!() };
                i += 1;

                // Get Block
                let block = if let Elem::Node(n) = self.children[i].clone() {
                    assert!(n.t.is_block());
                    i += 1;
                    let elem = LElem::Node(n.extract());
                    Box::new( elem )
                } else { panic!() };

                NodeData::Function {
                    ident,
                    params,
                    r_type,
                    block,
                }
            },
            NodeT::Block => {
                let v = self.children
                    .into_iter()
                    .map(|elem| {
                        if let Elem::Node(stmt) = elem {
                            let elem = LElem::Node(stmt.extract());
                            Box::new(elem)
                        } else { panic!() }
                    })
                    .collect();
                NodeData::Block{ v }
            }
            _ => panic!("extract unimplemented for node {}", self.t)
        };
        LNode { 
            start: self.start,
            end: self.end,
            data,
            t: Type::Void
        }
    }
}
