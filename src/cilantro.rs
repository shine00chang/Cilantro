mod public;
mod common;
mod lexer;
mod parser;
mod semantics;
mod codegen;
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
    println!("Parsed Concrete Sytnax Tree:");
    nodes.iter().for_each(|n| print!("{n}"));

    let nodes = semantics::to_ast(nodes);
    println!("Abstract Syntax Tree:");
    // TODO: Make AST/LNode Printer

    let code = codegen::gen(nodes);
    println!("Generated code:");
    println!("{code}");
}
