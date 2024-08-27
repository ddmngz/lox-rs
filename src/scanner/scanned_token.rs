use std::fmt;
use std::num::ParseFloatError;
pub use crate::token::Token;
use ascii::AsciiStr;
/// The Token struct and funcitonality
#[derive(Default, Clone, Debug)]
pub struct ScannedToken<'source> {
    /// What the token this is, also stores value
    pub type_: Token<'source>,
    /// Line number of the lexeme
    line: u32,
}

impl<'source> ScannedToken<'source> {
    pub fn new(type_: Token<'source>, line: u32) -> Self {
        Self { type_, line }
    }

    // this would be so easy with nom bytes
    pub fn new_string(lexeme: &'source AsciiStr, line: u32) -> Self {
        Self {
            type_: Token::STRING(lexeme),
            line,
        }
    }

    pub fn new_number(lexeme: &'source AsciiStr, line: u32) -> Result<Self, ParseFloatError> {
        let value: f64 = lexeme.as_str().parse()?;
        let type_ = Token::NUMBER {
            lexeme,
            value,
        };
        Ok(Self { type_, line })
    }

    pub fn eof(line: u32) -> Self {
        Self {
            type_: Token::EOF,
            line,
        }
    }
}

impl fmt::Display for ScannedToken<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.line, self.type_)
    }
}
