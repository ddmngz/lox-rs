mod error;
use crate::syntax_trees::statement::Statement;
pub use error::RuntimeError;

use crate::syntax_trees::lox_object::LoxObject;
use crate::token::SmartString;
use std::collections::HashMap;

use crate::syntax_trees::expression::Expression;
use crate::syntax_trees::expression::BinaryOperator;
use crate::syntax_trees::expression::UnaryOperator;


#[derive(Default)]
pub struct Interpreter {
    environment: HashMap<SmartString, Option<LoxObject>>,
}
pub type Result<T> = std::result::Result<T, RuntimeError>;

pub fn interpret(statements: Vec<Statement>) -> Result<()> {
    let mut _interpreter = Interpreter {
        environment: HashMap::new(),
    };
    for statement_ in statements {
        _interpreter.execute(statement_)?;
    }

    Ok(())
}


use std::collections::hash_map::Entry;
impl Interpreter {

    pub fn interpret(&mut self, statements:Vec<Statement>) -> Result<()>{
        for statement_ in statements {
            self.execute(statement_)?;
        }
        Ok(())
    }

    fn execute(&mut self, statement: Statement) -> Result<()> {
        match statement {
            Statement::Expression(statement) => {
                self.evaluate(statement)?;
                Ok(())
            }
            Statement::Print(statement) => {
                println!("{}", self.evaluate(statement)?);
                Ok(())
            }
            Statement::Var { name, initializer } => {
                let initial_value = if let Some(expression) = initializer {
                    Some(self.evaluate(expression)?)
                } else {
                    None
                };

                self.define(name, initial_value);
                Ok(())
            }
        }
    }

    fn get(&self, key: &str) -> Result<&Option<LoxObject>> {
        self.environment
            .get(key)
            .ok_or_else(|| RuntimeError::Undefined(SmartString::from(key)))
    }

    fn define(&mut self, key: SmartString, value: Option<LoxObject>) {
        self.environment.insert(key, value);
    }

    fn assign(&mut self, key: SmartString, value: LoxObject) -> Result<()>{
        if let Entry::Occupied(mut variable) = self.environment.entry(key.clone()){
            *variable.get_mut() = Some(value);
                Ok(())
        }else{
            Err(RuntimeError::Undefined(key))
        }
    }


    pub fn evaluate(&mut self, expression:Expression) -> Result<LoxObject> {
        match expression {
            Expression::Binary {
                left,
                operator,
                right,
            } => self.handle_binary(*left, operator, *right),
            Expression::Grouping(inner) => self.evaluate(*inner),
            Expression::Literal(inner) => Ok(inner),
            Expression::Unary { operator, inner } => self.handle_unary(operator, *inner),
            Expression::Variable(name) => self.handle_variable(&name),
            Expression::Assign {name, value} => {
                let value = self.evaluate(*value)?;
                self.assign(name, value.clone())?;
                Ok(value)
            },
        }
    }

    fn handle_variable(&self, key:&str) -> Result<LoxObject>{
        match self.get(key){
            Ok(None) => Ok(LoxObject::Nil),
            Ok(Some(object)) => Ok(object.clone()),
            Err(error) => Err(error),
        }
    }

    fn handle_binary(&mut self, left: Expression, operator: BinaryOperator, right: Expression) -> Result<LoxObject> {
        use BinaryOperator::{
            BANGEQUAL, EQUALEQUAL, GREATER, GREATEREQUAL, LESS, LESSEQUAL, MINUS, PLUS, SLASH, STAR,
        };
        use LoxObject::Bool;

        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        // can_compare does the typecheck so that we throw invalidOperand when comparing instead of
        // returning false
        let can_compare = left.partial_cmp(&right).is_some();
        // worst line of code ever written

        match operator {
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

    fn handle_unary(&mut self, operator: UnaryOperator, inner: Expression) -> Result<LoxObject> {
        let inner = self.evaluate(inner)?;
        match operator {
            UnaryOperator::BANG => !inner,
            UnaryOperator::MINUS => -inner,
        }
    }

}


