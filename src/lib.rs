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

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;


pub trait CilantroErrorTrait {
    fn fmt(&self, source: &String) -> Result<String, std::fmt::Error>;
}

pub type CilantroError = Box<dyn CilantroErrorTrait>;



/// Runs Lexer, Parser, Interpreter, and Visualizer
pub fn compile (source: &String) -> Result<String, CilantroError> {

    let tokens = lexer::tokenize(source)
        .map_err(|e| -> CilantroError { Box::new(e) })?;

    println!("Token Stream:\n{}\n", visualizer::print_tokens(&tokens, &source).unwrap());

    let nodes = Parser::new(tokens, &source)
        .parse()
        .map_err(|e| -> CilantroError { Box::new(e) })?;

    println!("Parsed Concrete Sytnax Tree:");
    nodes.iter().for_each(|n| print!("{n}"));

    let nodes = semantics::to_ast(nodes)?;

    println!("Abstract Syntax Tree:");
    nodes.iter().for_each(|n| print!("{n}"));

    println!("Generating WASI...");
    let code = codegen::gen(nodes);
    /*
    println!("Generated code:");
    println!("{code}");
    */ 

    Ok(code)
}


/// Wraps `compile(..)`, maps Result<String, CilantroError> -> Result<String, String> for ease of
/// wasm porting.
#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn compile_web (source: String) -> Result<String, String> {
    compile(&source)
        .map_err(|err| err.fmt(&source).expect("error formating failed.").clone())
}

/// Transpiles the generated WAT to WASM. Necessary since JS does not natively support this
/// feature.
#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn tobytes (wat_str: String) -> js_sys::Uint8Array {
    let bytes = wat::parse_str(wat_str).expect("could not convert from wat to wasm");

    js_sys::Uint8Array::from(&bytes[..])
}
