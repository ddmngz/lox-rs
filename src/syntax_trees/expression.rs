pub mod binary;
pub mod grouping;
pub mod literal;
pub mod unary;

use super::lox_object::LoxObject;
use crate::token::SmartString;

pub type Result<T> = std::result::Result<T, crate::interpreter::RuntimeError>;
pub use unary::Operator as UnaryOperator;
pub use binary::Operator as BinaryOperator;


pub enum Expression{
    Binary(binary::Binary),
    Grouping(grouping::Grouping),
    Literal(literal::Literal),
    Unary(unary::Unary),
    Variable(SmartString)
}

pub trait Expr {
    fn evaluate(&self) -> Result<LoxObject>;
    fn print(&self) -> String;
}

pub trait Visitor<T> {
    fn accept(&self) -> T;
}

impl<E: Visitor<LoxObject>> Visitor<Result<LoxObject>> for E {
    fn accept(&self) -> Result<LoxObject> {
        Ok(self.accept())
    }
}

impl<E: Visitor<String> + Visitor<Result<LoxObject>>> Expr for E {
    fn evaluate(&self) -> Result<LoxObject> {
        <Self as Visitor<Result<LoxObject>>>::accept(self)
    }
    fn print(&self) -> String {
        <Self as Visitor<String>>::accept(self)
    }
}

impl Expression{
    pub fn binary(left:Expression, operator:BinaryOperator, right:Expression) -> Self{
        Self::Binary(binary::Binary::new(left,operator,right))
    }

    pub fn grouping(expression:Expression) -> Self{
        Self::Grouping(grouping::Grouping::new(expression))
    }
    pub fn unary(operator:UnaryOperator, expression:Expression) -> Self{
        Self::Unary(unary::Unary::new(operator, expression))
    }

    pub fn nil() -> Self{
        Self::Literal(literal::Literal::new(LoxObject::Nil))
    }
}

impl From<f64> for Expression{
    fn from(source:f64) -> Self{
        Self::Literal(literal::Literal::new(LoxObject::Float(source)))
    }
}

impl From<bool> for Expression{
    fn from(source:bool) -> Self{
        Self::Literal(literal::Literal::new(LoxObject::Bool(source)))
    }
}

impl From<SmartString> for Expression{
    fn from(source:SmartString) -> Self{
        Self::Literal(literal::Literal::new(LoxObject::String(source)))
    }
}


// this is kind of a silly way of doing it but I started with enums then switched to trait objects
// then switched back to enums so the sunk cost fallacy is in full swing
impl Expr for Expression{
    fn evaluate(&self) -> Result<LoxObject>{
        match self{
            Self::Binary(b) => b.evaluate(),
            Self::Grouping(g) => g.evaluate(),
            Self::Literal(l) => l.evaluate(),
            Self::Unary(u) => u.evaluate(),
        }
    }

    fn print(&self) -> String{
        match self{
            Self::Binary(b) => b.print(),
            Self::Grouping(g) => g.print(),
            Self::Literal(l) => l.print(),
            Self::Unary(u) => u.print(),
        }
    }
}
