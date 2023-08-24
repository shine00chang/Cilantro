use super::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum TokenT {
    a,
    b,
    INT(i32),
    BOOL(bool),
    x
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
