pub mod binary;
pub mod grouping;
pub mod literal;
pub mod unary;

use crate::scanner::Token;
use super::lox_object::LoxObject;
use strum_macros::Display;
use beef::lean::Cow;

pub type Result<T> = std::result::Result<T, crate::interpreter::RuntimeError>;

pub trait Expr{
    fn evaluate(&self) -> Result<LoxObject>;
    fn print(&self) -> String;
}

pub trait Visitor<T>{
    fn accept(&self) -> T;
}

impl<E:Visitor<LoxObject>> Visitor<Result<LoxObject>> for E{
    fn accept(&self) -> Result<LoxObject>{
        Ok(self.accept())
    }
}

impl<E:Visitor<String> + Visitor<Result<LoxObject>>> Expr for E 
{
    fn evaluate(&self) -> Result<LoxObject>{
        <Self as Visitor<Result<LoxObject>>>::accept(self)
    }
    fn print(&self) -> String {
        <Self as Visitor<String>>::accept(self)
    }
}




