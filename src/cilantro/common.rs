use strum_macros::EnumIs;

use super::*;
use std::collections::{HashMap, HashSet};


#[derive(Debug, Clone)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub data: TokenData,
}
pub type Tokens = Vec<Token>;


#[derive(Debug, Clone)]
pub struct Node {
/*
    pub start: Rc<Token>,
    pub end: Rc<Token>,
*/
    pub data: NodeData,
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
impl Elem {
    pub fn t (&self) -> ElemT {
        match self {
            Elem::Node(n)  => ElemT::Node(NodeT::from(n.data.clone())),
            Elem::Token(t) => ElemT::Token(TokenT::from(t.data.clone()))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub roots: HashSet<NodeT>,
}
impl std::fmt::Display for Production {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> ", self.node)?;
        for x in &self.v {
            write!(f, "{}", x)?;
        }
        Ok(())
    }
}

