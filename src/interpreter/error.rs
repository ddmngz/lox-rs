use crate::syntax_trees::lox_callable::NotCallable;
use crate::token::SmartString;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum RuntimeError {
    #[error("Operator must be a number.")]
    InvalidOperand,
    #[error("Undefined Variable {0}.")]
    Undefined(SmartString),

    #[error(transparent)]
    CantCall(#[from] NotCallable),

    #[error("Expected {expected} Arguments, but got {got}.")]
    Arity { expected: usize, got: usize },
}
