use std::ops::Range;

use tungsten_utils::Atom;

use crate::{kind::Kind, position::Position};

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub span: Range<usize>,
    pub position: Position,
    pub kind: Kind,
    pub lexeme: Atom,
    pub value: Option<Value>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(Atom),
    Integer(u64),
    Float(f64),
    Boolean(bool),
    Primitive(PrimitiveType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrimitiveType {
    String,
    Boolean,
    UnsignedInteger,
    SignedInteger,
    Float,
}
