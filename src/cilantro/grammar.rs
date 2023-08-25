use super::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum TokenT {
    a,
    b,
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


#[derive(Debug, Clone)]
pub enum NodeT {
    A, 
    S,
}


pub fn make_productions () -> Vec<Production> {
    vec![
        Production {
            node: NodeT::S,
            word: vec![ ElemT::Node(NodeT::A), ElemT::Node(NodeT::A) ]
        },
        Production {
            node: NodeT::A,
            word: vec![ ElemT::Token(TokenT::b) ]
        },
        Production {
            node: NodeT::A,
            word: vec![ ElemT::Token(TokenT::a), ElemT::Node(NodeT::A) ]
        }
    ]
}
