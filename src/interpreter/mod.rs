mod error;
use std::io::Write;
pub use error::RuntimeError;
use crate::syntax_trees::{expression::{self, *},statement::{self, Statement}, lox_object::LoxObject};

pub struct Interpreter {}
pub type Result<T> = std::result::Result<T, RuntimeError>;

pub fn interpret(statements: impl Iterator<Item = Statement>) -> Result<()>{
    let mut interpreter = Interpreter{};
    for statement_ in statements{
        interpreter.execute(statement_)?;
    }

    Ok(())
}

impl statement::Visitor<()> for Interpreter {
    fn visit_expression(&mut self, statement: statement::Expression) -> Result<()> {
        self.evaluate(statement.as_ref())?;
        Ok(())
    }

    fn visit_print(&mut self, statement: statement::Print) -> Result<()> {
        let value = self.evaluate(statement.as_ref())?;
        println!("{value}");
        let _ = std::io::stdout().flush();
        Ok(())
    }
}



impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<LoxObject> {
        walk_expr(self, expr)
    }
    
    pub fn execute(&mut self, statement: Statement) -> Result<()>{
        use statement::Visitor;
        match statement{
            Statement::Expression(expr) => self.visit_expression(expr),
            Statement::Print(expr) => self.visit_print(expr),
        }?;
        Ok(())
    }

}


impl expression::Visitor<LoxObject> for Interpreter {
    fn visit_binary(&self, expr: &Binary) -> Result<LoxObject> {
        use BinaryOperator::{
            BANGEQUAL, EQUALEQUAL, GREATER, GREATEREQUAL, LESS, LESSEQUAL, MINUS, PLUS, SLASH, STAR,
        };
        use LoxObject::Bool;

        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        // can_compare does the typecheck so that we throw invalidOperand when comparing instead of
        // returning false
        let can_compare = left.partial_cmp(&right).is_some();
        // worst line of code ever written

        match expr.operator {
            PLUS => left + right,
            MINUS => left - right,
            STAR => left * right,
            SLASH => left / right,
            GREATER if can_compare => Ok(Bool(left > right)),
            GREATEREQUAL if can_compare => Ok(Bool(left >= right)),
            LESS if can_compare => Ok(Bool(left < right)),
            LESSEQUAL if can_compare => Ok(Bool(left <= right)),
            EQUALEQUAL if can_compare => Ok(Bool(left == right)),
            BANGEQUAL if can_compare => Ok(Bool(left != right)),
            _ => Err(RuntimeError::InvalidOperand),
        }
    }

    fn visit_grouping(&self, expr: &Grouping) -> Result<LoxObject> {
        let expression = *expr.expression.clone();
        self.evaluate(&expression)
    }
    fn visit_literal(&self, expr: &Literal) -> Result<LoxObject> {
        // could remove the clone soon
        Ok(expr.value.clone())
    }
    fn visit_unary(&self, expr: &Unary) -> Result<LoxObject> {
        let right = self.evaluate(&expr.right)?;
        match expr.operator {
            UnaryOperator::MINUS => -right,
            UnaryOperator::BANG => !right,
        }
    }
}


