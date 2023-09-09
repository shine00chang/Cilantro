mod table;

use super::*;
pub use table::ParseTable;



/// Parser object. Contains parser table, productions, and source.
pub struct Parser {
    table: ParseTable,
    productions: Productions,
    tokens: Vec<Token>,
    source: String
}

impl Parser {
    /// Creates a new parser instance. 
    /// Generates Parser table from production list. 
    /// Edit production list in `/src/cilantro/grammar.rs`
    pub fn new (tokens: Tokens, source: String) -> Self {
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
    pub fn new_test (tokens: Tokens, source: String) -> Self {
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

        loop {
            //print_stacks(&l, &r);

            // Exit condition: If only token left is EOF and the last node it a root
            if r.len() == 1 {
                if let Elem::Node(node) = &l.last().unwrap().0 {
                    if self.productions.roots.contains(&node.t) {
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
                else { panic!("unreachable") }
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
            let a = 0.max(l.start() as i32 -10) as usize;
            let b = self.source.len().min(r.end()+10);
            //let mut extra = 0;
            print!("    ");
            for c in self.source[a..b].chars() {
                let c = c.escape_debug().to_string();
                //extra += c.len()-1;
                print!("{}", c);
            }
            print!("\n    {:w$}", "", w=l.start()-a);
            println!("{:-<wa$}^{:-<wb$}", "", "", wa=r.start()-l.start(), wb=r.end()-r.start());
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

        let parser = Parser::new_test(toks, s);
        println!("starting parse");
        let nodes = parser.parse();
        
        // TODO:
        for node in nodes {
            println!("{}", node);
        }
    }
}
