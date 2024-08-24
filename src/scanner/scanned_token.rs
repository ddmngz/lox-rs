
use byteyarn::ByteYarn;
use std::fmt;
use std::num::ParseFloatError;
pub use crate::token::Token;
/// The Token struct and funcitonality
#[derive(Default, Clone, Debug)]
pub struct ScannedToken {
    /// What the token this is, also stores value
    pub type_: Token,
    /// Line number of the lexeme
    line: u32,
}

impl ScannedToken {
    pub fn new(type_: Token, line: u32) -> Self {
        Self { type_, line }
    }

    pub fn new_string(lexeme: String, line: u32) -> Self {
        Self {
            type_: Token::STRING(ByteYarn::from_string(lexeme)),
            line,
        }
    }

    pub fn new_number(lexeme: String, line: u32) -> Result<Self, ParseFloatError> {
        let value: f64 = lexeme.parse()?;
        let type_ = Token::NUMBER {
            lexeme: ByteYarn::from_string(lexeme),
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

impl fmt::Display for ScannedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.line, self.type_)
    }
}
