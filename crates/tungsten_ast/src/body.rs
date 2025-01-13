use tungsten_symbols::SymbolTable;

use crate::{ConstantItem, FunctionItem, Node, VariableItem};

#[derive(Debug)]
pub struct Body {
    pub node: Node,
    pub items: Vec<BodyItem>,
    pub symbol: SymbolTable,
}

#[derive(Debug)]
pub enum BodyItem {
    Func(Box<FunctionItem>),
    Constant(Box<ConstantItem>),
    Variable(Box<VariableItem>),
}
