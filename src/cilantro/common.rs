use super::*;

pub struct Production {
    node: NodeT,
    word: Vec<ElemT>
}

pub enum Action {
    Shift(usize),
    Reduce(usize)
}
