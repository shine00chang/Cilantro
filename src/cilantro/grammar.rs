use super::*;

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
