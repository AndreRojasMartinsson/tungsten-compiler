use std::ops::Range;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("illegal character: `{ch}`, in `{ctx}`")]
    IllegalCharacter { ch: char, ctx: &'static str },

    #[error("encountered non-ascii character `{0}`")]
    NonAsciiCharacter(char),

    #[error("unterminated string literal")]
    UnterminatedString,

    #[error("invalid string escape sequence")]
    InvalidEscape(String),

    #[error("invalid unicode code point")]
    InvalidUnicode,

    #[error("unicode escape sequences (surrogates)")]
    UnicodeEscape,

    #[error("unexpected end in {0}")]
    UnexpectedEnd(&'static str),
}
