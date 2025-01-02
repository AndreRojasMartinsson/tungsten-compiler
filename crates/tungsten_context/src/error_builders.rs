use std::ops::Range;

use codespan_reporting::diagnostic::{Diagnostic, Label};

const NON_ASCII_CHARACTER_CODE: &str = "001";
const UNTERMINATED_STRING_CODE: &str = "002";
const INVALID_ESCAPE_SEQUENCE_CODE: &str = "003";
const ILLEGAL_CHARACTER_CODE: &str = "004";
const UNEXPECTED_END_CODE: &str = "005";
const INVALID_UNICODE_CODEPOINT: &str = "006";

pub fn build_non_ascii_character_error(span: Range<usize>, ch: char) -> Diagnostic<()> {
    Diagnostic::error()
        .with_message(format!("Encountered non-ASCII character `{ch}`"))
        .with_code(format!("E{NON_ASCII_CHARACTER_CODE}"))
        .with_notes(vec![
            "Make sure you only use ASCII-compliant characters in the source code".to_string(),
            format!("`{ch}` is not a ASCII character"),
        ])
        .with_labels(vec![
            Label::primary((), span).with_message("illegal character found here")
        ])
}

pub fn build_illegal_character_error(span: Range<usize>, ch: char, ctx: &str) -> Diagnostic<()> {
    Diagnostic::error()
        .with_message(format!("Encountered illegal character `{ch}` in {ctx}"))
        .with_code(format!("E{ILLEGAL_CHARACTER_CODE}"))
        .with_notes(vec![
            // "Make sure you only use ASCII-compliant characters in the source code".to_string(),
            // format!("`{ch}` is not a ASCII character"),
        ])
        .with_labels(vec![
            Label::primary((), span).with_message("illegal character found here")
        ])
}

pub fn build_invalid_unicode_codepoint_error(span: Range<usize>, ch: &str) -> Diagnostic<()> {
    Diagnostic::error()
        .with_message(format!("Encountered invalid unicode codepoint `{ch}`"))
        .with_code(format!("E{INVALID_UNICODE_CODEPOINT}"))
        .with_notes(vec![
            // "Make sure you only use ASCII-compliant characters in the source code".to_string(),
            // format!("`{ch}` is not a ASCII character"),
        ])
        .with_labels(vec![
            Label::primary((), span).with_message("invalid unicode codepoint found here")
        ])
}

pub fn build_unexpected_end_error(span: Range<usize>, ctx: &str) -> Diagnostic<()> {
    Diagnostic::error()
        .with_message(format!("Encountered unexpected end in {ctx}"))
        .with_code(format!("E{UNEXPECTED_END_CODE}"))
        .with_notes(vec![
            // "Make sure you only use ASCII-compliant characters in the source code".to_string(),
            // format!("`{ch}` is not a ASCII character"),
        ])
        .with_labels(vec![
            Label::primary((), span).with_message("unexpected end here")
        ])
}

pub fn build_unterminated_string_error(span: Range<usize>) -> Diagnostic<()> {
    Diagnostic::error()
        .with_message("Encountered an unterminated string literal")
        .with_code(format!("E{UNTERMINATED_STRING_CODE}"))
        .with_notes(vec![
            "Make sure there are no unmatched string quotes in any string literal in the source"
                .to_string(),
        ])
        .with_labels(vec![
            Label::primary((), span.clone()).with_message("unterminated string literal here")
        ])
}

pub fn build_invalid_escape_error(span: Range<usize>, escape: &str) -> Diagnostic<()> {
    Diagnostic::error()
        .with_message(format!(
            "Encountered an invalid escape sequence in string literal `{escape}`"
        ))
        .with_code(format!("E{INVALID_ESCAPE_SEQUENCE_CODE}"))
        .with_notes(vec!["Make sure you use a valid escape sequence".to_string()])
        .with_labels(vec![
            Label::primary((), span.clone()).with_message("invalid escape sequence here")
        ])
}

