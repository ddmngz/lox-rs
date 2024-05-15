pub mod token_type;

use std::fmt;
pub use token_type::TokenType;

pub struct Token<'a> {
    typ_e: TokenType<'a>,
    lexeme: &'a str,
    line: u32,
}

impl<'a> Token<'a> {
    pub fn new(typ: TokenType<'a>, lexeme: &'a str, line: u32) -> Self {
        Self {
            typ_e: typ,
            lexeme,
            line,
        }
    }

    pub fn eof(line: u32) -> Self {
        Self {
            typ_e: TokenType::EOF,
            lexeme: (""),
            line,
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.typ_e, self.lexeme, self.line)
    }
}
