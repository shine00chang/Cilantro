use std::collections::HashMap;
use super::*;
use super::grammar::make_productions;


/// Parser object. Contains parser table, productions, and source.
pub struct Parser {
    table: Vec<HashMap<String, Action>>,
    productions: Vec<Production>,
    tokens: Vec<Token>,
}


impl Parser {
    /// Creates a new parser instance. 
    /// Generates Parser table from production list. 
    /// Edit production list in `/src/cilantro/grammar.rs`
    pub fn new (tokens: Tokens) -> Self {
        let productions = make_productions();

        let table: Vec<HashMap<String, Action>> = vec![];

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
