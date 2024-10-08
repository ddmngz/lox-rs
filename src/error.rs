use crate::interpreter::RuntimeError;
use crate::parser::ParsingError;
pub use crate::scanner::ScanningError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Usage: jlox [script] | jlox")]
    Usage,
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Sorry, I only except Ascii (my bad, its a skill issue)")]
    NotAscii,

    #[error("Scanning Error: {0}")]
    ScanningError(#[from] ScanningError),

    #[error("Parsing Error: {0}")]
    ParsingError(#[from] ParsingError),

    #[error("Runtime Error: {0}")]
    RuntimeError(#[from] RuntimeError),
}

impl Error {
    pub fn error(self, line: u32) -> Self {
        self.report(line, "");
        self
    }

    pub fn report(&self, line: u32, whre: &str) {
        eprintln!("[line {}] at {}: {}", line, whre, self);
    }
}
