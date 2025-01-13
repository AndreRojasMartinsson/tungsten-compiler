use std::mem::uninitialized;

use tungsten_ast::{
    BinaryExpression, BinaryOperator, Expression, Literal, Node, Operator, OperatorPrecedence,
    ParenthesisExpression, UnaryExpression, UnaryOperator,
};
use tungsten_lexer::{Kind, Token, Value};

use super::Parser;

impl Parser<'_> {
    pub(crate) fn parse_expression(
        &mut self,
        lhs: Option<Expression>,
        min_precedence: u8,
    ) -> Expression {
        let mut lhs = if let Some(lhs) = lhs {
            lhs
        } else {
            self.parse_primary()
        };

        while let Some(op) = BinaryOperator::from_kind(self.current_kind().clone()) {
            let precedence = OperatorPrecedence::from_binary(op).u8();
            if precedence < min_precedence {
                break;
            }

            self.advance();
            let mut rhs = self.parse_primary();

            loop {
                match BinaryOperator::from_kind(self.current_kind().clone()) {
                    Some(next_op) => {
                        let next_precedence = OperatorPrecedence::from_binary(next_op);
                        let next_min_precedence = if next_precedence.is_right_associative() {
                            precedence
                        } else {
                            precedence + 1
                        };

                        if next_precedence.u8() < next_min_precedence {
                            break;
                        }

                        rhs = self.parse_expression(Some(rhs), next_min_precedence);
                        continue;
                    }
                    _ => break,
                }
            }

            lhs = Expression::Binary(Box::new(BinaryExpression {
                node: Node::default(),
                left: lhs,
                right: rhs,
                operator: Operator {
                    operator: op,
                    node: Node::default(),
                },
            }))
        }

        lhs
    }

    pub(crate) fn parse_unary(&mut self) -> UnaryExpression {
        let node = self.start_node();
        let op_token = self.advance();

        if let Some(operator) = UnaryOperator::from_kind(op_token.kind) {
            return UnaryExpression {
                node: self.finish_node(node),
                operator: Operator {
                    node: Node::new(op_token.span),
                    operator,
                },
                operand: self.parse_expression(None, OperatorPrecedence::Unary.u8() + 1),
            };
        }

        panic!("Ah")
    }

    pub(crate) fn parse_primary(&mut self) -> Expression {
        match self.current_kind() {
            Kind::LParen => {
                let node = self.start_node();

                self.bump(Kind::LParen);
                let expr = self.parse_expression(None, 0);
                self.bump(Kind::RParen);

                Expression::Paren(Box::new(ParenthesisExpression {
                    node: self.finish_node(node),
                    expression: expr,
                }))
            }
            // Unary
            Kind::Dash
            | Kind::Plus
            | Kind::Hash
            | Kind::DoublePlus
            | Kind::DoubleDash
            | Kind::Ampersand
            | Kind::Asterisk => Expression::Unary(Box::new(self.parse_unary())),
            Kind::FloatLiteral
            | Kind::StringLiteral
            | Kind::BooleanLiteral
            | Kind::IntegerLiteral => Expression::Literal(Box::new(self.parse_literal())),
            Kind::Identifier => Expression::Identifier(Box::new(self.parse_identifier())),
            _ => unimplemented!(),
        }
    }

    // pub(crate) fn parse_literal(&mut self) -> Literal {
    //     let token = self.current_token();
    //
    //     Literal {
    //         node: Node::new(token.span.clone()),
    //         value: token.value.clone().unwrap(),
    //     }
    // }
}
