use crate::{errors::LexerError, numeric_result::NumericResult, Lexer};

impl Lexer<'_> {
    pub(crate) fn read_hex_4_digits(&mut self) -> Result<char, LexerError> {
        let mut value = 0;
        for _ in 0..4 {
            value = (value << 4) | self.read_hex_digit()?;
        }

        Self::code_point_to_char(value)
    }

    pub(crate) fn read_hex_digit(&mut self) -> Result<u32, LexerError> {
        match self.chars.next() {
            None => Err(LexerError::InvalidEscape(self.buffer.clone())),
            Some(c @ '0'..='9') => Ok(c as u32 - '0' as u32),
            Some(c @ 'a'..='f') => Ok(10 + (c as u32 - 'a' as u32)),
            Some(c @ 'A'..='F') => Ok(10 + (c as u32 - 'A' as u32)),
            Some(c) => Err(LexerError::IllegalCharacter {
                ch: c,
                ctx: "hex digit",
            }),
        }
    }

    pub(crate) fn read_numeric_literal_starting_with_zero(
        &mut self,
    ) -> Result<NumericResult, LexerError> {
        match self.peek() {
            Some('.') => {
                self.push_to_buffer();
                self.read_float_after_decimal_point_after_digits()?;
                return Ok(NumericResult::Float);
            }
            Some('e') | Some('E') => {
                self.push_to_buffer();
                self.read_decimal_exponent()?;
                return Ok(NumericResult::Float);
            }
            _ => {}
        };

        Ok(NumericResult::Integer)
    }

    pub(crate) fn read_float_after_decimal_point_after_digits(
        &mut self,
    ) -> Result<NumericResult, LexerError> {
        self.read_optional_decimal_digits()?;
        self.read_optional_exponent()?;

        Ok(NumericResult::Float)
    }

    pub(crate) fn read_optional_decimal_digits(&mut self) -> Result<(), LexerError> {
        if let Some('0'..='9') = self.peek() {
            self.push_to_buffer();
        } else {
            return Ok(());
        }

        self.read_decimal_digits_after_first_digit()?;
        Ok(())
    }

    pub(crate) fn read_float_after_decimal_point(&mut self) -> Result<(), LexerError> {
        // The parts after `.` in
        //
        // `.` DecimalDigits ExponentPart?
        self.read_decimal_digits()?;
        self.read_optional_exponent()?;

        Ok(())
    }

    pub(crate) fn read_decimal_digits(&mut self) -> Result<(), LexerError> {
        if let Some('0'..='9') = self.peek() {
            self.push_to_buffer();
        } else if let Some(c) = self.peek() {
            return Err(LexerError::IllegalCharacter {
                ch: c,
                ctx: "number",
            });
        } else if self.peek().is_none() {
            return Err(LexerError::UnexpectedEnd("number"));
        };

        self.read_decimal_digits_after_first_digit()?;

        Ok(())
    }

    pub(crate) fn read_decimal_literal_after_first_digit(
        &mut self,
    ) -> Result<NumericResult, LexerError> {
        self.read_decimal_digits_after_first_digit()?;

        if let Some('.') = self.peek() {
            self.push_to_buffer();
            return self.read_float_after_decimal_point_after_digits();
        }

        // match self.peek() {
        //     Some('.') => {
        //         self.push_to_buffer();
        //         return self.read_float_after_decimal_point_after_digits();
        //     }
        //     _ => {}
        // }

        let has_exponent = self.read_optional_exponent()?;
        let result = if has_exponent {
            NumericResult::Float
        } else {
            NumericResult::Integer
        };

        Ok(result)
    }

    pub(crate) fn read_decimal_digits_after_first_digit(&mut self) -> Result<(), LexerError> {
        while let Some(next) = self.peek() {
            match next {
                '_' => {
                    self.chars.next();

                    if let Some('0'..='9') = self.peek() {
                        self.push_to_buffer();
                    } else if let Some(c) = self.peek() {
                        return Err(LexerError::IllegalCharacter {
                            ch: c,
                            ctx: "number",
                        });
                    } else if self.peek().is_none() {
                        return Err(LexerError::UnexpectedEnd("number"));
                    };
                }
                '0'..='9' => {
                    self.push_to_buffer();
                }
                _ => break,
            }
        }

        Ok(())
    }

    pub(crate) fn read_optional_exponent(&mut self) -> Result<bool, LexerError> {
        if let Some('e') | Some('E') = self.peek() {
            self.push_to_buffer();
            self.read_decimal_exponent()?;

            return Ok(true);
        }

        Ok(false)
    }

    pub(crate) fn read_decimal_exponent(&mut self) -> Result<(), LexerError> {
        if let Some('+') | Some('-') = self.peek() {
            self.push_to_buffer();
        }

        self.read_decimal_digits()?;

        Ok(())
    }
}
