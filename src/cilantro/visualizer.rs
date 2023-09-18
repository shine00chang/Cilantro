use std::fmt::Write;
use core::fmt;
use super::*;

impl LNode {
    fn get_children (&self) -> Vec<&Box<LElem>> {
        match &self.data {
            NodeData::Declaration { ident:_, expr } => vec![expr],
            _ => vec![],
        }
    }
}

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

impl fmt::Display for ChildRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.i)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

use super::semantics::TypeError;
impl TypeError {
    pub fn print(self, source: &String) {
        print!("=== Type Error ===\n");
        
        print!("Error at: {}\n", self.start);
        {
            // Get start & end of line slice
            let mut a = self.start;
            for _ in 0..20 {
                if source.as_bytes()[a].is_ascii_control() { 
                    a += 1;
                    break 
                }
                a -= 1;
                if a == 0 { break }
            }
            let mut b = self.start;
            for _ in 0..20 {
                if b == source.len() || source.as_bytes()[b].is_ascii_control() { 
                    break
                }
                b += 1;
            }

            print!("    ");
            for c in source[a..b].chars() {
                assert!(!c.is_ascii_control());
                let c = c.escape_debug();
                print!("{}", c);
            }

            // Underline
            print!("\n    {:w$}^", "", w=self.start-a);
            print!("{:-<w$}{}\n", "", self.msg, w=5);

            // Note
            if let Some(expected) = self.expected { print!("  expected type: {}\n", expected) }
            if let Some(found)    = self.found    { print!("  found type: {}\n", found) }
        }
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
            write!(f, "{p}{c}── ")?;
            self.fmt_fields_only(f)?;
            write!(f, "\n")?;
        }
        
        // Update prefix
        let mut np = prefix.clone();
        np.push_str("   │");

        // For each children
        let children = self.get_children();
        let len = children.len();
        for (i, child) in children.into_iter().enumerate()
        {
            // If is last children, no need to inlcude line for next child in prefix
            if i == len-1 {
                np = prefix.clone();
                np.push_str("    ");
            }

            // If child is node, recurse. Else, print.
            match &**child {
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

    fn fmt_fields_only (&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.data {
            NodeData::Declaration { ident, expr:_ } => {
                write!(f, "Declaration: {{ ")?;
                write!(f, "ident: {:?}, ", ident)?;
            },
            NodeData::Expr { t1:_, t2:_, op } => {
                write!(f, "Expr: {{ ")?;
                write!(f, "op: {:?}, ", op)?;
            },
            _ => write!(f, "Unimplemented")?
        }
        write!(f, " }}")?;
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
