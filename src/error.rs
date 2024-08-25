use thiserror::Error;
use crate::scanner::ScanningError;
use crate::parser::ParsingError;
use crate::interpreter::RuntimeError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Usage: jlox [script] | jlox")]
    Usage,
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),

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
