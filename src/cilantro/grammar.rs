use super::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, EnumIter)]
pub enum TokenT {
    a,
    b,
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



#[derive(Debug, Clone, Hash, PartialEq, Eq, EnumIter)]
pub enum NodeT {
    A, 
    S,
}

use std::collections::{HashMap, HashSet};
impl Productions {
    pub fn make () -> Self {
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
        Self::new(v)
    }
    
    #[cfg(test)]
    pub fn make_test () -> Self {
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
        Self::new(v)
    }

    fn new (v: Vec<Production>) -> Self {
        let follows = Self::make_follows(&v);

        Self {
            root: 0,
            v,
            follows
        }
    }

    /// Generates the FOLLOWS(x) set.
    fn make_follows (prods: &Vec<Production>) -> HashMap<NodeT, HashSet<ElemT>> {

        let mut begins: HashMap<_, _> = NodeT::iter()
            .map(|n| (n, (HashSet::new(), HashSet::new())))
            .collect();
        let mut follows: HashMap<_, _> = NodeT::iter()
            .map(|n| (n, (HashSet::new(), HashSet::new())))
            .collect();

        for prod in prods {
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
