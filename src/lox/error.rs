use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum LoxError {
    Misused,
    IO,
    UntermString,
    Syntax,
}

impl From<std::io::Error> for LoxError {
    fn from(_: std::io::Error) -> Self {
        Self::IO
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Self::Misused => "Usage: jlox [script].",
            Self::IO => "IO Error.",
            Self::UntermString => "Unterminated string.",
            Self::Syntax => "Unexpected character.",
        };

        write!(f, "{}", msg)
    }
}

impl LoxError {
    pub fn error(&self, line: u32) -> Self {
        self.report(line, "");
        *self
    }

    pub fn report(&self, line: u32, whre: &str) {
        eprintln!("[line {}] Error {}: {}", line, whre, self);
    }

    pub fn usage() -> Self{
        eprintln!("{}",Self::Misused);
        Self::Misused
    }
}
