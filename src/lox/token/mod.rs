pub mod token_type;
use std::fmt;
use std::num::ParseFloatError;
use std::str::FromStr;
pub use token_type::TokenType;
/// The Token struct and funcitonality
#[derive(Default, Clone)]
pub struct Token {
    /// What the token this is, also stores value
    pub r#type: TokenType,
    /// The base lexeme we're holding
    pub lexeme: String,
    /// Line number of the lexeme
    line: u32,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: &str, line: u32) -> Self {
        let lexeme = String::from(lexeme);
        Self {
            r#type,
            lexeme,
            line,
        }
    }

    pub fn new_string(lexeme: &str, line: u32) -> Self {
        let literal = String::from(lexeme);
        Self {
            r#type: TokenType::STRING(literal.clone()),
            lexeme: literal,
            line,
        }
    }

    pub fn new_number(lexeme: &str, line: u32) -> Result<Self, ParseFloatError> {
        let literal = f64::from_str(lexeme)?;
        Ok(Self {
            r#type: TokenType::NUMBER(literal),
            lexeme: String::from(lexeme),
            line,
        })
    }

    pub fn eof(line: u32) -> Self {
        Self {
            r#type: TokenType::EOF,
            lexeme: String::from(""),
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r#type, self.lexeme, self.line)
    }
}
