use crate::token::SmartString;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum RuntimeError {
    #[error("Operator must be a number.")]
    InvalidOperand,
    #[error("Undefined Variable {0}.")]
    Undefined(SmartString),
}
