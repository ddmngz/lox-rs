use super::expression::Expression;

pub enum Statement{
    Expression(Expression),
    Print(Expression)
}


