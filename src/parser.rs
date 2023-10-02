mod table;

use std::fmt::Write;

use super::*;
pub use table::ParseTable;

pub struct SyntaxError {
    msg: String,
}

impl CilantroErrorTrait for SyntaxError {
    fn fmt (&self, _source: &String) -> Result<String, std::fmt::Error> {
        Ok(self.msg.clone())
    }
}

/// Parser object. Contains parser table, productions, and source.
pub struct Parser<'a> {
    table: ParseTable,
    productions: Productions,
    tokens: Vec<Token>,
    source: &'a str 
}

impl<'a> Parser<'a> {
    /// Creates a new parser instance. 
    /// Generates Parser table from production list. 
    /// Edit production list in `/src/cilantro/grammar.rs`
    pub fn new (tokens: Tokens, source: &'a str) -> Self {
        assert!(tokens.len() > 0); 
        let productions = Productions::make();
        let table = productions.make_table();

        Self {
            table,
            productions,
            tokens,
            source
        }
    }

    #[cfg(test)]
    pub fn new_test (tokens: Tokens, source: &'a str) -> Self {
        let productions = Productions::make_test();
        let table = productions.make_table();

        Self {
            table,
            productions,
            tokens,
            source
        }
    }

    /// Parses the passed soruce.
    pub fn parse (mut self) -> Result<Vec<Node>, SyntaxError> {
        let mut l: Vec<(Elem, usize)> = vec![];
        let mut r: Vec<_> = self.tokens.into_iter().map(|t| Elem::Token(t)).rev().collect();

        println!("parsing...");
        loop {
            //print_stacks(&l, &r);

            // Exit condition: If only token left is EOF and the last node is a root
            if r.len() == 1 {
                if let Elem::Node(node) = &l.last().unwrap().0 {
                    if self.productions.roots.contains(&node.t) {
                        println!("quiting with last node as root: {}", node);
                        break;
                    }
                }
            }

            let t = r.last().unwrap();
            let s = if let Some((_, s)) = l.last() { *s } else { 0 };

            let action = self.table[s].get(&t.t());

            // Unfilled cell in table should mean syntax error
            if action.is_none() {
                // To satisfy borrow checker, since 'tokens' was moved.
                self.tokens = vec![];
                return Err(
                        self.syntax_error(&l, &r)
                            .expect("syntax error formatting panicked.") 
                    );
            }
            let action = action.unwrap();
            
            match action {
                Action::Shift(ns) => {
                    //println!("shifting to {}", ns);
                    l.push((r.pop().unwrap(), *ns))
                }
                Action::Reduce(p) => {
                    //println!("reducing with {}", p);
                    let p = &self.productions.v[*p];
                    let elems = l.split_off(l.len()-p.v.len())
                        .into_iter()
                        .map(|x| x.0)
                        .collect();
                    let n = Node::make(p.node, elems);
                    r.push(Elem::Node(n));
                }
            }
        }
        
        let out = l.into_iter()
            .map(|elem| 
                if let Elem::Node(node) = elem.0 { node }
                // There should not be a token in the resulting stream.
                else { panic!("Parser did not remove all tokens. Check definition of 'root' nodes.") }
            )
            .collect();

        Ok(out)
    }

    /// Creates a SyntaxError Object to propogate.
    fn syntax_error (&self, l: &Vec<(Elem, usize)>, r: &Vec<Elem>) -> Result<SyntaxError, std::fmt::Error> {
        let mut f = String::new();

        write!(f, "==Syntax Error==\n")?;
        // write!(f, "Parser stack dump:\n")?;
        // write!(f, "{}", print_stacks(l, r)?)?;

        let (l, s) = l.last().unwrap();
        let r = r.last().unwrap();

        write!(f, "Error At: {}\n", l.start())?;
        {
            // Get Segment
            let mut a = l.start();
            for _ in 0..20 {
                if self.source.as_bytes()[a].is_ascii_control() { 
                    a += 1;
                    break 
                }
                a -= 1;
                if a == 0 { break }
            }
            let mut b = r.end();
            for _ in 0..20 {
                if b == self.source.len() || self.source.as_bytes()[b].is_ascii_control() { 
                    break
                }
                b += 1;
            }
            // Segment
            write!(f, "    ")?;
            for mut c in self.source[a..b].chars() {
                if c == '\n' { c = ' '; }
                let c = c.escape_debug();
                write!(f, "{}", c)?;
            } 
            write!(f, "...\n")?;
            // Underline
            write!(f, "    {:w$}", "", w=l.start()-a)?;
            write!(f, "{:-<wa$}^{:-<wb$}\n", "", "", wa=r.start()-l.start(), wb=r.end()-r.start())?;

            // Note
            write!(f, "unexpected token: {}\n", r.t())?;
            let expected: Vec<_> = self.table[*s].iter().map(|(x, _)| x).collect();
            write!(f, "expected: {:?}\n", expected)?;
        }

        Ok(SyntaxError { msg: f })
    }
}


/// Formats parser stack
fn print_stacks (l: &Vec<(Elem, usize)>, r: &Vec<Elem>) -> Result<String, std::fmt::Error> {
    const W: usize = 10;
    let mut f = String::new();

    for (_, s) in l {
        write!(f, "{:<width$.width$} ", s, width=W)?;
    }
    write!(f, "\n")?;

    for (e, _) in l {
        write!(f, "{:<width$.width$} ", format!("{e}"), width=W)?;
    }
    write!(f, " | ")?;
    for e in r.iter().rev() {
        write!(f, "{:<width$.width$} ", format!("{e}"), width=W)?;
    }
    write!(f, "\n")?;

    Ok(f)
}

