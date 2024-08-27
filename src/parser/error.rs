use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ParsingError {
    #[error("Expected ')' after expression.")]
    UntermParen,
    #[error("Expected Expression.")]
    NoExpr,
    #[error("Expect ';' after expression.")]
    NoSemi,
}
