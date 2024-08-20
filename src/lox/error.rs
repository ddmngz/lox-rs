use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoxError {
    #[error("Usage: jlox [script] | jlox")]
    Misused,
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
    
    #[error("Unterminated String")]
    UntermString,

    #[error("Unexpected Character")]
    Syntax,

    #[error("Float Parsing Error")]
    FloatParse(#[from] std::num::ParseFloatError),

    #[error("Parsing Error: {0}")]
    ParsingError(#[from] LoxParsingError),

    #[error("Runtime Error: {0}")]
    RuntimeError(#[from]LoxRuntimeError),
}

#[derive(Debug, Clone, Error)]
pub enum LoxRuntimeError{
}

#[derive(Debug, Clone, Error)]
pub enum LoxParsingError {
    #[error("Expected ')' after expression.")]
    UntermParen,
    #[error("Expected Expression.")]
    NoExpr,
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
