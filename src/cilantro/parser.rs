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
        let mut r: Vec<_> = self.tokens.into_iter().map(|t| Elem::Token(t)).collect();

        while !r.is_empty() {
            println!("{:?} | {:?}", l, r);
            let t = r.pop().unwrap();
            let s = l.last().unwrap().1;

            let action = self.table[s].get(&t.t());
            if action.is_none() {
                // Syntax Error
                // TODO: Format Parser Error
                panic!("Parser Error!");
            }
            let action = action.unwrap();
            
            match action {
                Action::Shift(ns) => l.push((t, *ns)),
                Action::Reduce(p) => {
                    let p = &self.productions.v[*p];
                    let elems = l.split_off(l.len()-p.v.len())
                        .into_iter()
                        .map(|x| x.0)
                        .collect();
                    let n = Node::make(p.node.clone(), elems).unwrap();
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
        
        for node in nodes {
            println!("{}", node);
        }
    }
}
