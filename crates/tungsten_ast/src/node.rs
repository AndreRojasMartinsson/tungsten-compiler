use std::ops::Range;

use tungsten_utils::Atom;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Node {
    pub span: Range<usize>,
}

impl Node {
    pub fn new(span: Range<usize>) -> Self {
        Self { span }
    }
}
