use std::rc::Rc;
pub use super::grammar::{NodeT, TokenT};


#[derive(Debug, Clone)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub t: TokenT,
}


#[derive(Debug, Clone)]
pub struct Node {
    pub start: Rc<Token>,
    pub end: Rc<Token>,
    pub t: NodeT,
    pub children: Vec<Elem>
}

#[derive(Debug, Clone)]
pub enum ElemT {
    Node(NodeT),
    Token(TokenT)
}

#[derive(Debug, Clone)]
pub enum Elem {
    Node(Node),
    Token(Token),
}
