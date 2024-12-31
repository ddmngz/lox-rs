mod environment;
pub mod error;

use crate::syntax_trees::statement::Statement;
pub use error::RuntimeError;

use crate::syntax_trees::lox_object::LoxObject;

use crate::syntax_trees::expression::BinaryOperator;
use crate::syntax_trees::expression::Expression;
use crate::syntax_trees::expression::LogicalOperator;
use crate::syntax_trees::expression::UnaryOperator;
use environment::Environment;
use environment::EnvironmentTree;

#[derive(Default)]
pub struct Interpreter {
    environment: EnvironmentTree,
}
pub type Result<T> = std::result::Result<T, RuntimeError>;

pub fn interpret(statements: Vec<Statement>) -> Result<()> {
    let mut _interpreter = Interpreter::default();
    for statement_ in statements {
        _interpreter.execute(statement_)?;
    }

    Ok(())
}

impl Interpreter {
    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<()> {
        for statement_ in statements {
            eprintln!("{}", self.environment);
            self.execute(statement_)?;
        }
        Ok(())
    }

    fn execute(&mut self, statement: Statement) -> Result<Option<LoxObject>> {
        match statement {
            Statement::Expression(statement) => {
                self.evaluate(statement)?;
                Ok(None)
            }
            Statement::Print(statement) => {
                println!("{}", self.evaluate(statement)?);
                Ok(None)
            }
            Statement::Var { name, initializer } => {
                let initial_value = if let Some(expression) = initializer {
                    Some(self.evaluate(expression)?)
                } else {
                    None
                };
                self.environment.cur_mut().define(name, initial_value);
                Ok(None)
            }
            Statement::If {
                condition,
                then,
                else_case,
            } => {
                if self.evaluate(condition)?.truthy() {
                    self.execute(*then)
                } else if let Some(case) = else_case {
                    self.execute(*case)
                } else {
                    Ok(None)
                }
            }
            Statement::While { condition, body } => {
                while self.evaluate(condition.clone())?.truthy() {
                    if let Some(return_value) = self.execute(*body.clone())? {
                        return Ok(Some(return_value));
                    }
                }
                Ok(None)
            }
            Statement::Block(statements) => {
                eprintln!("block{{");
                self.environment.add_scope();
                let ret_val = self.execute_block(statements)?;
                self.environment.remove_scope();
                eprintln!("}}");
                Ok(ret_val)
            }
            Statement::Function(function) => {
                println!("Function Declaration! current env: {}", self.environment);
                let name = function.name.clone().into();
                let function_object = LoxObject::LoxFunction(function);
                self.environment
                    .cur_mut()
                    .define(name, Some(function_object));
                Ok(None)
            }

            Statement::Return { value, .. } => {
                if let Some(value) = value {
                    let value = self.evaluate(value)?;
                    Ok(Some(value))
                } else {
                    Ok(Some(LoxObject::Nil))
                }
            }
        }
    }

    fn execute_block(&mut self, statements: Vec<Statement>) -> Result<Option<LoxObject>> {
        for statement in statements {
            //eprintln!("{}", self.environment);
            if let Some(return_value) = self.execute(statement)? {
                return Ok(Some(return_value));
            }
        }
        Ok(None)
    }

