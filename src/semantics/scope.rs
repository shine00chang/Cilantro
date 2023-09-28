use std::collections::{LinkedList, HashSet};

use super::*;

pub struct ScopeError {

}

#[derive(Debug)]
struct SymbolStack {
    stack: LinkedList<HashSet<String>>,
    scope: usize
}
impl SymbolStack {
    fn new () -> Self {
        let mut stack = LinkedList::new();
        stack.push_back(HashSet::new());

        Self {
            stack,
            scope: 0,
        }
    }

    fn get_scope (&self, ident: &String) -> Option<usize> {
        println!("trying to find {} in:\n{:?}", ident, self.stack);
        let mut level = self.scope;
        for set in self.stack.iter() {
            if set.contains(ident) {
                return Some(level)
            }
            if level != 0 { 
                level -= 1;
            }
        }
        None
    }

    /// Declare an identifier. Adds identifier to current scope.
    /// Returns `Err()` if the identifier is found in the scope (redeclaration).
    fn declare (&mut self, ident: String) -> Result<usize, ()> {
        println!("defining '{ident}' in:\n{:?}", self.stack);
        if self.stack
            .front_mut()
            .expect("Should always have global scope")
            .insert(ident.clone()) {
            Ok(self.scope)
        } else {
            Err(())
        }
    }

    fn new_scope (&mut self) {
        self.stack.push_front(HashSet::new());
        self.scope += 1;
    }

    fn end_scope (&mut self) {
        if self.stack.len() == 1 || self.scope == 0 {
            panic!("tried removing global scope");
        }
        self.stack.pop_front();
        self.scope -= 1;
    }
}

pub fn resolve_scope (nodes: &mut Vec<Node>) -> Result<(), ScopeError> {
    let mut stack = SymbolStack::new();
    for node in nodes {
        node.resolve_scope(&mut stack)?;
    }
    Ok(())
}

impl Node {
    /// Uses a scope table (Set Stack) to annotate scope onto each identifier.
    /// Algorithm:
    /// - Recursive traversal
    /// - If self is Node::Declare => Add identifier to scope
    /// - If self is Node::Block   => Add new set to stack
    /// - If child is Token::IDENT => Check if identifier exists & annotate scope level
    fn resolve_scope (&mut self, stack: &mut SymbolStack) -> Result<(), ScopeError> {
        
        // If self is declaration, add to stack.
        if self.t == NodeT::Declaration {
            let ident = {
                if let Elem::Token(tok) = &mut self.children[0] {
                    if let TokenData::IDENT(ident) = &mut tok.data { ident }
                    else { panic!() }
                } else { panic!() }
            };
            let scope_level = stack.declare(ident.clone())
                .map_err(|_| {
                    ScopeError{} 
                })?;
            ident.push('@');
            ident.push_str(&scope_level.to_string());

            return Ok(());
        } 

        // If self is function, define symbol. Don't add scope annotation
        if self.t == NodeT::Function {
            let ident = {
                if let Elem::Token(tok) = &mut self.children[0] {
                    if let TokenData::IDENT(ident) = &mut tok.data { ident }
                    else { panic!() }
                } else { panic!() }
            };
            let scope_level = stack.declare(ident.clone())
                .map_err(|_| { ScopeError{} })?;
            
            if scope_level > 0 {
                panic!("Attempted to define function at non-global scope");
            }
            return Ok(());
        }

        // If self is block, start scope
        if self.t == NodeT::Block {
            stack.new_scope();
        }

        for (i, child) in self.children.iter_mut().enumerate() {
            match child {
                Elem::Node(child)  => child.resolve_scope(stack)?,
                Elem::Token(child) => {
                    match &mut child.data 
                    {
                        TokenData::IDENT(ident) => 
                        {
                            // NOTE: Exception: Function 
                            // TODO: Add lib functions to symbolstack so we can check function
                            // symbols
                            if self.t == NodeT::Invoke && i == 0 { continue };

                            // Add scope annotation to end of identifier
                            if let Some(scope_level) = stack.get_scope(&ident) {
                                ident.push('@');
                                ident.push_str(&scope_level.to_string())
                            } else {
                                return Err(ScopeError{})
                            }
                        }
                        _ => ()
                    }
                }
            }
        }

        // If self is block, end started scope
        if self.t == NodeT::Block {
            stack.end_scope();
        }

        Ok(())
    }
}
