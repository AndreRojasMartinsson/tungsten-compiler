use std::collections::HashMap;

use tungsten_utils::Atom;

use crate::Literal;

#[derive(Debug)]
pub enum VariableValue {
    Integer(u64),
    Float(f64),
    Boolean(bool),
    String(Atom),
}

#[derive(Debug)]
pub enum DataType {
    Generic, // TODO fix it so it is good.
    Object(HashMap<String, DataType>),
    Tuple(Vec<DataType>),
    Array(Box<(DataType, Option<Literal>)>),
    Simple(SimpleType),
}

impl DataType {
    pub fn to_string(&self) -> String {
        match self {
            DataType::Generic => "<generic>".to_string(),
            DataType::Simple(simple) => simple.to_string(),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub enum SimpleType {
    UnsignedInteger,
    SignedInteger,
    Float,
    Boolean,
    String,
    Identifier(Atom),
}

impl SimpleType {
    pub fn to_string(&self) -> String {
        match self {
            Self::String => "<string>".to_string(),
            Self::UnsignedInteger => "<unsigned_int>".to_string(),
            Self::SignedInteger => "<signed_int>".to_string(),
            Self::Float => "<float>".to_string(),
            Self::Boolean => "<bool>".to_string(),
            Self::Identifier(ident) => ident.to_string(),
        }
    }
}
