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

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;


#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn tobytes (wat_str: String) -> js_sys::Uint8Array {
    let bytes = wat::parse_str(wat_str).expect("could not convert from wat to wasm");

    js_sys::Uint8Array::from(&bytes[..])
}


/// Runs Lexer, Parser, Interpreter, and Visualizer
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn compile (source: String) -> String {

    let tokens = lexer::tokenize(source.clone());
    println!("Token Stream:\n{}\n", visualizer::print_tokens(&tokens, &source).unwrap());

    let nodes = Parser::new(tokens, &source).parse();
    println!("Parsed Concrete Sytnax Tree:");
    nodes.iter().for_each(|n| print!("{n}"));

    let nodes = semantics::to_ast(&source, nodes);

    println!("Abstract Syntax Tree:");
    nodes.iter().for_each(|n| print!("{n}"));

    println!("Generating WASI...");
    let code = codegen::gen(nodes);
    /*
    println!("Generated code:");
    println!("{code}");
    */ 

    code
}

pub fn compile_to (source: String, out_path: &str) -> std::io::Result<()> {

    let code = compile(source);

    let mut file = File::create(out_path).expect(format!("Could not create file '{}'", out_path).as_str());
    file.write_all(code.as_bytes()).expect(format!("Could not write to file '{}'", out_path).as_str());

    println!("Generated code written to '{out_path}'");

    Ok(())
}
