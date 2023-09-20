mod trim;
mod scope;
mod extract;
mod type_check;

use super::*;

// TypeTable exposed for annotation parser
// TypeError exposed for visualizer
pub use type_check::{TypeTable, TypeError};

pub fn to_ast (source: &String, nodes: Vec<Node>) -> Vec<LNode> {
    
    // Trim unecessary grammar elements.
    let mut nodes: Vec<_> = nodes.into_iter().map(|n| {
        if let Elem::Node(n) = n.trim() {
            n
        } else {
            panic!("root should not collapse to token");
        }
    }).collect();
    
    println!("Trimmed Tree:\n");
    nodes.iter().for_each(|n| println!("{n}"));

    // Identifier Scope Resolution 
    nodes.iter_mut().for_each(|n| n.resolve_scope());

    // Extract children values & Map to Node data.
    let nodes: Vec<_> = nodes.into_iter().map(|n| n.extract()).collect();

    println!("Extracted Tree:\n");
    nodes.iter().for_each(|n| println!("{n}"));


    // Type checking
    let nodes = match type_check::type_check(nodes) {
        Err(err) => {
            let out = String::new();
            err.print(source);
            println!("{out}");
            panic!();
        }, 
        Ok(v) => v
    };

    nodes
}

