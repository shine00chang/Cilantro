use super::*;
use std::collections::HashSet;

impl Node {
    /// Recursively removes grammatical elements from the syntax tree.
    /// Could collapse the element if it contains only one value.
    pub fn trim (mut self) -> Elem {
        match self.t {
            NodeT::Declaration => {
                self.filter_tok(vec![TokenT::K_LET, TokenT::EQ_1]) 
                    .recurse()
                    .cast()
            },
            NodeT::Return => {
                self.filter_tok(vec![TokenT::K_RETURN])
                    .recurse()
                    .cast()
            },
            NodeT::Invoke => {
                self = self
                    .filter_tok(vec![TokenT::PAREN_L, TokenT::PAREN_R])
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
                self.filter_tok(vec![TokenT::COMMA])
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
                self.filter_tok(vec![TokenT::PAREN_L, TokenT::PAREN_R])
                    .change_t(NodeT::Expr)
                    .recurse()
                    .collapse_if_1()
            },
            NodeT::Statement => {
                self.recurse()
                    .collapse_if_1()
            },
            NodeT::Function => {
                self.filter_tok(vec![TokenT::K_FUNC, TokenT::PAREN_L, TokenT::PAREN_R, TokenT::ARROW])
                    .recurse()
                    .cast()
            },
            NodeT::Block => {
                self = self
                    .filter_tok(vec![TokenT::CURLY_L, TokenT::CURLY_R])
                    .recurse();

                // Consume child "List", set its children as own.
                if let Elem::Node(list) = self.children.pop().unwrap() {
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
                self.filter_tok(vec![TokenT::COMMA, TokenT::COLON])
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

        /*
        // Pop second, then extend first's children onto self.
        if let Elem::Node(node) = &mut self.children[0] {
            node.children.extend(self.children.);
        }
        println!("compacting: {:?}", self.children);
        let last = self.children.pop().unwrap();
        let elem = self.children
            .pop()
            .expect("Did not find 2 children. This should mean that the grammar of this node does not follow the list pattern.");
        if let Elem::Node(node) = elem {
            if node.t != self.t {
                self.children.extend(node.children);
            }
        }
        self.children.push(last);
        */
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


