use bitflags::bitflags;
use tungsten_symbols::SymbolTable;
use tungsten_utils::Atom;

use crate::{values::DataType, BlockItem, Body, Identifier, Node};

bitflags! {
    #[derive(Debug)]
    pub struct ConstantFlags: u8 {
        const PUBLIC = 1;
        const GLOBAL = 1 << 1;
    }

    #[derive(Debug)]
    pub struct VariableFlags: u8 {
        const PUBLIC = 1;
        const CONSTANT = 1 << 1;
        const GLOBAL = 1 << 2;
        const MUTABLE = 1 << 3;
    }

    #[derive(Debug)]
    pub struct FunctionFlags: u8 {
        const PUBLIC = 1;
        const CONSTANT = 1 << 1;
        const GLOBAL = 1 << 2;
    }
}

#[derive(Debug)]
pub struct SourceFile {
    pub node: Node,
    pub body: Body,
}

#[derive(Debug)]
pub struct Parameter {
    pub node: Node,
    pub r#type: DataType,
    pub name: Identifier,
}

#[derive(Debug)]
pub struct Block {
    pub node: Node,
    pub symbol: SymbolTable,
    pub items: Vec<BlockItem>,
}
