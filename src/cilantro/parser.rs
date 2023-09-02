mod table;

use super::*;
pub use table::ParseTable;



/// Parser object. Contains parser table, productions, and source.
pub struct Parser {
    table: ParseTable,
    productions: Productions,
    tokens: Vec<Token>,
}

impl Parser {
    /// Creates a new parser instance. 
    /// Generates Parser table from production list. 
    /// Edit production list in `/src/cilantro/grammar.rs`
    pub fn new (tokens: Tokens) -> Self {
        assert!(tokens.len() > 0); 
        let productions = Productions::make();
        let table = productions.make_table();

        Self {
            table,
            productions,
            tokens 
        }
    }

    #[cfg(test)]
    pub fn new_test (tokens: Tokens) -> Self {
        let productions = Productions::make_test();
        let table = productions.make_table();

        Self {
            table,
            productions,
            tokens 
        }
    }

    /// Parses the passed soruce.
    pub fn parse (self) -> Vec<Node> {
        let mut l: Vec<(Elem, usize)> = vec![];
        let mut r: Vec<_> = self.tokens.into_iter().map(|t| Elem::Token(t)).rev().collect();

        loop {
            print_stacks(&l, &r);

            // Exit condition: If only token left is EOF and the last node it a root
            if r.len() == 1 {
                if let Elem::Node(node) = &l.last().unwrap().0 {
                    if self.productions.roots.contains(&NodeT::from(node.data.clone())) {
                        break;
                    }
                }
            }

            let t = r.last().unwrap();
            let s = if let Some((_, s)) = l.last() { *s } else { 0 };

            let action = self.table[s].get(&t.t());
            if action.is_none() {
                // TODO: Report Syntax Error
                println!("Syntax Error: Parser stack dump:");
                print_stacks(&l, &r);
                panic!("Syntax Error?");
            }
            let action = action.unwrap();
            
            match action {
                Action::Shift(ns) => {
                    println!("shifting to {}", ns);
                    l.push((r.pop().unwrap(), *ns))
                }
                Action::Reduce(p) => {
                    println!("reducing with {}", p);
                    let p = &self.productions.v[*p];
                    let elems = l.split_off(l.len()-p.v.len())
                        .into_iter()
                        .map(|x| x.0)
                        .collect();
                    let n = Node::make(&p.node, elems).unwrap();
                    r.push(Elem::Node(n));
                }
            }
        }
        
        l.into_iter()
            .map(|elem| 
                if let Elem::Node(node) = elem.0 { node }
                else { panic!("unreachable") }
            )
            .collect()
    }
}

fn print_stacks (l: &Vec<(Elem, usize)>, r: &Vec<Elem>) {
    const W: usize = 10;

    for (_, s) in l {
        print!("{:<width$} ", s, width=W);
    }
    println!();

    for (e, _) in l {
        print!("{:<width$}", format!("{e}"), width=W);
    }
    print!("| ");
    for e in r.iter().rev() {
        print!("{:<width$}", format!("{e}"), width=W);
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

        let parser = Parser::new_test(toks);
        println!("starting parse");
        let nodes = parser.parse();
        
        // TODO:
        for node in nodes {
            println!("{}", node);
        }
    }
}
