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
    NUMOP_1(String),
    NUMOP_2(String),
    
    PAREN_L,
    PAREN_R,
}



#[derive(PartialEq, Eq, Debug, Clone, EnumIter, EnumDiscriminants)]
#[strum_discriminants(derive(Hash, EnumIter))]
#[strum_discriminants(name(NodeT))]
pub enum NodeData {
    // FIXED
    ROOT,

    A { c: char },
    S { x: usize },

    Declaration,
    Expr,
    T1, 
    T2, 
    Invoke,
    Args,
}


impl Node { 
    pub fn make (t: &NodeT, v: Vec<Elem>) -> Result<Node, &'static str> {
        let data = match t {
            // Testing types
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
            },
            
            NodeT::Declaration => {
                NodeData::Declaration
            },
            NodeT::Expr => {
                NodeData::Expr
            },
            NodeT::T1 => {
                NodeData::T1
            },
            NodeT::T2 => {
                NodeData::T2
            },
            NodeT::Invoke => {
                NodeData::Invoke
            },
            NodeT::Args => {
                NodeData::Args
            },
            NodeT::ROOT => {
                NodeData::ROOT
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
            NodeT::Declaration,
            NodeT::Invoke
        ];
        let v = vec![
            Production {
                node: NodeT::Declaration,
                v: vec![ 
                    ElemT::Token(TokenT::K_LET),
                    ElemT::Token(TokenT::IDENT),
                    ElemT::Token(TokenT::EQ_1),
                    ElemT::Node(NodeT::Expr)
                ]
            },
            Production {
                node: NodeT::Invoke,
                v: vec![ 
                    ElemT::Token(TokenT::IDENT),
                    ElemT::Token(TokenT::PAREN_L),
                    ElemT::Node(NodeT::Args),
                    ElemT::Token(TokenT::PAREN_R)
                ]
            },
            Production {
                node: NodeT::Args,
                v: vec![ 
                    ElemT::Node(NodeT::Expr)
                ]
            },
            // Expressions
            Production {
                node: NodeT::Expr,
                v: vec![ ElemT::Node(NodeT::T1), ElemT::Token(TokenT::NUMOP_1), ElemT::Node(NodeT::Expr)]
            },
            Production {
                node: NodeT::Expr,
                v: vec![ ElemT::Node(NodeT::T1) ]
            },
            Production {
                node: NodeT::T1,
                v: vec![ ElemT::Node(NodeT::T2), ElemT::Token(TokenT::NUMOP_2), ElemT::Node(NodeT::T1)]
            },
            Production {
                node: NodeT::T1,
                v: vec![ ElemT::Node(NodeT::T2) ]
            },
            Production {
                node: NodeT::T2,
                v: vec![ ElemT::Token(TokenT::INT) ]
            },
            Production {
                node: NodeT::T2,
                v: vec![ ElemT::Token(TokenT::IDENT) ]
            },
            Production {
                node: NodeT::T2,
                v: vec![ ElemT::Token(TokenT::PAREN_L), ElemT::Node(NodeT::Expr), ElemT::Token(TokenT::PAREN_R) ]
            },
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
