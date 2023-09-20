use super::*;
use strum_macros::{EnumIter, EnumDiscriminants};


#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Type {
    Void,
    Int,
    String,
}
impl Default for Type {
    fn default() -> Self {
        Self::Void
    }
}


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
    K_RETURN,

    EQ_1,
    EQ_2,
    IDENT(String),
    INT(i64),
    STR_LIT(String),
    BOOL(bool),
    NUMOP_1(String),
    NUMOP_2(String),
    TYPE(Type),
    
    PAREN_L,
    PAREN_R,
    CURLY_L,
    CURLY_R,
    COMMA, 
    COLON,
    ARROW,
}


#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(Hash, EnumIter, EnumIs))]
#[strum_discriminants(name(NodeT))]
#[allow(dead_code)]
pub enum NodeData {
    // Tests
    A { c: char },
    S { x: usize },

    // FIXED
    ROOT,
    Statement,

    Function {
        ident: String,
        params: Option<ChildRef>,
        r_type: Type,
        block: ChildRef,
    },
    Block { v: Vec<ChildRef> },
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

    Return { expr: ChildRef },
    Invoke {
        ident: String,
        args: Vec<ChildRef>,
    },
    Args ,
    Params { v: Vec<(String, Type)> },
}

impl NodeT {
    pub fn is_evaluable (&self) -> bool {
        match self {
            NodeT::Expr | NodeT::Invoke => true,
            _ => false,
        }
    }
}
impl TokenT {
    pub fn is_evaluable (&self) -> bool {
        match self {
            TokenT::INT | TokenT::IDENT | TokenT::STR_LIT => true,
            _ => false,
        }
    }
}
impl ElemT {
    pub fn is_evaluable (&self) -> bool {
        match self {
            ElemT::Node(t)  => t.is_evaluable(),
            ElemT::Token(t) => t.is_evaluable()
        }
    }
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
                vec![ 
                    vec![
                        ElemT::Token(TokenT::IDENT),
                        ElemT::Token(TokenT::COLON),
                        ElemT::Token(TokenT::TYPE),
                    ],
                    vec![
                        ElemT::Node(NodeT::Params),
                        ElemT::Token(TokenT::COMMA),
                        ElemT::Token(TokenT::IDENT),
                        ElemT::Token(TokenT::COLON),
                        ElemT::Token(TokenT::TYPE),
                    ]
                ]
            ),
            (
                NodeT::Function,
                vec![
                    vec![
                        ElemT::Token(TokenT::K_FUNC),
                        ElemT::Token(TokenT::IDENT),
                        ElemT::Token(TokenT::PAREN_L),
                        ElemT::Node(NodeT::Params),
                        ElemT::Token(TokenT::PAREN_R),
                        ElemT::Token(TokenT::ARROW), 
                        ElemT::Token(TokenT::TYPE), 
                        ElemT::Node(NodeT::Block),
                    ],
                    vec![
                        ElemT::Token(TokenT::K_FUNC),
                        ElemT::Token(TokenT::IDENT),
                        ElemT::Token(TokenT::PAREN_L),
                        ElemT::Token(TokenT::PAREN_R),
                        ElemT::Token(TokenT::ARROW), 
                        ElemT::Token(TokenT::TYPE),
                        ElemT::Node(NodeT::Block),
                    ],
                ]
            ),
            (
                NodeT::Statement,
                vec![
                    vec![ElemT::Node(NodeT::Declaration)],
                    vec![ElemT::Node(NodeT::Block)],
                    vec![ElemT::Node(NodeT::Invoke)],
                    vec![ElemT::Node(NodeT::Return)]
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
                NodeT::Return,
                vec![ vec![ 
                    ElemT::Token(TokenT::K_RETURN),
                    ElemT::Node(NodeT::Expr)
                ] ]
            ),
            (
                NodeT::Invoke,
                vec![ 
                    vec![
                        ElemT::Token(TokenT::IDENT),
                        ElemT::Token(TokenT::PAREN_L),
                        ElemT::Node(NodeT::Args),
                        ElemT::Token(TokenT::PAREN_R)
                    ],
                    vec![
                        ElemT::Token(TokenT::IDENT),
                        ElemT::Token(TokenT::PAREN_L),
                        ElemT::Token(TokenT::PAREN_R)
                    ]
                ]
            ),
            ( 
                NodeT::Args,
                vec![ 
                    vec![ElemT::Node(NodeT::Expr)],
                    vec![ElemT::Node(NodeT::Args), ElemT::Token(TokenT::COMMA), ElemT::Node(NodeT::Expr)]
                ]
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
                    vec![ ElemT::Node(NodeT::Invoke) ],
                    vec![ ElemT::Token(TokenT::INT) ],
                    vec![ ElemT::Token(TokenT::IDENT) ],
                    vec![ ElemT::Token(TokenT::STR_LIT) ],
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
