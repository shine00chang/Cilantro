mod trim;
mod scope;
mod extract;
mod type_check;

use super::*;

// TypeTable exposed for annotation parser
// TypeError exposed for visualizer
pub use type_check::{TypeTable, TypeError};
pub use scope::ScopeError;

pub fn to_ast (nodes: Vec<Node>) -> Result<Vec<LNode>, CilantroError> {
    
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
    scope::resolve_scope(&mut nodes)
        .map_err(|err| -> CilantroError { Box::new(err) })?;

    // Extract children values & Map to Node data.
    let nodes: Vec<_> = nodes.into_iter().map(|n| n.extract()).collect();

    println!("Extracted Tree:\n");
    nodes.iter().for_each(|n| println!("{n}"));


    // Type checking
    let nodes = type_check::type_check(nodes).map_err(|err| -> CilantroError { Box::new(err) })?;

    Ok(nodes)
}

