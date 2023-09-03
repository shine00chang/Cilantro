use super::*;
use std::collections::HashSet;

impl Node {
    pub fn trim (self) -> Elem {
        match self.t {
            NodeT::Declaration => {
                self.filter_tok(vec![TokenT::K_LET, TokenT::EQ_1]) 
                    .recurse()
                    .cast()
            },
            NodeT::Invoke => {
                self.filter_tok(vec![TokenT::PAREN_L, TokenT::PAREN_R])
                    .recurse()
                    .cast()
            },
            NodeT::Args => {
                self.recurse()
                    .cast()
            },
            NodeT::Expr => {
                self.recurse()
                    .collapse_if_1()
            },
            NodeT::T1 => {
                self.recurse()
                    .collapse_if_1()
            },
            NodeT::T2 => {
                self.filter_tok(vec![TokenT::PAREN_L, TokenT::PAREN_R])
                    .recurse()
                    .collapse_if_1()
            },
            _ => panic!("none") 
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

    /// Recusively trim
    fn recurse (mut self) -> Self {
        self.children = self.children.into_iter().map(|e| match e {
            Elem::Node(n)      => n.trim(),
            e @ Elem::Token(_) => e
        }).collect();
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


