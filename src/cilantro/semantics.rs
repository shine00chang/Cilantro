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
    nodes.iter_mut().for_each(|n| println!("{n}"));

    // Identifier Scope Resolution 
    nodes.iter_mut().for_each(|n| n.resolve_scope());

    // Extract children values & Map to Node data.
    let mut nodes: Vec<_> = nodes.into_iter().map(|n| n.extract()).collect();

    // Type checking
    if let Err(err) = type_check::type_check(&mut nodes) {
        let out = String::new();
        err.print(source);
        println!("{out}");
        panic!();
    }

    nodes
}