    pub fn evaluate(&mut self, expression: Expression) -> Result<LoxObject> {
        //println!("evaluate {expression}");
        match expression {
            Expression::Binary {
                left,
                operator,
                right,
            } => self.handle_binary(*left, operator, *right),
            Expression::Grouping(inner) => self.evaluate(*inner),
            Expression::Literal(inner) => Ok(inner),
            Expression::Unary { operator, inner } => self.handle_unary(operator, *inner),
            Expression::Variable { name, line } => self.handle_variable(&name, line),
            Expression::Assign { name, value } => {
                let value = self.evaluate(*value)?;
                self.environment.cur_mut().assign(name, value.clone())?;
                Ok(value)
            }
            Expression::Logical {
                left,
                right,
                operator,
            } => {
                let left = self.evaluate(*left)?;
                if operator == LogicalOperator::OR {
                    if left.truthy() {
                        return Ok(left);
                    };
                } else if !left.truthy() {
                    return Ok(left);
                }
                self.evaluate(*right)
            }
            Expression::Call {
                callee,
                paren,
                args,
            } => {
                let callee = self.evaluate(*callee)?;
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.evaluate(arg)?);
                }

                let LoxObject::LoxFunction(callee) = callee else {
                    return Err(error(RuntimeError::NotCallable, Some(paren.line)));
                };

                let arity = callee.arity();
                if evaluated_args.len() == arity {
                    Ok(callee.call(self, evaluated_args)?)
                } else {
                    Err(error(
                        RuntimeError::Arity {
                            expected: arity,
                            got: evaluated_args.len(),
                        },
                        Some(paren.line),
                    ))
                }
            }
        }
    }

    fn handle_variable(&self, key: &str, line: u32) -> Result<LoxObject> {
        match self.environment.cur_leaf().get(key) {
            Ok(None) => Ok(LoxObject::Nil),
            Ok(Some(object)) => Ok(object.clone()),
            Err(e) => Err(error(e, Some(line))),
        }
    }

    fn handle_binary(
        &mut self,
        left: Expression,
        operator: BinaryOperator,
        right: Expression,
    ) -> Result<LoxObject> {
        use BinaryOperator::{
            BANGEQUAL, EQUALEQUAL, GREATER, GREATEREQUAL, LESS, LESSEQUAL, MINUS, PLUS, SLASH, STAR,
        };
        use LoxObject::Bool;

        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        // can_compare does the typecheck so that we throw invalidOperand when comparing instead of
        // returning false
        let can_compare = left.partial_cmp(&right).is_some();
        let line = match operator {
            PLUS(line) | MINUS(line) | STAR(line) | SLASH(line) | GREATER(line)
            | GREATEREQUAL(line) | LESS(line) | LESSEQUAL(line) | EQUALEQUAL(line)
            | BANGEQUAL(line) => line,
        };
        // worst line of code ever written
        let res = match operator {
            PLUS(_) => left + right,
            MINUS(_) => left - right,
            STAR(_) => left * right,
            SLASH(_) => left / right,
            GREATER(_) if can_compare => Ok(Bool(left > right)),
            GREATEREQUAL(_) if can_compare => Ok(Bool(left >= right)),
            LESS(_) if can_compare => Ok(Bool(left < right)),
            LESSEQUAL(_) if can_compare => Ok(Bool(left <= right)),
            EQUALEQUAL(_) if can_compare => Ok(Bool(left == right)),
            BANGEQUAL(_) if can_compare => Ok(Bool(left != right)),
            _ => Err(RuntimeError::InvalidOperand),
        };
        if let Err(e) = res {
            Err(error(e, Some(line)))
        } else {
            res
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

fn error(error: RuntimeError, line: Option<u32>) -> RuntimeError {
    match line {
        Some(line) => {
            println!("Error on line {}: {}", line, error)
        }
        None => {
            println!("Error at end of file: {}", error)
        }
    };
    error
}

use crate::syntax_trees::lox_callable;
use crate::syntax_trees::lox_callable::Callable;
use crate::syntax_trees::statement::Function;
impl Callable for Function {
    fn call(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        args: Vec<LoxObject>,
    ) -> lox_callable::CallableResult {
        eprintln!("function {}({:?}){{", self.name, args);
        interpreter.environment.add_function_scope();
        for (param, arg) in self.params.iter().zip(args) {
            interpreter
                .environment
                .cur_mut()
                .define(param.clone().into(), Some(arg))
        }
        eprintln!("{}", interpreter.environment);
        let return_value = interpreter.execute_block(self.body.clone())?;
        //eprintln!("returning {return_value:?}");
        interpreter.environment.remove_scope();
        eprintln!("}}");
        match return_value {
            Some(value) => Ok(value),
            None => Ok(LoxObject::Nil),
        }
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}
