use core::fmt;

use crate::cilantro::{ Node, Tokens, Elem };

impl fmt::Display for Node {
    /// Prints out node tree in a vertical graph. Wraps Node::ft
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ft(f, &String::from(" "))
    }
}

impl Node {
    /// Prints out node tree in a vertical graph
    fn ft (&self, f: &mut fmt::Formatter<'_>, prefix: &String) -> fmt::Result {
        
        // Print self
        {
            // If the last character is │, not last child, use ├, else use └
            
            let mut p = prefix.clone();
            let c = if p.pop() == Some('│') { "├" } else { "└" };
            write!(f, "{p}{c}── {:?}\n", self.t)?;
        }
        
        // Update prefix
        let mut np = prefix.clone();
        np.push_str("   │");

        // For each children
        for i in 0..self.children.len() 
        {
            // If is last children, no need to inlcude line for next child in prefix
            if i == self.children.len()-1 {
                np = prefix.clone();
                np.push_str("    ");
            }

            // If child is node, recurse. Else, print.
            match &self.children[i] {
                Elem::Node(n)  => n.ft(f, &np)?, 
                Elem::Token(t) => {
                    let mut p = np.clone();
                    let c = if p.pop() == Some('│') { "├" } else { "└" };
                    write!(f, "{p}{c}── {:?}\n", t.t)?;
                }
            }
        }
        Ok(())
    }
}

use std::fmt::Write;
pub fn visualize_tokens (toks: &Tokens, source: &String) -> Result<String, std::fmt::Error> { 
    let mut f = String::new();
    let mut i = 0;
    for tok in toks {
        while i < tok.start * 3 {
            write!(f, " ")?;
            i += 1;
        }
        let mut s = format!("{:?}", tok.t);
        if let Some(t) = s.split_once('(') {
            s = String::from(t.0);
        }
        i += s.len();
        write!(f, "{s}")?;
    }
    write!(f, "\n")?;

    let mut i = 0;
    for tok in toks {
        while i < tok.start * 3 {
            write!(f, " ")?;
            i += 1;
        }
        write!(f, "│")?;
        i += 1;
    }
    write!(f, "\n")?;
    
    for c in source.chars() {
        write!(f, "{:<3}", c.escape_debug().to_string())?;
    }
    write!(f, "\n")?;
    Ok(f)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::rc::Rc;
    use crate::cilantro::{
        TokenT,
        NodeT,
        Token,
    };

    #[allow(non_snake_case)]
    #[test]
    fn test () {
        let n = {
            let t = Rc::new(Token {
                start: 0,
                end: 0,
                t: TokenT::x
            });
            let a = Token {
                start: 0,
                end: 0,
                t: TokenT::a
            };
            let b1 = Token {
                start: 0,
                end: 0,
                t: TokenT::b
            };
            let b2 = Token {
                start: 0,
                end: 0,
                t: TokenT::b
            };
            let A1 = Node {
                start: t.clone(), 
                end: t.clone(),
                t: NodeT::A,
                children: vec![Elem::Token(b2)]
            };
            let A2 = Node {
                start: t.clone(), 
                end: t.clone(),
                t: NodeT::A,
                children: vec![Elem::Token(a), Elem::Node(A1)]
            };
            let A3 = Node {
                start: t.clone(), 
                end: t.clone(),
                t: NodeT::A,
                children: vec![Elem::Token(b1)]
            };
            Node {
                start: t.clone(),
                end: t.clone(),
                t: NodeT::S,
                children: vec![Elem::Node(A2), Elem::Node(A3)]
            }
        };

        let s = concat!(
                "└── S\n",
                "    ├── A\n",
                "    │   ├── a\n",
                "    │   └── A\n",
                "    │       └── b\n",
                "    └── A\n",
                "        └── b\n",
            );

        let o = format!("{n}");
        print!("{n}");

        assert_eq!(o, s);
    }


    use crate::cilantro::tokenize;
    #[test]
    fn tokens () {
        let source = "let a = 100\n let b = 68_104".to_owned();
        let toks = tokenize(source.clone());
        let s = visualize_tokens(&toks, &source).unwrap();

        let res = concat!(
            "K_LET       IDENT EQ_1  INT            K_LET       IDENT EQ_1  INT\n",
            "│           │     │     │              │           │     │     │\n",
            "l  e  t     a     =     1  0  0  \\n    l  e  t     b     =     6  8  _  1  0  4  \n"
        );
        println!("{}", res);

        assert_eq!(s, res);
    }
}
