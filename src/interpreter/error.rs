use crate::token::SmartString;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum RuntimeError {
    #[error("Operator must be a number.")]
    InvalidOperand,
    #[error("Undefined Variable {0}.")]
    Undefined(SmartString),

    #[error("Can only call functions and classes.")]
    NotCallable,

    #[error("Expected {expected} Arguments, but got {got}.")]
    Arity { expected: usize, got: usize },
}
