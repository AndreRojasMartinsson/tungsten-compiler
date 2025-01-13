use crate::{
    Block, CommaSeperatedList, ConstantFlags, DataType, Expression, FunctionFlags, Node, Parameter,
    VariableFlags, VariableValue,
};
use tungsten_utils::Atom;

#[derive(Debug)]
pub struct ConstantItem {
    pub node: Node,
    pub value: VariableValue,
    pub flags: ConstantFlags,
}

#[derive(Debug)]
pub struct FunctionItem {
    pub node: Node,
    pub return_type: DataType,
    pub name: Identifier,
    pub flags: FunctionFlags,
    pub parameters: CommaSeperatedList<Parameter>,
    pub block: Block,
}

#[derive(Debug)]
pub struct VariableItem {
    pub node: Node,
    pub r#type: DataType,
    pub name: Identifier,
    pub flags: VariableFlags,
    pub initializer: Option<Expression>,
}

#[derive(Debug)]
pub struct ReturnItem {
    pub node: Node,
    pub initializer: Option<Expression>,
}

#[derive(Debug)]
pub enum BlockItem {
    Constant(Box<ConstantItem>),
    Variable(Box<VariableItem>),
    Expression(Box<Expression>),
    Return(Box<ReturnItem>),
}

#[derive(Debug)]
pub struct Identifier {
    pub node: Node,
    pub image: Atom,
}
