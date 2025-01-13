use tungsten_lexer::Value;

use crate::{
    AssignmentOperator, BinaryOperator, Identifier, Node, Operator, PostfixOperator, UnaryOperator,
};

#[derive(Debug)]
pub enum Expression {
    Unary(Box<UnaryExpression>),
    Binary(Box<BinaryExpression>),
    Postfix(Box<PostfixExpression>),
    Assignment(Box<AssignmentExpression>),
    Paren(Box<ParenthesisExpression>),
    Identifier(Box<Identifier>),
    Literal(Box<Literal>),
}

#[derive(Debug)]
pub struct ParenthesisExpression {
    pub node: Node,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub node: Node,
    pub operator: Operator<UnaryOperator>,
    pub operand: Expression,
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub node: Node,
    pub operator: Operator<BinaryOperator>,
    pub left: Expression,
    pub right: Expression,
}

#[derive(Debug)]
pub struct PostfixExpression {
    pub node: Node,
    pub operator: Operator<PostfixOperator>,
    pub operand: Expression,
}

#[derive(Debug)]
pub struct AssignmentExpression {
    pub node: Node,
    pub target: AssignTarget,
    pub operator: Operator<AssignmentOperator>,
    pub initializer: Expression,
}

#[derive(Debug)]
pub enum AssignTarget {}

#[derive(Debug)]
pub struct Literal {
    pub node: Node,
    pub value: Value,
}
