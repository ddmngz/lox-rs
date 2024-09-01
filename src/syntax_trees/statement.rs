use super::expression::Expression;
use crate::token::Token;
pub enum Statement{
    Expression(Expression),
    Print(Expression),
    Var{
        name:Token,
        initializer: Option<Expression>,
    },
}


