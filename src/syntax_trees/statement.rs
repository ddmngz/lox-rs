use super::expression::Expression;
use crate::token::SmartString;
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    Var {
        name: SmartString,
        initializer: Option<Expression>,
    },
    Block(Vec<Statement>)
}
