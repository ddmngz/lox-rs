use super::expression::{Expr, Result};

pub trait Visitor<T> {
    fn visit_expression(statement: Expr) -> Result<T>;
    fn visit_print(statement: Expr) -> Result<T>;
}

pub enum Statement {
    Expression(Expr),
    Print(Expr),
}
