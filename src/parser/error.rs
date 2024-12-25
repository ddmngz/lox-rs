use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ParsingError {
    #[error("Expected ')' after expression.")]
    UntermParen,
    #[error("Expected Expression.")]
    NoExpr,
    #[error("Expect ';' after expression.")]
    NoSemi,
    #[error("Expected Variable Name.")]
    NoVarName,
    #[error("Invalid.")]
    Invalid,
    #[error("Expected Identifier.")]
    NoIdentifier,
    #[error("Invalid Assignment Target.")]
    InvalidAssignment,

    #[error("Expected '}}' after Block.")]
    UntermBrace,

    #[error("Expected '(' after 'if'.")]
    IfParenOpen,
    #[error("Expected ')' after if condition.")]
    IfParenClosed,
}
