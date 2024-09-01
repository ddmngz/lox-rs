use super::Expr;
use super::LoxObject;
use super::Result;
use super::Visitor;
pub struct Grouping {
    pub expression: Box<dyn Expr>,
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
