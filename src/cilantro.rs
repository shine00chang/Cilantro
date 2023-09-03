mod public;
mod common;
mod lexer;
mod parser;
mod visualizer;

pub use public::*;
use common::*;
use parser::Parser;
pub use lexer::tokenize;


/// Runs Lexer, Parser, Interpreter, and Visualizer
pub fn from_source (source: String) {

    let tokens = lexer::tokenize(source.clone());
    println!("Token Stream:\n{}\n", visualizer::print_tokens(&tokens, &source).unwrap());

    let nodes = Parser::new(tokens, source).parse();
    
    for node in nodes.iter() {
        print!("{node}");
    }

    //Interpreter::exec(nodes);
}
