use super::expression::{Expr, Result};

pub trait Statement{
    fn execute(&self) -> Result<()>;
}

impl Statement for Expression{
    fn execute(&self) -> Result<()>{
        self.0.evaluate()?;
        Ok(())
    }
}

impl Statement for Print{
    fn execute(&self) -> Result<()>{
        println!("{}", self.0.evaluate()?);
        Ok(())
    }
}

pub struct Expression(Box<dyn Expr>);
pub struct Print(Box<dyn Expr>);

