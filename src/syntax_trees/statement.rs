use super::expression::{Expr, Result};

pub trait Statement<'s>{
    fn execute(&self) -> Result<()>;
}

impl Statement<'_> for Expression<'_>{
    fn execute(&self) -> Result<()>{
        self.0.evaluate()?;
        Ok(())
    }
}

impl Statement<'_> for Print<'_>{
    fn execute(&self) -> Result<()>{
        println!("{}", self.0.evaluate()?);
        Ok(())
    }
}

pub struct Expression<'s>(Box<dyn Expr<'s>>);
pub struct Print<'s>(Box<dyn Expr<'s>>);

