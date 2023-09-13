*under construction*

# Cilantro
A transpiler for a high-level scripting language to WASM Text (`.wat`) in WASI. Written in Rust 🦀. <br>
Lexer built with parser-combinator library `nom`. <br>
Parser implements an SLR(1) parser. <br>
Integrated WASM runtime with `wasmtime`. <br>
<br>
Build it locally: `cargo run examples/arith.txt` <br>
(**not yet made**) Or try it out on the web [here](www.soon.tm). <br>

## Examples
- `cargo run examples/arith.txt`: Arithmetics showcase. Implements proper order of operations.
- `cargo run examples/funcs.txt`: Functions showcase. Supports return types and variable length arguments.
- `cargo run examples/syntax-error.txt`: Syntax error logging showcase. Demonstrates traceability.
- `cargo run examples/type-error.txt`: Type error logging showcase. Demonstrates type inferencing & traceability.


A continuation of Parsley. <br>
