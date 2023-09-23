mod cilantro; 

use std::env;
use std::fs::File;
use std::io::prelude::*;
use wasmtime::*;
use wasmtime_wasi::*;

fn main () {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Err: No source file path supplied");
        return;
    }

    println!("Reading from file '{}'...", args[1]);
    let source = {
        let mut file = File::open(&args[1]).expect("Could not open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Could not read file");
        contents
    };
    cilantro::from_source(source).expect("Failed to compile.");

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    println!("Building WASI context...");
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args().unwrap()
        .build();
    let mut store = Store::new(&engine, wasi);

    // Instantiate our module with the imports we've created, and run it.
    println!("Compiling module...");
    let module = Module::from_file(&engine, "out/prog.wat").expect("Could not build module");
    linker.module(&mut store, "", &module).expect("Could not link");

    println!("Running...\n=== OUTPUT ===");
    linker
        .get_default(&mut store, "").unwrap()
        .typed::<(), ()>(&store).unwrap()
        .call(&mut store, ()).unwrap();
}
