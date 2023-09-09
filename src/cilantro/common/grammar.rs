use super::*;
use strum_macros::{EnumIter, EnumDiscriminants};



#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Debug, Clone, EnumIter, EnumDiscriminants)]
#[strum_discriminants(derive(Hash, EnumIter, EnumIs))]
#[strum_discriminants(name(TokenT))]
#[strum_discriminants(allow(non_camel_case_types))]
pub enum TokenData {
    EOF,

    a(char),
    b(char),
    x,

    K_LET,
    K_FUNC,
    EQ_1,
    EQ_2,
    IDENT(String),
    INT(i32),
    BOOL(bool),
    NUMOP_1(String),
    NUMOP_2(String),
    
    PAREN_L,
    PAREN_R,
    CURLY_L,
    CURLY_R,
}


#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(Hash, EnumIter, EnumIs))]
#[strum_discriminants(name(NodeT))]
pub enum NodeData {
    // FIXED
    ROOT,

    A { c: char },
    S { x: usize },

    Statement,
    Function {
        ident: String,
        params: ChildRef,
        block: ChildRef,
    },
    Block,
    List,
    Declaration { 
        ident: String,
        expr: ChildRef,
    },
    Expr {
        t1: ChildRef,
        t2: ChildRef,
        op: String, 
    },
    T1, 
    T2, 
    Invoke {
        ident: String,
        args: ChildRef,
    },
    Args,
    Params { v: Vec<String> },
}


impl Productions {
    pub fn make () -> Self {
        let apices = vec![
            NodeT::Statement,
            NodeT::Function,
            NodeT::Block
        ];
        let v = vec![
            (
                NodeT::Block,
                vec![ vec![
                    ElemT::Token(TokenT::CURLY_L),
                    ElemT::Node(NodeT::List),
                    ElemT::Token(TokenT::CURLY_R)
                ] ]
            ),
            (
                NodeT::List,
                vec![ 
                    vec![ElemT::Node(NodeT::Statement)],
                    vec![
                        ElemT::Node(NodeT::List),
                        ElemT::Node(NodeT::Statement),
                    ]
                ]
            ),
            ( 
                NodeT::Params,
                vec![ vec![
                    ElemT::Token(TokenT::IDENT)
                ] ]
            ),
            (
                NodeT::Function,
                vec![ vec![
                    ElemT::Token(TokenT::K_FUNC),
                    ElemT::Token(TokenT::IDENT),
                    ElemT::Token(TokenT::PAREN_L),
                    ElemT::Node(NodeT::Params),
                    ElemT::Token(TokenT::PAREN_R),
                    ElemT::Node(NodeT::Block),
                ] ]
            ),
            (
                NodeT::Statement,
                vec![
                    vec![ElemT::Node(NodeT::Declaration)],
                    vec![ElemT::Node(NodeT::Block)],
                    vec![ElemT::Node(NodeT::Invoke)]
                ]
            ),
            ( 
                NodeT::Declaration,
                vec![ vec![ 
                    ElemT::Token(TokenT::K_LET),
                    ElemT::Token(TokenT::IDENT),
                    ElemT::Token(TokenT::EQ_1),
                    ElemT::Node(NodeT::Expr)
                ] ]
            ),
            (
                NodeT::Invoke,
                vec![ vec![
                    ElemT::Token(TokenT::IDENT),
                    ElemT::Token(TokenT::PAREN_L),
                    ElemT::Node(NodeT::Args),
                    ElemT::Token(TokenT::PAREN_R)
                ] ]
            ),
            ( 
                NodeT::Args,
                vec![ vec![
                    ElemT::Node(NodeT::Expr)
                ] ]
            ),
            // Expressions
            ( 
                NodeT::Expr,
                vec![
                    vec![ ElemT::Node(NodeT::T1), ElemT::Token(TokenT::NUMOP_1), ElemT::Node(NodeT::Expr)],
                    vec![ ElemT::Node(NodeT::T1) ]
                ]
            ),
            ( 
                NodeT::T1,
                vec![ 
                    vec![ ElemT::Node(NodeT::T2), ElemT::Token(TokenT::NUMOP_2), ElemT::Node(NodeT::T1)],
                    vec![ ElemT::Node(NodeT::T2) ]
                ],
            ),
            ( 
                NodeT::T2,
                vec![
                    vec![ ElemT::Token(TokenT::INT) ],
                    vec![ ElemT::Token(TokenT::IDENT) ],
                    vec![ ElemT::Token(TokenT::PAREN_L), ElemT::Node(NodeT::Expr), ElemT::Token(TokenT::PAREN_R) ]
                ],
            ),
        ];
        Self::new(apices, v)
    }
    
    #[cfg(test)]
    pub fn make_test () -> Self {
        let apices = vec![
            NodeT::S
        ];
        let v = vec![
            (
                NodeT::S,
                vec![ vec![ ElemT::Node(NodeT::A), ElemT::Node(NodeT::A) ] ]
            ),
            ( 
                NodeT::A,
                vec![ 
                    vec![ ElemT::Token(TokenT::b) ],
                    vec![ ElemT::Token(TokenT::a), ElemT::Node(NodeT::A) ]
                ]
            )
        ];
        Self::new(apices, v)
    }
}
