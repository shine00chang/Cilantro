mod public;
mod common;
mod lexer;
mod parser;
mod stdlib;
mod semantics;
mod codegen;
mod visualizer;

pub use public::*;
use common::*;
use parser::Parser;
pub use lexer::tokenize;

use std::fs::File;
use std::io::prelude::*;


/// Runs Lexer, Parser, Interpreter, and Visualizer
pub fn from_source (source: String) -> std::io::Result<()> {

    let tokens = lexer::tokenize(source.clone());
    println!("Token Stream:\n{}\n", visualizer::print_tokens(&tokens, &source).unwrap());

    let nodes = Parser::new(tokens, &source).parse();
    println!("Parsed Concrete Sytnax Tree:");
    nodes.iter().for_each(|n| print!("{n}"));

    let nodes = semantics::to_ast(&source, nodes);

    /*
    println!("Abstract Syntax Tree:");
    nodes.iter().for_each(|n| print!("{n}"));
    */

    let code = codegen::gen(nodes);
    println!("Generated code:");
    println!("{code}");

    // Write to 'out/prog.wat'

    let path = "out/prog.wat";
    let mut file = File::create(path).expect(format!("Could not create file '{}'", path).as_str());
    file.write_all(code.as_bytes()).expect(format!("Could not write to file '{}'", path).as_str());

    Ok(())
}
