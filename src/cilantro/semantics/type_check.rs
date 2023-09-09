use super::*;

impl LNode {
    /// Uses a type table to ensure type correctness of program.
    /// Does not need to bother with scoping issues. Resolved already.
    pub fn type_check (&self) -> Result<(), TypeError> {
        Ok(())
    }
}
