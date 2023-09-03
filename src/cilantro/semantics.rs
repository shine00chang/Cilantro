mod trim;
mod extract;

use super::*;

pub fn to_ast (nodes: Vec<Node>) -> Vec<LNode> {
    let trimmed: Vec<_> = nodes.into_iter().map(|n| {
        if let Elem::Node(n) = n.trim() {
            n
        } else {
            panic!("root should not collapse to token");
        }
    }).collect();
    println!("Trimmed Tree:\n");
    trimmed.iter().for_each(|n| println!("{n}"));

    trimmed.into_iter().map(|n| n.extract()).collect()
}

