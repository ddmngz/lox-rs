mod error;
use crate::syntax_trees::statement::Statement;
pub use error::RuntimeError;

use crate::syntax_trees::lox_object::LoxObject;
use crate::token::SmartString;
use std::collections::HashMap;

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

impl Interpreter {
    fn execute(&mut self, statement: Statement) -> Result<()> {
        match statement {
            Statement::Expression(statement) => {
                statement.evaluate()?;
                Ok(())
            }
            Statement::Print(statement) => {
                println!("{}", statement.evaluate()?);
                Ok(())
            }
            Statement::Var { name, initializer } => {
                let initial_value = if let Some(expression) = initializer {
                    Some(expression.evaluate()?)
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
}
