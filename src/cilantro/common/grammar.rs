use super::*;
use strum_macros::{EnumIter, EnumDiscriminants};


#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Type {
    Void,
    Int,
    Bool,
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
    K_IF,

    IDENT(String),
    ASSIGN,

    INT(i64),
    STR_LIT(String),
    BOOL(bool),

    OP1_b(String),
    OP2_b(String),
    OP3_n(String),
    OP4_n(String),
    OP_UNARY(String),
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

    If {
        expr: ChildRef,
        block: ChildRef,
    },
    Declaration { 
        ident: String,
        expr: ChildRef,
    },
    Expr {
        t1: ChildRef,
        t2: ChildRef,
        op: String, 
    },
    UExpr {
        op: String,
        t: ChildRef,
    },
    T1, T2, T3, TBase, 

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
            Expr | UExpr | Invoke => true,
            _ => false,
        }
    }
}
impl TokenT {
    pub fn is_evaluable (&self) -> bool {
        match self {
            INT | BOOL | IDENT | STR_LIT => true,
            _ => false,
        }
    }
}
impl ElemT {
    pub fn is_evaluable (&self) -> bool {
        match self {
            Node(t)  => t.is_evaluable(),
            Token(t) => t.is_evaluable()
        }
    }
}

use ElemT::Node as Node;
use ElemT::Token as Token;
use NodeT::*;
use TokenT::*;
impl Productions {
    pub fn make () -> Self {
        let apices = vec![
            Statement,
            Function,
        ];
        let v = vec![
            (
                Block,
                vec![
                    vec![ Token(CURLY_L), Node(List), Token(CURLY_R) ],
                    vec![ Token(CURLY_L), Token(CURLY_R) ]
                ]
            ),
            (
                List,
                vec![vec![ Node(List), Node(Statement) ], vec![ Node(Statement) ]]
            ),
            ( 
                Params,
                vec![ 
                    vec![ Token(IDENT), Token(COLON), Token(TYPE) ],
                    vec![ Node(Params), Token(COMMA), Token(IDENT), Token(COLON), Token(TYPE) ]
                ]
            ),
            (
                Function,
                vec![
                    vec![
                        Token(K_FUNC), Token(IDENT), Token(PAREN_L), Node(Params), Token(PAREN_R),
                        Token(ARROW), Token(TYPE), 
                        Node(Block),
                    ],
                    vec![
                        Token(K_FUNC), Token(IDENT), Token(PAREN_L), Token(PAREN_R),
                        Token(ARROW), Token(TYPE),
                        Node(Block),
                    ],
                ]
            ),
            (
                Statement,
                vec![
                    vec![Node(Declaration)],
                    vec![Node(Block)],
                    vec![Node(Invoke)],
                    vec![Node(If)],
                    vec![Node(Return)]
                ]
            ),
            ( 
                Declaration,
                vec![vec![ Token(K_LET), Token(IDENT), Token(ASSIGN), Node(Expr) ]]
            ),
            ( 
                If,
                vec![vec![ Token(K_IF), Node(Expr), Node(Block) ]]
            ),
            ( 
                Return,
                vec![vec![ Token(K_RETURN), Node(Expr) ]]
            ),
            (
                Invoke,
                vec![ 
                    vec![ Token(IDENT), Token(PAREN_L), Node(Args), Token(PAREN_R) ],
                    vec![ Token(IDENT), Token(PAREN_L), Token(PAREN_R) ]
                ]
            ),
            ( 
                Args,
                vec![ vec![Node(Args), Token(COMMA), Node(Expr)], vec![Node(Expr)] ]
            ),
            // Expressions
            ( 
                Expr,
                vec![vec![ Node(T1), Token(OP1_b), Node(Expr)], vec![ Node(T1) ]]
            ),
            ( 
                T1,
                vec![vec![ Node(T2), Token(OP2_b), Node(T1)], vec![ Node(T2) ]],
            ),
            ( 
                T2,
                vec![vec![ Node(T3), Token(OP3_n), Node(T2)], vec![ Node(T3) ]],
            ),
            ( 
                T3,
                vec![vec![ Node(TBase), Token(OP4_n), Node(T3)], vec![ Node(TBase) ]],
            ),
            ( 
                TBase,
                vec![
                    vec![ Node(Invoke) ],
                    vec![ Token(INT) ],
                    vec![ Token(BOOL) ],
                    vec![ Token(IDENT) ],
                    vec![ Token(STR_LIT) ],
                    vec![ Token(OP_UNARY), Node(TBase) ],
                    vec![ Token(PAREN_L), Node(Expr), Token(PAREN_R) ]
                ],
            ),
        ];
        Self::new(apices, v)
    }
    
    #[cfg(test)]
    pub fn make_test () -> Self {
        let apices = vec![
            S
        ];
        let v = vec![
            (
                S,
                vec![ vec![ Node(A), Node(A) ] ]
            ),
            ( 
                A,
                vec![ 
                    vec![ Token(b) ],
                    vec![ Token(a), Node(A) ]
                ]
            )
        ];
        Self::new(apices, v)
    }
}
