mod cilantro; 

use std::env;
use std::fs::File;
use std::io::prelude::*;


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
    cilantro::from_source(source);
}
