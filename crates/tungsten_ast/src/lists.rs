use crate::Node;

#[derive(Debug)]
pub struct CommaSeperatedList<T> {
    pub node: Node,
    pub items: Vec<T>,
}
