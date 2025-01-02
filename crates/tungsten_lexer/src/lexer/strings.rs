use crate::{errors::LexerError, Lexer};

// U+0009 CHARACTER TABULATION, abbreviated <TAB>.
const TAB: char = '\u{9}';

/// U+000B VERTICAL TAB, abbreviated <VT>.
const VT: char = '\u{b}';

/// U+000C FORM FEED, abbreviated <FF>.
const FF: char = '\u{c}';

///  U+000A LINE FEED, abbreviated in the spec as <LF>.
const LF: char = '\u{a}';

/// U+000D CARRIAGE RETURN, abbreviated in the spec as <CR>.
const CR: char = '\u{d}';

/// U+2028 LINE SEPARATOR, abbreviated <LS>.
const LS: char = '\u{2028}';

/// U+2029 PARAGRAPH SEPARATOR, abbreviated <PS>.
const PS: char = '\u{2029}';

impl Lexer<'_> {
    pub(crate) fn read_string_literal(&mut self) -> Result<(), LexerError> {
        loop {
            match self.chars.next() {
                None | Some('\r') | Some('\n') => return Err(LexerError::UnterminatedString),
                Some(_c @ '"') => break,
                Some('\\') => {
                    self.read_escape_sequence()?;
                }
                Some(other) => {
                    self.buffer.push(other);
                }
            }
        }
        Ok(())
    }

    pub(crate) fn read_escape_sequence(&mut self) -> Result<(), LexerError> {
        match self.chars.next() {
            None => return Err(LexerError::UnterminatedString),
            Some(c) => match c {
                LF | LS | PS => {
                    // Ignore line continuation
                }
                CR => {
                    // LineContinuation, check for the sequence \r\n; otherwise
                    // ignore it.
                    if self.peek() == Some(LF) {
                        self.chars.next();
                    }
                }
                '\'' | '"' | '\\' => {
                    self.buffer.push(c);
                }
                'b' => {
                    self.buffer.push('\u{8}');
                }
                'f' => {
                    self.buffer.push(FF);
                }
                'n' => {
                    self.buffer.push(LF);
                }
                'r' => {
                    self.buffer.push(CR);
                }
                't' => {
                    self.buffer.push(TAB);
                }
                'v' => {
                    self.buffer.push(VT);
                }
                'x' => {
                    let mut value = self.read_hex_digit()?;
                    value = (value << 4) | self.read_hex_digit()?;

                    match char::try_from(value) {
                        Err(_) => return Err(LexerError::InvalidEscape),
                        Ok(c) => self.buffer.push(c),
                    }
                }
                'u' => {
                    let c = self.read_unicode_escape_sequence_after_backslash_and_u()?;
                    self.buffer.push(c)
                }
                other => self.buffer.push(other),
            },
        }

        Ok(())
    }

    pub(crate) fn read_unicode_escape_sequence_after_backslash_and_u(
        &mut self,
    ) -> Result<char, LexerError> {
        let value = match self.peek() {
            Some('{') => {
                self.chars.next();

                let value = self.read_code_point()?;
                match self.chars.next() {
                    Some('}') => {}
                    _ => return Err(LexerError::InvalidEscape),
                }

                value
            }
            _ => self.read_hex_4_digits()?,
        };

        Ok(value)
    }

    pub(crate) fn read_code_point(&mut self) -> Result<char, LexerError> {
        let mut value = self.read_hex_digit()?;

        loop {
            let next = match self.peek() {
                None => return Err(LexerError::InvalidEscape),
                Some(c @ '0'..='9') => c as u32 - '0' as u32,
                Some(c @ 'a'..='f') => 10 + (c as u32 - 'a' as u32),
                Some(c @ 'A'..='F') => 10 + (c as u32 - 'A' as u32),
                Some(_) => break,
            };
            self.chars.next();
            value = (value << 4) | next;
            if value > 0x10FFFF {
                return Err(LexerError::InvalidEscape);
            }
        }

        Self::code_point_to_char(value)
    }

    pub(crate) fn code_point_to_char(value: u32) -> Result<char, LexerError> {
        if (0xd800..=0xdfff).contains(&value) {
            Err(LexerError::UnicodeEscape)
        } else {
            char::try_from(value).map_err(|_| LexerError::InvalidUnicode)
        }

        // if 0xd800 <= value && value <= 0xdfff {
        //     bail!("Unicode escape sequences (surrogates)")
        // } else {
        //     Ok(char::try_from(value)?)
        // }
    }
}
