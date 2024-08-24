pub mod token_type;
use byteyarn::ByteYarn;
use std::fmt;
use std::num::ParseFloatError;
pub use token_type::TokenType;
/// The Token struct and funcitonality
#[derive(Default, Clone, Debug)]
pub struct Token {
    /// What the token this is, also stores value
    pub type_: TokenType,
    /// Line number of the lexeme
    line: u32,
}

impl Token {
    pub fn new(type_: TokenType, line: u32) -> Self {
        Self { type_, line }
    }

    pub fn new_string(lexeme: String, line: u32) -> Self {
        Self {
            type_: TokenType::STRING(ByteYarn::from_string(lexeme)),
            line,
        }
    }

    pub fn new_number(lexeme: String, line: u32) -> Result<Self, ParseFloatError> {
        let value: f64 = lexeme.parse()?;
        let type_ = TokenType::NUMBER {
            lexeme: ByteYarn::from_string(lexeme),
            value,
        };
        Ok(Self { type_, line })
    }

    pub fn eof(line: u32) -> Self {
        Self {
            type_: TokenType::EOF,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.line, self.type_)
    }
}
