use super::expression::{Expr, Result};

pub trait Visitor<T> {
    fn visit_expression(&mut self, statement: Expression) -> Result<T>;
    fn visit_print(&mut self, statement: Print) -> Result<T>;
}


pub enum Statement{
    Expression(Expression),
    Print(Print),
}

impl Statement{
    pub fn as_expr(&self) -> &Expr{
        match self{
            Self::Expression(e) => &e.0,
            Self::Print(e) => &e.0,
        }
    }

    pub fn into_expr(self) -> Expr{
        match self{
            Self::Expression(e) => e.0,
            Self::Print(e) => e.0,
        }
    }

    pub fn new_expression(expr:Expr) -> Self{
        let expression = Expression(expr);
        Self::Expression(expression)
    }

    pub fn new_print(expr:Expr) -> Self{
        let expression = Print(expr);
        Self::Print(expression)
    }
}

impl Into<Expr> for Expression{
    fn into(self) -> Expr{
        self.0
    }
}

impl Into<Expr> for Print{
    fn into(self) -> Expr{
        self.0
    }
}

impl AsRef<Expr> for Expression{
    fn as_ref(&self) -> &Expr{
        &self.0
    }
}

impl AsRef<Expr> for Print{
    fn as_ref(&self) -> &Expr{
        &self.0
    }
}

pub struct Expression(pub Expr);
pub struct Print(pub Expr);

