use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Usage: jlox [script] | jlox")]
    Usage,
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),

    #[error("IO Error: {0}")]
    ScanningError(#[from] ScanningError),

    #[error("IO Error: {0}")]
    ParsingError(#[from] ParsingError),

    #[error("Runtime Error: {0}")]
    RuntimeError(#[from] RuntimeError),
}

#[derive(Debug, Error)]
pub enum ScanningError{
    #[error("Unterminated String")]
    UntermString,
    #[error("Unexpected Character")]
    Syntax,
    #[error("Float Parsing Error")]
    FloatParse(#[from] std::num::ParseFloatError),
}


#[derive(Debug, Clone, Error)]
pub enum RuntimeError {
    #[error("Operator must be a number.")]
    InvalidOperand,
}

#[derive(Debug, Clone, Error)]
pub enum ParsingError {
    #[error("Expected ')' after expression.")]
    UntermParen,
    #[error("Expected Expression.")]
    NoExpr,
    #[error("Expect ';' after expression.")]
    NoSemi,
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
