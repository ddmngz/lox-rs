use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum LoxError {
    Misused,
    IO,
    UntermString,
    Syntax,
    FloatParse,
    ParsingError(LoxParsingError),
}
#[derive(Debug, Clone)]
pub enum LoxParsingError {
    UntermParen,
    NoExpr,
}

impl From<LoxParsingError> for LoxError {
    fn from(error: LoxParsingError) -> Self {
        Self::ParsingError(error)
    }
}

impl From<std::io::Error> for LoxError {
    fn from(_: std::io::Error) -> Self {
        Self::IO
    }
}

impl From<std::num::ParseFloatError> for LoxError {
    fn from(_: std::num::ParseFloatError) -> Self {
        Self::FloatParse
    }
}

impl Display for LoxParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Self::UntermParen => "Expected ')' after expression.",
            Self::NoExpr => "Expected Expression.",
        };
        write!(f, "{}", msg)
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message_owner: String;
        let msg = match self {
            Self::Misused => "Usage: jlox [script].",
            Self::IO => "IO Error.",
            Self::UntermString => "Unterminated string.",
            Self::Syntax => "Unexpected character.",
            Self::FloatParse => "Float Parsing Error.",
            Self::ParsingError(error) => {
                message_owner = error.to_string();
                &message_owner
            }
        };

        write!(f, "{}", msg)
    }
}

impl LoxError {
    pub fn error(self, line: u32) -> Self {
        self.report(line, "");
        self
    }

    pub fn report(&self, line: u32, whre: &str) {
        eprintln!("[line {}] Error {}: {}", line, whre, self);
    }

    pub fn usage() -> Self {
        eprintln!("{}", Self::Misused);
        Self::Misused
    }
}
