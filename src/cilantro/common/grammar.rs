use super::*;
use strum_macros::{EnumIter, EnumDiscriminants};



#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Debug, Clone, EnumIter, EnumDiscriminants)]
#[strum_discriminants(derive(Hash, EnumIter))]
#[strum_discriminants(name(TokenT))]
#[strum_discriminants(allow(non_camel_case_types))]
pub enum TokenData {
    EOF,

    a(char),
    b(char),
    x,

    K_LET,
    EQ_1,
    EQ_2,
    IDENT(String),
    INT(i32),
    BOOL(bool),
    PAREN_L,
    PAREN_R,
}



#[derive(PartialEq, Eq, Debug, Clone, EnumIter, EnumDiscriminants)]
#[strum_discriminants(derive(Hash, EnumIter))]
#[strum_discriminants(name(NodeT))]
pub enum NodeData {
    A { c: char },
    S { x: usize },
}


impl Node { 
    pub fn make (t: &NodeT, v: Vec<Elem>) -> Result<Node, &'static str> {
        let data = match t {
            NodeT::A => {
                let c = if let Elem::Token(t) = &v[0] {
                    match t.data {
                        TokenData::a(c) => c,
                        TokenData::b(c) => c,
                        _ => return Err("false")
                    }
                } else { return Err("false") };
                NodeData::A { c }
            },
            NodeT::S => {
                NodeData::S { x: 10 }
            }
        };
        Ok(Node {
            data,
            children: v,
        })
    }
}

impl Productions {
    pub fn make () -> Self {
        let apices = vec![
            NodeT::S
        ];
        let v = vec![
            Production {
                node: NodeT::S,
                v: vec![ ElemT::Node(NodeT::A), ElemT::Node(NodeT::A) ]
            },
            Production {
                node: NodeT::A,
                v: vec![ ElemT::Token(TokenT::b) ]
            },
            Production {
                node: NodeT::A,
                v: vec![ ElemT::Token(TokenT::a), ElemT::Node(NodeT::A) ]
            }
        ];
        Self::new(apices, v)
    }
    
    #[cfg(test)]
    pub fn make_test () -> Self {
        let apices = vec![
            NodeT::S
        ];
        let v = vec![
            Production {
                node: NodeT::S,
                v: vec![ ElemT::Node(NodeT::A), ElemT::Node(NodeT::A) ]
            },
            Production {
                node: NodeT::A,
                v: vec![ ElemT::Token(TokenT::b) ]
            },
            Production {
                node: NodeT::A,
                v: vec![ ElemT::Token(TokenT::a), ElemT::Node(NodeT::A) ]
            }
        ];
        Self::new(apices, v)
    }
}
