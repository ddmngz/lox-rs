use super::expression::{Expr, Result};

pub trait StatementVisitor<T> {
    fn visit_expression_statement(statement: Expr) -> Result<T>;
    fn visit_print_statement(statement: Expr) -> Result<T>;
}

pub enum Statement {
    Expression(Expr),
    Print(Expr),
}
