use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum RuntimeError {
    #[error("Operator must be a number.")]
    InvalidOperand,

}
