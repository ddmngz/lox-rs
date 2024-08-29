use super::Expr;
use super::Visitor;
use super::LoxObject;
use super::Result;
pub struct Grouping {
    pub expression: Box<dyn Expr>,
}


impl Visitor<String> for Grouping{
    fn accept(&self) -> String{
        self.expression.print()
    }
}

impl Visitor<Result<LoxObject>> for Grouping{
    fn accept(&self) -> Result<LoxObject>{
        self.expression.evaluate()
    }
}

