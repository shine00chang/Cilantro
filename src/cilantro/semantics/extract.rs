use super::*;

impl Elem {
    fn tok_data (&self) -> &TokenData {
        if let Elem::Token(t) = self {
            &t.data
        } else { panic!() }
    }
}

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
            },
            NodeT::If => {
                // Get Block (in reverse since we are pop()'ing)
                let block = if let Elem::Node(n) = self.children.pop().unwrap() {
                    assert!(n.t.is_block());
                    let elem = LElem::Node(n.extract());
                    Box::new( elem )
                } else { panic!("'If' expected Block as last child.") };


                // Get Expression
                let expr = self.children.pop().unwrap();
                assert!(expr.t().is_evaluable());
                let expr = match expr { 
                    Elem::Node(n)  => LElem::Node(n.extract()),
                    Elem::Token(t) => LElem::Token(LToken::from(t))
                };
                let expr = Box::new(expr);
                

                NodeData::If { expr, block }
            },
            NodeT::Invoke => {
                // Get Function name
                let ident = if let Elem::Token(t) = &self.children[0] {
                    if let TokenData::IDENT(s) = &t.data {
                        s.clone()
                    } else { panic!() }
                } else { panic!() };

                // Extract Arguments
                let args = self.children
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, child)| {
                        if i < 1 { return None }
                        let elem = match child {
                            Elem::Node(n)  => {
                                assert!(n.t.is_evaluable());
                                let x = n.clone().extract();
                                LElem::Node(x)
                            },
                            Elem::Token(t) => LElem::Token(LToken::from(t.clone()))
                        };
                        Some(Box::new(elem))
                    })
                    .collect::<Vec<_>>();
                            
                NodeData::Invoke { ident, args }
            },
            NodeT::Params => {
                let mut v = vec![];
                for i in 0..self.children.len()/2 {
                    let ident = 
                        if let TokenData::IDENT(s) = self.children[2*i].tok_data() { s }
                        else { panic!() };
                    let t = 
                        if let TokenData::TYPE(t) = self.children[2*i+1].tok_data() { t }
                        else { panic!() };
                    v.push((ident.clone(), t.clone()));
                }
                NodeData::Params{ v }
            },
            NodeT::Expr => {
                let t2 = match self.children.pop().unwrap() {
                    Elem::Node(n)  => {
                        assert!(n.t.is_evaluable());
                        LElem::Node(n.extract())
                    },
                    Elem::Token(t) => LElem::Token(LToken::from(t))
                };

                let op = if let Elem::Token(t) = self.children.pop().unwrap() {
                    match t.data {
                        TokenData::OP1_b(op) |
                        TokenData::OP2_b(op) | 
                        TokenData::OP3_n(op) | 
                        TokenData::OP4_n(op) |
                        TokenData::OP_UNARY(op) => op,
                        t @ _ => panic!("Found a non-operator token in expression: {t}")
                    }
                } else { panic!() };

                let t1 = match self.children.pop().unwrap() {
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
            NodeT::UExpr => {
                let t = match self.children.pop().unwrap() {
                    Elem::Node(n)  => {
                        assert!(n.t.is_evaluable());
                        LElem::Node(n.extract())
                    },
                    Elem::Token(t) => LElem::Token(LToken::from(t))
                };
                let t = Box::new(t);

                let op = if let Elem::Token(t) = self.children.pop().unwrap() {
                    match t.data {
                        TokenData::OP_UNARY(op) => op,
                        t @ _ => panic!("Found a non-unary-operator token in expression: {t}")
                    }
                } else { panic!() };

                NodeData::UExpr{
                    op,
                    t
                }
            },
            NodeT::Function => {
                let mut i = 0;

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
