use super::Visitor;
use super::Expr;
use super::Result;
use crate::syntax_trees::lox_object::LoxObject;
use strum_macros::Display;
pub struct Unary {
    pub operator: Operator,
    pub inner: Box<dyn Expr>,
}

#[derive(Clone, Display, Debug)]
pub enum Operator {
    #[strum(serialize = "!")]
    BANG,
    #[strum(serialize = "-")]
    MINUS,
}

impl Visitor<String> for Unary{
    fn accept(&self) -> String{
        let inner = self.inner.print();
        format!("{}{}",self.operator, inner)
    }
}

impl Visitor<Result<LoxObject>> for Unary{
    fn accept(&self) -> Result<LoxObject>{
        let inner = self.inner.evaluate()?;
        match self.operator{
            Operator::BANG => {!inner},
            Operator::MINUS => {-inner},
        }
    }
}
