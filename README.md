*under construction*

# Cilantro
A transpiler for a high-level scripting language to WASM Text (`.wat`) in WASI. Written in Rust 🦀. <br>
Lexer built with parser-combinator library `nom`. <br>
Parser implements an SLR(1) parser. <br>
Multi-pass recursive traversal for semantic analysis (Concrete Syntax Tree trimming, type checking, symbol scope resolution).
Integrated WASM runtime through `wasmtime`. <br>
<br>

**Quick Demo: Recursive Fibonacci**
```
func fib (n: i64) -> i64 {
    if n == 1 {
        return 1
    }
    if n == 2 {
        return 1
    }
    return fib(n-1) + fib(n-2)
}

let num = 13 
print("fibonacci no.")
print64(num)
print(" = ")
print64(fib(num))
println("")
```
<br>

Build it locally: `cargo run examples/conditionals.txt` <br>
(**not yet made**) Or try it out on the web [here](www.soon.tm). <br>

## Demos 
**Type Error Logging & Tracing:**<br>
![type-error](https://github.com/shine00chang/cilantro/blob/main/demos/type-error.png)

**Syntax Error Logging & Tracing:**<br>
![syntax-error](https://github.com/shine00chang/cilantro/blob/main/demos/syntax-error.png)

**CST:**<br>
![cst](https://github.com/shine00chang/cilantro/blob/main/demos/cst.png)

**AST:**<br>
![ast](https://github.com/shine00chang/cilantro/blob/main/demos/ast.png)

**Tokenization:**<br>
![tokens](https://github.com/shine00chang/cilantro/blob/main/demos/tokenization.png)

## Examples
- `cargo run examples/fibonacci.txt`: Recursion showcase.
- `cargo run examples/arith.txt`: Arithmetics showcase. Implements proper order of operations (*no negation operator yet*)
- `cargo run examples/conditionals.txt.txt`: Boolean logc & Conditional forks showcase. Supports if statements, boolean algebra, and equality checks (*no string equality yet*)
- `cargo run examples/funcs.txt`: Functions showcase. Supports function and parameter typing.
- `cargo run examples/strings.txt`: String literals showcase. Supports only string literals.
- `cargo run examples/syntax-error.txt`: Syntax error logging showcase. Demonstrates traceability.
- `cargo run examples/type-error.txt`: Type error logging showcase. Demonstrates type inferencing & traceability.

<br>
<br>
A continuation of Parsley.
