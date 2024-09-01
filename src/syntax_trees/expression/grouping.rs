use super::Expression;
use super::Expr;
use super::LoxObject;
use super::Result;
use super::Visitor;
pub struct Grouping {
    pub expression: Box<Expression>,
}

impl Grouping{
    pub fn new(e:Expression) -> Self{
        Self{
            expression: Box::new(e)
        }
    }
}

impl Visitor<String> for Grouping {
    fn accept(&self) -> String {
        self.expression.print()
    }
}

impl Visitor<Result<LoxObject>> for Grouping {
    fn accept(&self) -> Result<LoxObject> {
        self.expression.evaluate()
    }
}
