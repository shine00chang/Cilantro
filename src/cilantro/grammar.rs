use super::*;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumDiscriminants};



#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Debug, Clone, EnumIter, EnumDiscriminants)]
#[strum_discriminants(derive(Hash, EnumIter))]
#[strum_discriminants(name(TokenT))]
#[strum_discriminants(allow(non_camel_case_types))]
pub enum TokenData {
    EOF,

    a(char),
    b(char),
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



#[derive(PartialEq, Eq, Debug, Clone, EnumIter, EnumDiscriminants)]
#[strum_discriminants(derive(Hash, EnumIter))]
#[strum_discriminants(name(NodeT))]
pub enum NodeData {
    A { c: char },
    S { x: usize },
}


impl Node { 
    pub fn make (t: &NodeT, v: Vec<Elem>) -> Result<Node, &'static str> {
        let data = match t {
            NodeT::A => {
                let c = if let Elem::Token(t) = &v[0] {
                    match t.data {
                        TokenData::a(c) => c,
                        TokenData::b(c) => c,
                        _ => return Err("false")
                    }
                } else { return Err("false") };
                NodeData::A { c }
            },
            NodeT::S => {
                NodeData::S { x: 10 }
            }
        };
        Ok(Node {
            data,
            children: v,
        })
    }
}

use std::collections::{HashMap, HashSet};
impl Productions {
    pub fn make () -> Self {
        let apices = vec![
            NodeT::S
        ];
        let v = vec![
            Production {
                node: NodeT::S,
                v: vec![ ElemT::Node(NodeT::A), ElemT::Node(NodeT::A) ]
            },
            Production {
                node: NodeT::A,
                v: vec![ ElemT::Token(TokenT::b) ]
            },
            Production {
                node: NodeT::A,
                v: vec![ ElemT::Token(TokenT::a), ElemT::Node(NodeT::A) ]
            }
        ];
        Self::new(apices, v)
    }
    
    #[cfg(test)]
    pub fn make_test () -> Self {
        let apices = vec![
            NodeT::S
        ];
        let v = vec![
            Production {
                node: NodeT::S,
                v: vec![ ElemT::Node(NodeT::A), ElemT::Node(NodeT::A) ]
            },
            Production {
                node: NodeT::A,
                v: vec![ ElemT::Token(TokenT::b) ]
            },
            Production {
                node: NodeT::A,
                v: vec![ ElemT::Token(TokenT::a), ElemT::Node(NodeT::A) ]
            }
        ];
        Self::new(apices, v)
    }

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

        let out = follows.into_iter()
            .map(|(key, (ns, ts))| {
                let s = ns.into_iter().map(|n| ElemT::Node(n))
                    .chain(
                        ts.into_iter().map(|t| ElemT::Token(t))
                    )
                    .collect::<HashSet<_>>();
                
                (key, s)
            })
            .collect();
        out
    }
}
