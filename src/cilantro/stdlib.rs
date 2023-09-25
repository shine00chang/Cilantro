use std::collections::HashSet;

use crate::cilantro::lexer::types;

use super::*;
use super::semantics::TypeTable;

type Span<'a> = nom_locate::LocatedSpan<&'a str>;

pub const RESERVED_MEM: usize = 40;
const PREFIX: &'static str = ";;@signature ";

impl TypeTable {
    /// Creates a TypeTable instance with the types and signatures of symobls from STD lib.
    /// Annotation Format:
    /// - Memory Reservation: ';;@reserve [bytes]'
    /// - Function Siganture: ';;@signature $[identifier] : [return-type] ([param0],[param1],..)
    
    pub fn with_std () -> Self {
        let mut table = Self::default();

        // For each line
        for (linenum, line) in get_lib().lines().enumerate() {

            let suffix = format!(" found on annotation at line {}", linenum);

            // Check for prefix
            let line = line.trim();
            if !line.starts_with(PREFIX) { continue }

            // Extract segments
            let line = &line[PREFIX.len()..];
            let a = if let Some(a) = line.find('$') { a } 
                else { panic!("No identifier symbol ($) {suffix}") };
            let b = if let Some(b) = line.find(':') { b }
                else { panic!("No return type symbol (:) {suffix}") };
            let c = if let Some(c) = line.find('(') { c }
                else { panic!("No param list start symbol ('(') {suffix}") };
            let d = if let Some(d) = line.find(')') { d }
                else { panic!("No param list end symbol (')') {suffix}") };

            // Extract values
            let ident  = line[a+1..b].trim();
            let r_type = if let Ok(t) = to_type(line[b+1..c].trim()) { t } 
                else { panic!("Return type could not be interpreted: '{}'. {suffix}", &line[b+1..c]) };

            let params = line[c+1..d].split(',').map(|param| {
                let r = to_type(param.trim());
                if r.is_err() { 
                    panic!("Param type annotation could not be interpreted: '{}'. {suffix}", param);
                }
                r.unwrap()
            }).collect::<Vec<_>>();

            // Set signature
            if table.funcs.contains_key(&ident.to_owned()) {
                panic!("Overlapping function identifier '{}' {suffix}", ident);
            } 
            table.funcs.insert(ident.to_owned(), (params, r_type));
        }
        
        // DEBUG: Print
        println!("=== stdlib Signatures ===");
        for (k, t) in table.funcs.iter() {
            println!("{:?} : {:?}", k, t);
        }

        table
    }
}

fn to_type (s: &str) -> Result<Type, ()> {
    let span = Span::new(s);
    if let TokenData::TYPE(t) = types(span).map_err(|_| ())?.1.data {
        Ok(t)
    } else {
        panic!("'types' parser should not have returned non-TokenData::TYPE token");
    }
}
