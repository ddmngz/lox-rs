use super::FunctionKind;
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

    #[error("Expected '(' after 'while'.")]
    WhileParenOpen,

    #[error("Expected ')' after while condition.")]
    WhileParenClosed,

    #[error("Expected '(' after 'while'.")]
    ForParenOpen,

    #[error("Expected ')' after while condition.")]
    ForParenClosed,

    #[error("Expected ';' after loop condition.")]
    ConditionNoSemi,

    #[error("Expected ')' after Arguments")]
    FnNoCloseParen,

    #[error("Can't have more than 255 Arguments")]
    TooManyArgs,

    #[error("expected {0} name")]
    ExpectedFn(FunctionKind),

    #[error("Expected '(' after {0} name")]
    FnParenOpen(FunctionKind),
    #[error("Expected ')' after {0} arguments")]
    FnParenClosed(FunctionKind),

    #[error("Expected '{{' before {0} body")]
    FnNoBraceOpen(FunctionKind),
    #[error("Expected '}}' after {0} body")]
    FnNoBraceClosed(FunctionKind),
}
