pub use crate::token::Token;
use std::fmt;
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
}

impl fmt::Display for ScannedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.line, self.type_)
    }
}
