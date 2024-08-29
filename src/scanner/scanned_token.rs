use std::fmt;
pub use crate::token::Token;
use typed_builder::TypedBuilder;
/// The Token struct and funcitonality
#[derive(Default, Clone, Debug, TypedBuilder)]
pub struct ScannedToken {
    /// What the token this is, also stores value
    pub type_: Token,
    /// Line number of the lexeme
    line: u32,
}




impl fmt::Display for ScannedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.line, self.type_)
    }
}
