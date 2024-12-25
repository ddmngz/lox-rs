use super::expression::Expression;
use crate::token::SmartString;
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    Var {
        name: SmartString,
        initializer: Option<Expression>,
    },
    If {
        condition: Expression,
        then: Box<Statement>,
        else_case: Option<Box<Statement>>,
    },
    Block(Vec<Statement>),
}
