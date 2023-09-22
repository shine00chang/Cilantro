mod table;

use super::*;
pub use table::ParseTable;



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
    pub fn parse (mut self) -> Vec<Node> {
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
                self.syntax_error(&l, &r);
                panic!("syntax error");
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
        out
    }

    fn syntax_error (&self, l: &Vec<(Elem, usize)>, r: &Vec<Elem>) {

        println!("==Syntax Error==");
        println!("Parser stack dump:");
        print_stacks(l, r);

        let (l, s) = l.last().unwrap();
        let r = r.last().unwrap();

        println!("Error At: {}", l.start());
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
            println!("{}", b);
            println!("{}", &self.source[0..=b]);
            for _ in 0..20 {
                if b == self.source.len() || self.source.as_bytes()[b].is_ascii_control() { 
                    break
                }
                b += 1;
            }
            // Segment
            print!("    ");
            for mut c in self.source[a..b].chars() {
                if c == '\n' { c = ' '; }
                let c = c.escape_debug();
                print!("{}", c);
            } 
            print!("...\n");
            // Underline
            print!("    {:w$}", "", w=l.start()-a);
            print!("{:-<wa$}^{:-<wb$}\n", "", "", wa=r.start()-l.start(), wb=r.end()-r.start());

            // Note
            println!("unexpected token: {}", r.t());
            let expected: Vec<_> = self.table[*s].iter().map(|(x, _)| x).collect();
            println!("expected: {:?}", expected);
        }
    }
}


fn print_stacks (l: &Vec<(Elem, usize)>, r: &Vec<Elem>) {
    const W: usize = 10;

    for (_, s) in l {
        print!("{:<width$.width$} ", s, width=W);
    }
    println!();

    for (e, _) in l {
        print!("{:<width$.width$} ", format!("{e}"), width=W);
    }
    print!(" | ");
    for e in r.iter().rev() {
        print!("{:<width$.width$} ", format!("{e}"), width=W);
    }
    println!();
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parser_test () {
        let s = "baab".to_owned();
        let toks = tokenize(s.clone());

        let parser = Parser::new_test(toks, &s);
        println!("starting parse");
        let nodes = parser.parse();
        
        // TODO:
        for node in nodes {
            println!("{}", node);
        }
    }
}
