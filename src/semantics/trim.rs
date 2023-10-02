use super::*;
use std::collections::HashSet;

use TokenT::*;
impl Node {
    /// Recursively removes grammatical elements from the syntax tree.
    /// Could collapse the element if it contains only one value.
    pub fn trim (mut self) -> Elem {
        match self.t {
            NodeT::Declaration => {
                self.filter_tok(vec![K_LET, ASSIGN]) 
                    .recurse()
                    .cast()
            },
            NodeT::If => {
                self.filter_tok(vec![K_IF])
                    .recurse()
                    .cast()
            },
            NodeT::Return => {
                self.filter_tok(vec![K_RETURN])
                    .recurse()
                    .cast()
            },
            NodeT::Invoke => {
                self = self
                    .filter_tok(vec![PAREN_L, PAREN_R])
                    .recurse();

                if self.children.len() > 1 {
                    if let Elem::Node(args) = self.children.pop().unwrap() {
                        assert!(args.t.is_args());
                        self.children.extend(args.children);
                    }
                }

                self.cast()
            },
            NodeT::Args => {
                self.filter_tok(vec![COMMA])
                    .recurse()
                    .into_list()
                    .cast()
            },
            NodeT::Expr => {
                self.recurse()
                    .collapse_if_1()
            },
            NodeT::T1 => {
                self.change_t(NodeT::Expr)
                    .recurse()
                    .collapse_if_1()
            },
            NodeT::T2 => {
                self.change_t(NodeT::Expr)
                    .recurse()
                    .collapse_if_1()
            },
            NodeT::T3 => {
                self.change_t(NodeT::Expr)
                    .recurse()
                    .collapse_if_1()
            },
            NodeT::TBase => {
                self = self.filter_tok(vec![PAREN_L, PAREN_R]);
                let t = if self.children.len() == 2 { NodeT::UExpr } else { NodeT::Expr };
                self.change_t(t)
                    .recurse()
                    .collapse_if_1()
            },
            NodeT::Statement => {
                self.recurse()
                    .collapse_if_1()
            },
            NodeT::Function => {
                self.filter_tok(vec![K_FUNC, PAREN_L, PAREN_R, ARROW])
                    .recurse()
                    .cast()
            },
            NodeT::Block => {
                self = self
                    .filter_tok(vec![CURLY_L, CURLY_R])
                    .recurse();

                // If not empty, Consume child "List", set its children as own.
                if let Some(Elem::Node(list)) = self.children.pop() {
                    assert!(list.t.is_list());
                    self.children = list.children;
                } 

                self.cast()
            },
            NodeT::List => {
                self.recurse()
                    .into_list()
                    .cast()
            },
            NodeT::Params => {
                self.filter_tok(vec![COMMA, COLON])
                    .recurse()
                    .into_list()
                    .cast() 
            }
            _ => panic!("no trimmer implemented for {}", self.t) 
        }
    }

    /// filter certain tokens 
    fn filter_tok (mut self, toks: Vec<TokenT>) -> Self {
        let set: HashSet<_> = toks.into_iter().collect();
        self.children = self.children.into_iter().filter(|c| {
            if let Elem::Token(t) = c {
                let t = TokenT::from(t.data.clone());
                !set.contains(&t)
            } else { true }
        }).collect();
        self
    }

    /// Cast into different NodeT. This is to handle when multiple grammatical nodes refer to the same logical node
    fn change_t (mut self, t: NodeT) -> Self {
        self.t = t;
        self
    }

    /// Recusively trim
    fn recurse (mut self) -> Self {
        self.children = self.children.into_iter().map(|e| match e {
            Elem::Node(n)      => n.trim(),
            e @ Elem::Token(_) => e
        }).collect();
        self
    }
    
    /// Collapse the recursive list assuming the structure: X -> XA | A
    fn into_list (mut self) -> Self {

        // See if first node is same as self.
        // If so, return the length of children. Used to reserve new children size.
        let clen = if let Some(elem) = self.children.first() {
            if let Elem::Node(node) = elem {
                if node.t != self.t {
                    return self
                } 
                node.children.len()
            } else { return self }
        } else { return self };
        
        // Fold into new children vector
        let len = self.children.len() + clen;
        self.children = self.children.into_iter().fold(vec![], |mut v, elem| {
            if v.is_empty() {
                v.reserve(len);
                if let Elem::Node(node) = elem {
                    v.extend(node.children)
                } else { panic!() };
            } else {
                v.push(elem);
            }
            v
        });

        self
    }

    /// Convert to Elem::Node for chaining
    fn cast (self) -> Elem {
        Elem::Node(self)
    }

    /// Collapse only has a single term 
    fn collapse_if_1 (self) -> Elem {
        if self.children.len() == 1 {
            self.children[0].clone()
        } else {
            Elem::Node(self)
        }    
    }
}


