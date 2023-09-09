mod trim;
mod scope;
mod extract;
mod type_check;

use super::*;

pub struct TypeError {

}

pub fn to_ast (nodes: Vec<Node>) -> Result<Vec<LNode>, TypeError> {
    
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
    let nodes: Vec<_> = nodes.into_iter().map(|n| n.extract()).collect();

    // Type checking
    for node in &nodes {
        node.type_check()?;
    }

    Ok(nodes)
}

