mod grammar;
pub use grammar::{NodeT, NodeData, TokenT, TokenData};

use strum_macros::EnumIs;
use strum::IntoEnumIterator;

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
impl Productions {
    fn new (roots: Vec<NodeT>, v: Vec<Production>) -> Self {
        let mut s = Self {
            roots: roots.into_iter().collect(),
            v,
            follows: HashMap::new()
        };
        
        s.follows = s.make_follows();
        s
    }

    /// Generates the FOLLOWS(x) set.
    fn make_follows (&self) -> HashMap<NodeT, HashSet<ElemT>> {

        let mut begins: HashMap<_, _> = NodeT::iter()
            .map(|n| {
                let mut s = HashSet::new();
                s.insert(TokenT::EOF);
                (n, (HashSet::new(), s))
            })
            .collect();
        let mut follows: HashMap<_, _> = NodeT::iter()
            .map(|n| {
                if self.roots.contains(&n) {
                    let mut s = HashSet::new();
                    s.insert(TokenT::EOF);
                    (n, (HashSet::new(), s))
                } else {
                    (n, (HashSet::new(), HashSet::new()))
                }
            })
            .collect();

        for prod in &self.v {
            match &prod.v[0] {
                ElemT::Node(n) => 
                    begins.get_mut(&prod.node)
                        .unwrap().0
                        .insert(n.clone()),
                ElemT::Token(t) => 
                    begins.get_mut(&prod.node)
                        .unwrap().1
                        .insert(t.clone())
            };

            for i in 0..prod.v.len()-1 {
                if let ElemT::Node(node) = &prod.v[i] {
                    match &prod.v[i+1] {
                        ElemT::Node(n) => 
                            follows.get_mut(node)
                                .unwrap().0
                                .insert(n.clone()),
                        ElemT::Token(t) => 
                            follows.get_mut(node)
                                .unwrap().1
                                .insert(t.clone())
                    };
                } 
            }
        }

        // Expand
        loop {
            let mut mutated = false;
            for node in NodeT::iter() {
                for x in begins.get(&node).unwrap().0.clone() {
                    let to_extend = begins.get(&x).unwrap().1.clone();
                    let s = &mut begins.get_mut(&node).unwrap().1;
                    let prevl = s.len();
                    s.extend(to_extend);
                    mutated |= s.len() > prevl;
                }
            }
            if !mutated {
                break;
            }
        }
        loop {
            let mut mutated = false;
            for node in NodeT::iter() {
                for x in follows.get(&node).unwrap().0.clone() {
                    let to_extend = begins.get(&x).unwrap().1.clone();
                    let s = &mut follows.get_mut(&node).unwrap().1;
                    let prevl = s.len();
                    s.extend(to_extend);
                    mutated |= s.len() > prevl;
                }
            }
            if !mutated {
                break;
            }
        }

        follows.into_iter()
            .map(|(key, (ns, ts))| {
                let s = ns.into_iter().map(|n| ElemT::Node(n))
                    .chain(
                        ts.into_iter().map(|t| ElemT::Token(t))
                    )
                    .collect::<HashSet<_>>();
                
                (key, s)
            })
            .collect()
    }

}

