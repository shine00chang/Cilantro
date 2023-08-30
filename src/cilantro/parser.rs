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
        vec![]
    }
}
