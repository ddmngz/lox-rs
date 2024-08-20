use super::structs::*;



pub type LoxObject = LiteralValue;
pub struct Interpreter {}

impl Visitor<LoxObject> for Interpreter{
    fn visit_binary(&self, expr: &Binary) -> LoxObject{todo!()}
    fn visit_grouping(&self, expr: &Grouping) -> LoxObject{
        self.evaluate(&Expr::Grouping(expr.clone()))
    }
    fn visit_literal(&self, expr: &Literal) -> LoxObject{
        // could remove the clone soon
        expr.value.clone()
    }
    fn visit_unary(&self, expr: &Unary) -> LoxObject{
        let right = self.evaluate(&expr.right);
        match expr.operator.r#type{
            crate::lox::token::token_type::TokenType::MINUS => -1 * right,
            _ => todo!()
        }
    }

}

impl Interpreter{
    fn evaluate(&self, expr: &Expr) -> LoxObject{
       walk_expr(self,expr) 
    }

}
