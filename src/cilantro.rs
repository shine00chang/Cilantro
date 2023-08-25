mod public;
mod common;
mod grammar;
mod lexer;
mod parser;

pub use public::*;
use common::*;
use parser::Parser;
pub use lexer::tokenize;


/// Runs Lexer, Parser, Interpreter, and Visualizer
pub fn from_source (source: String) {
    let tokens = lexer::tokenize(source);
    let nodes = Parser::new(tokens).parse();
    
    for node in nodes.iter() {
        print!("{node}");
    }

    //Interpreter::exec(nodes);
}
