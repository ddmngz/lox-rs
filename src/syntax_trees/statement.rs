use super::expression::Expression;
use crate::token::SmartString;
#[derive(Clone, Debug)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    Var {
        name: SmartString,
        initializer: Option<Expression>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
    If {
        condition: Expression,
        then: Box<Statement>,
        else_case: Option<Box<Statement>>,
    },
    Block(Vec<Statement>),
}
