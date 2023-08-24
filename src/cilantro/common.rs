use super::*;

pub struct Production {
    pub node: NodeT,
    pub word: Vec<ElemT>
}

pub enum Action {
    Shift(usize),
    Reduce(usize)
}
