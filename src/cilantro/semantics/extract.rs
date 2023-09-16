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
            /*
            NodeT::Return => {
                // Get Expression
                let expr = self.children.pop().unwrap();
                assert!(expr.t().is_evaluable());
                match expr { 
                    Elem::Node(n)  => children.push(LElem::Node(n.extract())),
                    Elem::Token(t) => children.push(LElem::Token(t))
                }

                let expr = ChildRef::new(0);

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
                        children.push(LElem::Node(n.extract()));
                        Some(ChildRef::new(0))
                    } else { panic!() }
                } else { None };
                NodeData::Invoke { ident, args }
            },
            NodeT::Args => {
                for child in self.children.into_iter() {
                    let n = match child {
                        Elem::Node(n)  => {
                            n.t.is_evaluable();
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
            */
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
            /*
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
                        children.push(LElem::Node(n.extract()));
                        let out = Some(ChildRef::new(ref_i));
                        ref_i += 1;
                        i += 1;
                        out
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
                    children.push(LElem::Node(n.extract()));
                    let out =ChildRef::new(ref_i);
                    ref_i += 1;
                    i += 1;
                    out
                } else { panic!() };

                NodeData::Function {
                    ident,
                    params,
                    r_type,
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
            */
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
