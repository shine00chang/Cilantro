use strum_macros::EnumIs;

use super::*;
use std::rc::Rc;
use std::collections::{HashMap, HashSet};


#[derive(Debug, Clone)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub t: TokenT,
}
pub type Tokens = Vec<Token>;


#[derive(Debug, Clone)]
pub struct Node {
    pub start: Rc<Token>,
    pub end: Rc<Token>,
    pub t: NodeT,
    pub children: Vec<Elem>
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, EnumIs)]
pub enum ElemT {
    Node(NodeT),
    Token(TokenT)
}

#[derive(Debug, Clone)]
pub enum Elem {
    Node(Node),
    Token(Token),
}

#[derive(Debug, Clone)]
pub enum Action {
    Shift(usize),
    Reduce(usize)
}

pub struct Production {
    pub node: NodeT,
    pub v: Vec<ElemT>
}
pub struct Productions {
    pub v: Vec<Production>,
    pub follows: HashMap<NodeT, HashSet<ElemT>>,
    pub root: usize,
}


