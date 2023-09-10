use std::fmt::Write;
use core::fmt;
use super::*;


impl fmt::Display for ElemT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElemT::Node(n) => write!(f, "{n}"),
            ElemT::Token(t) => write!(f, "{t}")
        }
    }
}

impl fmt::Display for Elem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Elem::Node(n) => write!(f, "{}", n.t),
            Elem::Token(t) => write!(f, "{}", t.data)
        }
    }
}

impl fmt::Display for TokenT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl fmt::Display for NodeT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for TokenData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl fmt::Display for NodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", NodeT::from(self))
    }
}

impl fmt::Display for Node {
    /// Prints out node tree in a vertical graph. Wraps Node::ft
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ft(f, &String::from(" "))
    }
}

impl fmt::Display for LNode {
    /// Prints out node tree in a vertical graph. Wraps Node::ft
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ft(f, &String::from(" "))
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: 
        Ok(())
    }
}

impl fmt::Display for ChildRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.i)
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
            write!(f, "{p}{c}── {}\n", self.t)?;
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
                    write!(f, "{p}{c}── {}\n", t.data)?;
                }
            }
        }
        Ok(())
    }
} 

impl LNode {
    /// Prints out node tree in a vertical graph
    fn ft (&self, f: &mut fmt::Formatter<'_>, prefix: &String) -> fmt::Result {
        
        // Print self
        {
            // If the last character is │, not last child, use ├, else use └
            
            let mut p = prefix.clone();
            let c = if p.pop() == Some('│') { "├" } else { "└" };
            write!(f, "{p}{c}── {:?}\n", self.data)?;
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
                LElem::Node(n)  => n.ft(f, &np)?, 
                LElem::Token(t) => {
                    let mut p = np.clone();
                    let c = if p.pop() == Some('│') { "├" } else { "└" };
                    write!(f, "{p}{c}── {}\n", t.data)?;
                }
            }
        }
        Ok(())
    }
}


/// Visually maps tokens to the source string.
pub fn print_tokens (toks: &Tokens, source: &String) -> Result<String, std::fmt::Error> { 

    const WRAP: usize = 150;
    const K: usize = 4;

    let mut f = String::new();
    let mut char_i = 0;
    let mut tok_s = 0;
    let mut line_start = 0;

    while tok_s < toks.len() {
        let mut i = line_start;
        let mut tok_i = tok_s;

        while tok_i < toks.len() && i - line_start < WRAP {
            let tok = &toks[tok_i];
            while i < tok.start * K {
                write!(f, " ")?;
                i += 1;
            }
            if i - line_start > WRAP { break }
            let mut s = format!("{}", tok.data);
            if tok_i != toks.len() - 1 {
                let d = K * (toks[tok_i+1].start - tok.start - 1);
                s = format!("{:<width$.width$}", s, width=d);
            }
            i += s.len();

            write!(f, "{s}")?;

            tok_i += 1;
        }
        write!(f, "\n")?;
        
        let mut i = line_start;
        let mut tok_i = tok_s;

        while tok_i < toks.len() {
            let tok = &toks[tok_i];
            while i < tok.start * K {
                write!(f, " ")?;
                i += 1;
            }
            if i - line_start > WRAP { break }

            write!(f, "│")?;
            i += 1;
            
            tok_i += 1;
        }
        write!(f, "\n")?;
        
        
        while char_i < source.len() && char_i * K - line_start < WRAP {
            write!(f, "{:<width$}", &source[char_i..char_i+1].escape_debug().to_string(), width=K)?;
            char_i += 1;
        }
        write!(f, "\n")?;

        tok_s = tok_i;
        line_start = char_i * K;
    }
    Ok(f)
}



use super::parser::ParseTable;
use super::semantics::TypeError;
use strum::IntoEnumIterator;
/// Prints out & formats parsing table
pub fn print_table (table: &ParseTable) -> Result<String, std::fmt::Error> { 
    let mut f = String::new();

    // Top row. List out tokens & node types.
    write!(f, "   |")?;
    for tok in TokenT::iter() {
        write!(f, "{:8.8}|", format!("{tok}"))?;
    }
    for nod in NodeT::iter() {
        write!(f, "{:8.8}|", format!("{nod}"))?;
    }
    write!(f, "\n")?;

    // Rows
    for i in 0..table.len() {
        let row = &table[i];

        write!(f, "{:3}|", i)?;

        for tok in TokenT::iter() {
            if let Some(a) = row.get(&ElemT::Token(tok)) {
                match a {
                    Action::Shift(r)  => write!(f, "S {:6}|", r)?,
                    Action::Reduce(p) => write!(f, "R {:6}|", p)?,
                }
            } else {
                write!(f, "{:8}|", "")?;
            }
        }       

        for nod in NodeT::iter() {
            if let Some(a) = row.get(&ElemT::Node(nod)) {
                match a {
                    Action::Shift(r)  => write!(f, "S {:6}|", r)?,
                    Action::Reduce(p) => write!(f, "R {:6}|", p)?,
                }
            } else {
                write!(f, "{:8}|", "")?;
            }
        }       
        write!(f, "\n")?;
    }
    
    Ok(f)
}



#[cfg(test)]
mod test {
    use super::*;

    #[allow(non_snake_case)]
    #[test]
    fn node () {
        let n = {
            let a = Token {
                start: 0,
                end: 0,
                data: TokenData::a('a')
            };
            let b1 = Token {
                start: 0,
                end: 0,
                data: TokenData::b('b')
            };
            let b2 = Token {
                start: 0,
                end: 0,
                data: TokenData::b('b')
            };
            let A1 = Node {
                start: 0,
                end: 0,
                t: NodeT::A,
                children: vec![Elem::Token(b2)]
            };
            let A2 = Node {
                start: 0,
                end: 0,
                t: NodeT::A,
                children: vec![Elem::Token(a), Elem::Node(A1)]
            };
            let A3 = Node {
                start: 0,
                end: 0,
                t: NodeT::A,
                children: vec![Elem::Token(b1)]
            };
            Node {
                start: 0,
                end: 0,
                t: NodeT::S,
                children: vec![Elem::Node(A2), Elem::Node(A3)]
            }
        };

        let s = concat!(
                "└── S\n",
                "    ├── A\n",
                "    │   ├── a('a')\n",
                "    │   └── A\n",
                "    │       └── b('b')\n",
                "    └── A\n",
                "        └── b('b')\n",
            );

        let o = format!("{n}");
        print!("{n}");

        assert_eq!(o, s);
    }


    use crate::cilantro::tokenize;
    #[test]
    fn tokens () {
        let source = "let A = 100\n let B = 68_104".to_owned();
        let toks = tokenize(source.clone());
        let s = print_tokens(&toks, &source).unwrap();

        let res = concat!(
            "K_LET       IDENT EQ_1  INT(100)       K_LET       IDENT EQ_1  INT(68104)        EOF\n",
            "│           │     │     │              │           │     │     │                 │\n",
            "l  e  t     A     =     1  0  0  \\n    l  e  t     B     =     6  8  _  1  0  4  \n"
        );
        println!("{}", s);

        assert_eq!(s, res);
    }
}
