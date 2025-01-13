use tungsten_lexer::Kind;

use crate::Node;

#[derive(Debug)]
pub struct Operator<T> {
    pub node: Node,
    pub operator: T,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OperatorPrecedence {
    Assignment = 0,
    LogicalOr = 1,
    LogicalAnd = 2,
    BitwiseOr = 3,
    BitwiseXor = 4,
    BitwiseAnd = 5,
    Equality = 6,
    Relational = 7,
    BitShift = 8,
    Concatenation = 9,
    Additive = 10,
    Multiplicative = 11,
    Exponential = 12,
    Unary = 13,
    Postfix = 14,
    Scope = 15,
}

impl OperatorPrecedence {
    pub fn from_binary(op: BinaryOperator) -> Self {
        match op {
            BinaryOperator::LogicalOr => Self::LogicalOr,
            BinaryOperator::LogicalAnd => Self::LogicalAnd,
            BinaryOperator::BitOr => Self::BitwiseOr,
            BinaryOperator::BitXor => Self::BitwiseXor,
            BinaryOperator::BitAnd => Self::BitwiseAnd,
            BinaryOperator::Equal | BinaryOperator::NotEqual => Self::Equality,
            BinaryOperator::Less
            | BinaryOperator::Greater
            | BinaryOperator::LessEq
            | BinaryOperator::GreaterEq => Self::Relational,
            BinaryOperator::LeftShift | BinaryOperator::RightShift => Self::BitShift,
            BinaryOperator::Concat => Self::Concatenation,
            BinaryOperator::Add | BinaryOperator::Subtract => Self::Additive,
            BinaryOperator::Multiply
            | BinaryOperator::Modulus
            | BinaryOperator::Divide
            | BinaryOperator::FloorDivide => Self::Multiplicative,
            BinaryOperator::Power => Self::Exponential,
        }
    }

    pub fn is_right_associative(&self) -> bool {
        matches!(self, Self::Unary | Self::Assignment | Self::Exponential)
    }

    pub fn u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    /// -
    Negate,
    /// +
    Positive,
    /// !
    LogicalNot,
    /// ~
    BitNot,
    /// #
    Hash,
    /// &
    AddressOf,
    /// *
    Indirection,
    /// ++$expr
    Increment, // Prefix
    /// --$expr
    Decrement, // Prefix
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    /// %
    Modulus,
    /// **
    Power,
    /// <<
    LeftShift,
    /// >>
    RightShift,
    /// //
    FloorDivide,
    /// &
    BitAnd,
    /// |
    BitOr,
    /// ^
    BitXor,
    Equal,
    NotEqual,
    Less,
    Greater,
    GreaterEq,
    LessEq,
    /// &&
    LogicalAnd,
    /// ||
    LogicalOr,
    /// <>
    Concat,
}

#[derive(Debug)]
pub enum AssignmentOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Power,
    LeftShift,
    RightShift,
    FloorDivide,
    BitAnd,
    BitOr,
    BitXor,
    // MAYBE? Concat Assign
}

#[derive(Debug)]
pub enum PostfixOperator {
    Increment,
    Decrement,
}

impl UnaryOperator {
    pub fn from_kind(kind: Kind) -> Option<Self> {
        match kind {
            Kind::Plus => Some(Self::Positive),
            Kind::Dash => Some(Self::Negate),
            Kind::DoubleDash => Some(Self::Decrement),
            Kind::DoublePlus => Some(Self::Increment),
            Kind::Bang => Some(Self::LogicalNot),
            Kind::Tilde => Some(Self::BitNot),
            Kind::Ampersand => Some(Self::AddressOf),
            Kind::Hash => Some(Self::Hash),
            Kind::Asterisk => Some(Self::Indirection),
            _ => None,
        }
    }
}

impl BinaryOperator {
    pub fn from_kind(kind: Kind) -> Option<Self> {
        match kind {
            Kind::Plus => Some(Self::Add),
            Kind::Dash => Some(Self::Subtract),
            Kind::DoubleAsterisk => Some(Self::Power),
            Kind::DoubleSlash => Some(Self::FloorDivide),
            Kind::Slash => Some(Self::Divide),
            Kind::Percent => Some(Self::Modulus),
            Kind::Asterisk => Some(Self::Multiply),
            Kind::LessGreater => Some(Self::Concat),
            Kind::DoubleGreater => Some(Self::RightShift),
            Kind::DoubleLess => Some(Self::LeftShift),
            Kind::GreaterEq => Some(Self::GreaterEq),
            Kind::Greater => Some(Self::Greater),
            Kind::LessEq => Some(Self::LessEq),
            Kind::Less => Some(Self::Less),
            Kind::Ampersand => Some(Self::BitAnd),
            Kind::Pipe => Some(Self::BitOr),
            Kind::Caret => Some(Self::BitXor),
            Kind::Equal => Some(Self::Equal),
            Kind::BangEqual => Some(Self::NotEqual),
            Kind::DoubleAmpersand => Some(Self::LogicalAnd),
            Kind::DoublePipe => Some(Self::LogicalOr),
            _ => None,
        }
    }
}

impl AssignmentOperator {
    pub fn from_kind(kind: Kind) -> Option<Self> {
        match kind {
            Kind::PlusAssign => Some(Self::Add),
            Kind::DashAssign => Some(Self::Subtract),
            Kind::DoubleAsteriskAssign => Some(Self::Power),
            Kind::DoubleSlashAssign => Some(Self::FloorDivide),
            Kind::SlashAssign => Some(Self::Divide),
            Kind::PercentAssign => Some(Self::Modulus),
            Kind::AsteriskAssign => Some(Self::Multiply),
            Kind::DoubleGreaterAssign => Some(Self::RightShift),
            Kind::DoubleLessAssign => Some(Self::LeftShift),
            Kind::AmpersandAssign => Some(Self::BitAnd),
            Kind::PipeAssign => Some(Self::BitOr),
            Kind::CaretAssign => Some(Self::BitXor),
            _ => None,
        }
    }
}

impl PostfixOperator {
    pub fn from_kind(kind: Kind) -> Option<Self> {
        match kind {
            Kind::DoublePlus => Some(Self::Increment),
            Kind::DoubleDash => Some(Self::Decrement),
            _ => None,
        }
    }
}
