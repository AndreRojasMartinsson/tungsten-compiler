use std::ops::Range;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("duplicate function declaration `{name}`")]
    DuplicateFunction {
        name: String,
        orig_span: Range<usize>,
        func_span: Range<usize>,
    },
}
