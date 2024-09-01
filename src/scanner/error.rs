use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScanningError {
    #[error("Unterminated String")]
    UntermString,
    #[error("Unexpected Character")]
    Syntax,
    #[error("Float Parsing Error")]
    FloatParse(#[from] std::num::ParseFloatError),
}

impl ScanningError {
    pub fn error(self, line: u32) -> Self {
        self.report(line, "");
        self
    }

    pub fn report(&self, line: u32, whre: &str) {
        eprintln!("[line {}] at {}: {}", line, whre, self);
    }
}
