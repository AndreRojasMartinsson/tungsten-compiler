use std::str::Chars;

use tungsten_context::CompilerContext;
use tungsten_utils::{atom, Atom};

mod numbers;
mod strings;

use crate::{
    errors::LexerError, is_keyword, numeric_result::NumericResult, str_to_keyword_kind, Kind,
    Position, Token, Value,
};

#[derive(Debug)]
pub struct Lexer<'a> {
    pub(crate) context: &'a mut CompilerContext<'a>,
    pub(crate) source: &'a str,
    pub(crate) chars: Chars<'a>,
    pub(crate) buffer: String,
}

impl<'a> Lexer<'a> {
    pub fn new(context: &'a mut CompilerContext<'a>, source: &'a str) -> Self {
        Self {
            chars: source.chars(),
            buffer: String::new(),
            context,
            source,
        }
    }

    pub fn is_at_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_eof() {
            let token = self.read_next();
            tokens.push(token);
        }

        self.context.report_errors();

        tokens
    }

    pub fn read_next(&mut self) -> Token {
        let start = self.offset();
        let (kind, value) = self.read_next_kind();
        let end = self.offset();

        let trimmed = &self.source[start..end].trim_start_matches(char::is_whitespace);
        let len = self.source[start..end].len() - trimmed.len();
        println!("{:?}", &self.source[start + len..end]);

        // Column offset computation causes first line to have incorrect column number (starting at 0,
        // instead of 1)
        let (line, mut column) = self.calculate_line_column(start + len);
        if line == 1 {
            column += 1;
        }

        Token {
            span: start..end,
            position: Position { line, column },
            lexeme: atom!(self.source[start..end].trim_start()),
            kind,
            value,
        }
    }

    fn calculate_line_column(&self, start_offset: usize) -> (usize, usize) {
        let line = &self.source[..start_offset].matches('\n').count() + 1;
        let last_newline_index = &self.source[..start_offset].rfind('\n').unwrap_or(0);
        let column = start_offset - last_newline_index;

        (line, column)
    }

    pub(crate) fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    pub(crate) fn read_next_kind(&mut self) -> (Kind, Option<Value>) {
        while let Some(c) = self.chars.next() {
            match c {
                // Lookahead(0) Tokens
                ',' => return (Kind::Comma, None),
                ';' => return (Kind::Semicolon, None),
                '@' => return (Kind::At, None),
                '#' => return (Kind::Hash, None),
                '[' => return (Kind::LBracket, None),
                ']' => return (Kind::RBracket, None),
                ')' => return (Kind::RParen, None),
                '}' => return (Kind::RBrace, None),
                '?' => return (Kind::Question, None),
                '~' => return (Kind::Tilde, None),
                // Lookahead(1) Tokens
                '$' => match self.peek() {
                    Some('$') => {
                        self.chars.next();
                        return (Kind::DoubleDollar, None);
                    }
                    _ => panic!("Illegal character '$'"),
                },
                ':' => match self.peek() {
                    Some(':') => {
                        self.chars.next();
                        return (Kind::DoubleColon, None);
                    }
                    _ => return (Kind::Colon, None),
                },
                '^' => match self.peek() {
                    Some('=') => {
                        self.chars.next();
                        return (Kind::CaretAssign, None);
                    }
                    _ => return (Kind::Caret, None),
                },
                '+' => match self.peek() {
                    Some('+') => {
                        self.chars.next();
                        return (Kind::DoublePlus, None);
                    }
                    Some('=') => {
                        self.chars.next();
                        return (Kind::PlusAssign, None);
                    }
                    _ => return (Kind::Plus, None),
                },
                '-' => match self.peek() {
                    Some('>') => {
                        self.chars.next();
                        return (Kind::Arrow, None);
                    }
                    Some('-') => {
                        self.chars.next();
                        return (Kind::DoubleDash, None);
                    }
                    Some('=') => {
                        self.chars.next();
                        return (Kind::DashAssign, None);
                    }
                    _ => return (Kind::Dash, None),
                },
                '%' => match self.peek() {
                    Some('=') => {
                        self.chars.next();
                        return (Kind::PercentAssign, None);
                    }
                    _ => return (Kind::Percent, None),
                },
                '!' => match self.peek() {
                    // !=
                    Some('=') => {
                        self.chars.next();

                        return (Kind::BangEqual, None);
                    }
                    _ => return (Kind::Bang, None),
                },
                '(' => match self.peek() {
                    // (|
                    Some('|') => {
                        self.chars.next();

                        return (Kind::LParPipe, None);
                    }
                    _ => return (Kind::LParen, None),
                },
                '{' => match self.peek() {
                    // (|
                    Some('|') => {
                        self.chars.next();

                        return (Kind::LBraPipe, None);
                    }
                    _ => return (Kind::LBrace, None),
                },
                '&' => match self.peek() {
                    // ||
                    Some('&') => {
                        self.chars.next();

                        return (Kind::DoubleAmpersand, None);
                    }
                    // |)
                    Some('=') => {
                        self.chars.next();

                        return (Kind::AmpersandAssign, None);
                    }
                    _ => return (Kind::Ampersand, None),
                },
                '|' => match self.peek() {
                    // ||
                    Some('|') => {
                        self.chars.next();

                        return (Kind::DoublePipe, None);
                    }
                    // |>
                    Some('>') => {
                        self.chars.next();

                        return (Kind::ReturnKw, None);
                    }
                    // |)
                    Some(')') => {
                        self.chars.next();

                        return (Kind::RParPipe, None);
                    }
                    // |}
                    Some('}') => {
                        self.chars.next();

                        return (Kind::RBraPipe, None);
                    }
                    // |=
                    Some('=') => {
                        self.chars.next();

                        return (Kind::PipeAssign, None);
                    }
                    _ => return (Kind::Pipe, None),
                },

                // Lookahead(2) Tokens
                '*' => match self.peek() {
                    // *=
                    Some('=') => {
                        self.chars.next();

                        return (Kind::AsteriskAssign, None);
                    }
                    // **
                    Some('*') => {
                        self.chars.next();

                        match self.peek() {
                            // **=
                            Some('=') => {
                                self.chars.next();
                                return (Kind::DoubleAsteriskAssign, None);
                            }
                            // **
                            _ => return (Kind::DoubleAsterisk, None),
                        }
                    }
                    _ => return (Kind::Asterisk, None),
                },
                '/' => match self.peek() {
                    // /=
                    Some('=') => {
                        self.chars.next();

                        return (Kind::SlashAssign, None);
                    }
                    // //
                    Some('/') => {
                        self.chars.next();

                        match self.peek() {
                            // //=
                            Some('=') => {
                                self.chars.next();
                                return (Kind::DoubleSlashAssign, None);
                            }
                            // //
                            _ => return (Kind::DoubleSlash, None),
                        }
                    }
                    _ => return (Kind::Slash, None),
                },
                '>' => match self.peek() {
                    // >>
                    Some('>') => {
                        self.chars.next();

                        match self.peek() {
                            // >>=
                            Some('=') => {
                                self.chars.next();
                                return (Kind::DoubleGreaterAssign, None);
                            }
                            _ => return (Kind::DoubleGreater, None),
                        }
                    }
                    // >=
                    Some('=') => {
                        self.chars.next();

                        return (Kind::GreaterEq, None);
                    }
                    _ => return (Kind::Greater, None),
                },
                '=' => match self.peek() {
                    // =>
                    Some('>') => {
                        self.chars.next();

                        return (Kind::FatArrow, None);
                    }
                    // ==
                    Some('=') => {
                        self.chars.next();

                        return (Kind::DoubleEqual, None);
                    }
                    _ => return (Kind::Equal, None),
                },
                '<' => match self.peek() {
                    // <<
                    Some('<') => {
                        self.chars.next();

                        match self.peek() {
                            // <<=
                            Some('=') => {
                                self.chars.next();
                                return (Kind::DoubleLessAssign, None);
                            }
                            _ => return (Kind::DoubleLess, None),
                        }
                    }
                    // <>
                    Some('>') => {
                        self.chars.next();

                        return (Kind::LessGreater, None);
                    }
                    // <=
                    Some('=') => {
                        self.chars.next();

                        return (Kind::LessEq, None);
                    }
                    _ => return (Kind::Less, None),
                },
                // Literals
                // Number Literals
                // Eg: .345 .892858
                '.' => match self.peek() {
                    // ..
                    Some('.') => {
                        self.chars.next();

                        match self.peek() {
                            // ...
                            Some('.') => {
                                self.chars.next();
                                return (Kind::Ellipsis, None);
                            }
                            // ..=
                            Some('=') => {
                                self.chars.next();
                                return (Kind::DoublePeriodAssign, None);
                            }
                            _ => return (Kind::DoublePeriod, None),
                        }
                    }
                    Some('0'..='9') => {
                        self.buffer.clear();
                        self.buffer.push('.'); // Push the initial period

                        if let Err(err) = self.read_float_after_decimal_point() {
                            self.report_error(err);
                        }

                        let raw_value = self.buffer.clone();
                        self.buffer.clear();

                        let parsed = raw_value.parse::<f64>().expect("Could not parse to f64");

                        return (Kind::FloatLiteral, Some(Value::Float(parsed)));
                    }
                    _ => return (Kind::Period, None),
                },
                // Read Zero, or Float
                '0' => {
                    self.buffer.clear();
                    self.buffer.push('0'); // Push the initial zero

                    match self.read_numeric_literal_starting_with_zero() {
                        Err(err) => {
                            self.report_error(err);
                        }
                        Ok(numeric_result) => {
                            let raw_value = self.buffer.clone();
                            self.buffer.clear();

                            return match numeric_result {
                                NumericResult::Integer => {
                                    let parsed =
                                        raw_value.parse::<u64>().expect("Could not parse to u64");

                                    (Kind::IntegerLiteral, Some(Value::Integer(parsed)))
                                }
                                NumericResult::Float => {
                                    let parsed =
                                        raw_value.parse::<f64>().expect("Could not parse to f64");

                                    (Kind::FloatLiteral, Some(Value::Float(parsed)))
                                }
                            };
                        }
                    }
                }
                // Read number
                '1'..='9' => {
                    self.buffer.clear();
                    self.buffer.push(c); // Push the initial digit

                    match self.read_decimal_literal_after_first_digit() {
                        Err(err) => {
                            self.report_error(err);
                        }
                        Ok(numeric_result) => {
                            let raw_value = self.buffer.clone();
                            self.buffer.clear();

                            return match numeric_result {
                                NumericResult::Integer => {
                                    let parsed =
                                        raw_value.parse::<u64>().expect("Could not parse to u64");

                                    (Kind::IntegerLiteral, Some(Value::Integer(parsed)))
                                }
                                NumericResult::Float => {
                                    let parsed =
                                        raw_value.parse::<f64>().expect("Could not parse to f64");

                                    (Kind::FloatLiteral, Some(Value::Float(parsed)))
                                }
                            };
                        }
                    }
                }
                // Read String
                '"' => {
                    self.buffer.clear();

                    if let Err(err) = self.read_string_literal() {
                        self.report_error(err);
                    }

                    let raw_value = self.buffer.clone();
                    self.buffer.clear();

                    return (Kind::StringLiteral, Some(Value::String(atom!(raw_value))));
                }
                '_' | 'a'..='z' | 'A'..='Z' => return self.read_identifier(c),
                ' ' | '\t' | '\r' | '\n' => {}
                // TODO COMMENTS!
                ch => {
                    self.report_error(LexerError::NonAsciiCharacter(ch));
                }
            }
        }
        (Kind::Eof, None)
    }

    fn report_error(&mut self, err: LexerError) {
        match err {
            LexerError::NonAsciiCharacter(ch) => {
                self.context
                    .add_error(&format!("Encountered non-ascii character `{ch}`"));
            }
            LexerError::UnterminatedString => {
                self.context.add_error("Unterminated String Literal");
            }
            LexerError::InvalidEscape => {
                self.context.add_error("Invalid String Escape Sequence");
            }
            LexerError::InvalidUnicode => {
                self.context.add_error("Invalid Unicode Code Point");
            }
            LexerError::UnicodeEscape => {
                self.context
                    .add_error("Unicode escape sequences (surrogates)");
            }
            LexerError::IllegalCharacter { ch, ctx } => {
                self.context
                    .add_error(&format!("Illegal character: `{ch}`, in {ctx}"));
            }
            LexerError::UnexpectedEnd(ctx) => {
                self.context
                    .add_error(&format!("Encountered unexpected end of stream in {ctx}"));
            }
        };
    }

    fn read_identifier(&mut self, initial_char: char) -> (Kind, Option<Value>) {
        self.buffer.clear();
        self.buffer.push(initial_char);

        while let Some('_' | 'a'..='z' | 'A'..='Z' | '0'..='9') = self.peek() {
            self.push_to_buffer();
        }

        let raw_value = self.buffer.clone();
        self.buffer.clear();

        match raw_value.as_ref() {
            "true" => (Kind::BooleanLiteral, Some(Value::Boolean(true))),
            "false" => (Kind::BooleanLiteral, Some(Value::Boolean(false))),
            other if is_keyword(other) => (str_to_keyword_kind(other).unwrap(), None),
            other => (Kind::Identifier, Some(Value::String(atom!(other)))),
        }
    }

    pub(crate) fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub(crate) fn push_to_buffer(&mut self) {
        self.buffer.push(self.chars.next().unwrap());
    }
}