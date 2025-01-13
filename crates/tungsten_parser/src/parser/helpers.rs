use std::{clone, ops::Range};

use tungsten_ast::Node;
use tungsten_context::error_builders;
use tungsten_lexer::{Kind, Token, Value};

use crate::errors::ParserError;

use super::Parser;

impl Parser<'_> {
    pub(crate) fn start_node(&self) -> Node {
        let token = self.current_token();
        Node::new(token.span.start..0)
    }

    pub(crate) fn finish_node(&self, node: Node) -> Node {
        Node::new(node.span.start..self.prev_token_end)
    }

    pub(crate) fn current_token(&self) -> &Token {
        self.tokens.first().unwrap()
    }

    pub(crate) fn current_kind(&self) -> &Kind {
        &self.current_token().kind
    }

    /// Checkjs if the current index has token `Kind`
    pub(crate) fn at(&self, kind: Kind) -> bool {
        *self.current_kind() == kind
    }

    /// Advance if we are at `Kind`
    pub(crate) fn bump(&mut self, kind: Kind) -> Token {
        if self.at(kind.clone()) {
            self.advance()
        } else {
            panic!(
                "Unexpected token encountered, expected: {kind:?}, but got: {:?}",
                self.current_kind()
            )
        }
    }

    /// Advance if we are at any of the `Kind`
    pub(crate) fn bump_any(&mut self, kinds: &[Kind]) -> Token {
        if kinds.contains(self.current_kind()) {
            self.advance()
        } else {
            panic!(
                "Unexpected token encountered, expected: {kinds:?}, but got: {:?}",
                self.current_kind()
            )
        }
    }

    pub(crate) fn bump_seq(&mut self, chain: &[Kind]) -> Vec<Token> {
        let mut stream = vec![];

        for kind in chain {
            let token = self.bump(kind.clone());
            stream.push(token);
        }

        stream
    }

    pub(crate) fn is_at_end(&self) -> bool {
        *self.current_kind() == Kind::Eof
    }

    pub(crate) fn bump_sequence_by_delim<F, T>(
        &mut self,
        delimiter: Kind,
        mut consume_method: F,
    ) -> Vec<T>
    where
        F: FnMut() -> T,
    {
        let mut stream: Vec<T> = vec![];

        loop {
            let node = consume_method();

            stream.push(node);

            let (consumed, _) = self.eat(delimiter.clone());
            if !consumed {
                break;
            }
        }

        stream
    }

    pub(crate) fn is_type_identifier(&self, token: Token) -> bool {
        matches!(
            (token.kind, token.value),
            (Kind::PrimitiveType, _) | (Kind::Identifier, Some(_))
        )
    }

    /// Advance and return true + the token if we are at `Kind`, return false otherwise
    pub(crate) fn eat(&mut self, kind: Kind) -> (bool, Option<Token>) {
        if self.at(kind) {
            let token = self.advance();
            return (true, Some(token));
        }

        (false, None)
    }

    /// Advance and return true if we are at `Kind`, return false otherwise
    pub(crate) fn eat_ignore(&mut self, kind: Kind) -> bool {
        if self.at(kind) {
            self.advance();
            return true;
        }

        false
    }

    /// Consume the current token and advance
    pub(crate) fn advance(&mut self) -> Token {
        let token = self.tokens.remove(0);

        self.prev_token_end = self.current_token().span.end;

        token
    }

    pub(crate) fn report_error(&mut self, err: ParserError) {
        match err {
            ParserError::DuplicateFunction {
                name,
                orig_span,
                func_span,
            } => {
                self.context
                    .add_error(error_builders::build_duplicate_function_error(
                        name, func_span, orig_span,
                    ));
            } // LexerError::UnexpectedEnd(ctx) => {
              //     self.context
              //         .add_error(error_builders::build_unexpected_end_error(span, ctx));
              // }
        }
    }
}
