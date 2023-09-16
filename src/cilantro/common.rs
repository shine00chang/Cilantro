mod grammar;
pub use grammar::{NodeT, NodeData, TokenT, TokenData, Type};

use strum_macros::EnumIs;
use strum::IntoEnumIterator;

use std::collections::{HashMap, HashSet};


#[derive(Debug, Clone)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub data: TokenData,
}

#[derive(Debug, Clone)]
pub struct LToken {
    pub start: usize,
    pub end: usize,
    pub data: TokenData,
    pub t: Type
}
impl LToken {
    pub fn from (tok: Token) -> Self {
        Self {
            t: Type::Void,
            start: tok.start,
            end: tok.end,
            data: tok.data,
        }
    }
}
pub type Tokens = Vec<Token>;


#[derive(Debug, Clone)]
pub struct Node {
    pub start: usize,
    pub end: usize,
    pub t: NodeT,
    pub children: Vec<Elem>
}
impl Node {
    pub fn make (t: NodeT, children: Vec<Elem>) -> Node {
        Node {
            start: children.first().unwrap().start(), 
            end: children.last().unwrap().end(),
            t,
            children,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChildRef {
    pub i: usize
}
impl ChildRef {
    pub fn new (i: usize) -> Self {
        Self { i } 
    }
}
#[derive(Debug, Clone)]
pub struct LNode {
    pub start: usize,
    pub end: usize,
    pub data: NodeData,
    pub t: Type,
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
pub enum LElem {
    Node(LNode),
    Token(LToken),
}
impl Elem {
    pub fn t (&self) -> ElemT {
        match self {
            Elem::Node(n)  => ElemT::Node(NodeT::from(n.t.clone())),
            Elem::Token(t) => ElemT::Token(TokenT::from(t.data.clone()))
        }
    }
    pub fn start (&self) -> usize {
        match self {
            Elem::Node(n)  => n.start, 
            Elem::Token(t) => t.start
        }
    }
    pub fn end (&self) -> usize {
        match self {
            Elem::Node(n)  => n.end, 
            Elem::Token(t) => t.end
        }
    }
}
impl LElem {
    pub fn start (&self) -> usize {
        match self {
            LElem::Node(n)  => n.start, 
            LElem::Token(t) => t.start
        }
    }
    pub fn end (&self) -> usize {
        match self {
            LElem::Node(n)  => n.end, 
            LElem::Token(t) => t.end
        }
    }
    pub fn downcast_node (&self) -> &LNode {
        if let Self::Node(n) = self {
            n
        } else {
            panic!("downcasting to node failed. Check callstack");
        }
    }
    pub fn t (&self) -> &Type {
        match self {
            LElem::Node(n)  => &n.t, 
            LElem::Token(t) => &t.t,
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
            write!(f, "{} ", x)?;
        }
        Ok(())
    } 
}
impl std::fmt::Display for Productions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for p in &self.v {
            write!(f, "{}\n", p)?;
        }
        Ok(())
    } 
}
impl Productions {
    /// Creates a Productions instance
    /// @param roots: root elements
    /// @param productions: a vector of tuples mapping a list of productions to a node 
    fn new (roots: Vec<NodeT>, productions: Vec<(NodeT, Vec<Vec<ElemT>>)>) -> Self {

        // Create Productions from input
        let mut v = vec![];
        for (node, prods) in productions {
            for prod in prods {
                v.push(Production {
                    node,
                    v: prod
                });
            }
        }

        // Creates a temporary "ROOT" object & inserts productions 
        // to make the generation of the FOLLOWS(X) set easier 
        let mut i = 0;
        for root in &roots {
            let p = Production { node: NodeT::ROOT, v: vec![
                ElemT::Node(root.clone()),
                ElemT::Node(NodeT::ROOT)
            ]};
            v.push(p);
            i += 1;
        }

        let mut s = Self {
            roots: vec![NodeT::ROOT].into_iter().collect(),
            v,
            follows: HashMap::new()
        };
        
        // Remove ROOT
        s.follows = s.make_follows();
        s.roots = roots.into_iter().collect();
        s.v.truncate(s.v.len()-i);
        s
    }

    /// Generates the FOLLOWS(x) set.
    fn make_follows (&self) -> HashMap<NodeT, HashSet<ElemT>> {

        // Algorithm:
        // -> Find all elements that a node can begin with.
        // -> Find all elements that a node can end with.
        // -> For every node J that starts node I, Add all tokens that could start J to I.
        // -> For every node J that follows node I, Add all tokens that could start J to I.

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
        for prod in &self.v {
            let s = follows.get(&prod.node).unwrap().1.clone();
            if let ElemT::Node(n) = prod.v.last().unwrap() {
                follows.get_mut(n)
                    .unwrap().1
                    .extend(s);
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


use std::fs::read_to_string;
const LIBPATH: &'static str = "lib.wat";
/// Returns the native library as a string.
pub fn get_lib () -> String {
    read_to_string(LIBPATH) 
        .expect(&format!("could not find library file '{}'", LIBPATH))
        .lines()
        .fold(String::new(), |mut s, line| { s.push_str(line); s.push('\n'); s })
}
