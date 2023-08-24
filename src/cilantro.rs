mod public;
mod common;
mod grammar;
mod lexer;
mod parser;

use common::*;
pub use public::*;
pub use parser::Parser;


/// Runs Lexer, Parser, Interpreter, and Visualizer
pub fn from_source (source: String) {
    let tokens = lexer::tokenize(source);
    let nodes = Parser::new(tokens).parse();
    
    for node in nodes.iter() {
        print!("{node}");
    }

    //Interpreter::exec(nodes);
}
