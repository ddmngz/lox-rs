pub mod error;

use crate::syntax_trees::lox_object::LoxObject;
use crate::syntax_trees::statement::Function;
use crate::syntax_trees::statement::Statement;
use crate::token::SmartString;
pub use error::RuntimeError;

use crate::syntax_trees::expression::BinaryOperator;
use crate::syntax_trees::expression::Expression;
use crate::syntax_trees::expression::LogicalOperator;
use crate::syntax_trees::expression::UnaryOperator;
pub use environment::Environment;
pub use environment::Global;

pub type Result<T> = std::result::Result<T, RuntimeError>;

/*
 *
 *
 *
 */

pub fn interpret(statements: Vec<Statement>, env: &mut Environment) -> Result<()> {
    for statement in statements {
        execute(statement, env)?;
    }

    Ok(())
}

fn execute(statement: Statement, environment: &mut Environment) -> Result<Option<LoxObject>> {
    match statement {
        Statement::Expression(statement) => {
            evaluate(statement, environment)?;
            Ok(None)
        }
        Statement::Print(statement) => {
            println!("{}", evaluate(statement, environment)?);
            Ok(None)
        }
        Statement::Var { name, initializer } => {
            let initial_value = if let Some(expression) = initializer {
                Some(evaluate(expression, environment)?)
            } else {
                None
            };
            environment.define(&name, initial_value);
            Ok(None)
        }
        Statement::If {
            condition,
            then,
            else_case,
        } => {
            if evaluate(condition, environment)?.truthy() {
                execute(*then, environment)
            } else if let Some(case) = else_case {
                execute(*case, environment)
            } else {
                Ok(None)
            }
        }
        Statement::While { condition, body } => {
            while evaluate(condition.clone(), environment)?.truthy() {
                if let Some(return_value) = execute(*body.clone(), environment)? {
                    return Ok(Some(return_value));
                }
            }
            Ok(None)
        }
        Statement::Block(statements) => {
            environment.add_scope();
            let ret_val = execute_block(statements, environment)?;
            environment.remove_scope();
            Ok(ret_val)
        }
        Statement::Function(function) => new_function(function, environment),

        Statement::Return { value, .. } => {
            if let Some(value) = value {
                let value = evaluate(value, environment)?;
                Ok(Some(value))
            } else {
                Ok(Some(LoxObject::Nil))
            }
        }
    }
}

fn new_function(function: Function, env: &mut Environment) -> Result<Option<LoxObject>> {
    let name = function.name.to_string();
    let function_object = if env.global() {
        LoxObject::Function(function)
    } else {
        LoxObject::Closure {
            declaration: function,
            env: env.as_closure(),
        }
    };
    env.define(&name, Some(function_object));
    Ok(None)
}

pub fn evaluate(expression: Expression, environment: &mut Environment) -> Result<LoxObject> {
    match expression {
        Expression::Binary {
            left,
            operator,
            right,
        } => handle_binary(*left, operator, *right, environment),
        Expression::Grouping(inner) => evaluate(*inner, environment),
        Expression::Literal(inner) => Ok(inner),
        Expression::Unary { operator, inner } => handle_unary(operator, *inner, environment),
        Expression::Variable { name, line } => handle_variable(&name, line, environment),
        Expression::Assign { name, value } => {
            let value = evaluate(*value, environment)?;
            environment.assign(&name, value.clone())?;
            Ok(value)
        }
        Expression::Logical {
            left,
            right,
            operator,
        } => {
            let left = evaluate(*left, environment)?;
            if operator == LogicalOperator::OR {
                if left.truthy() {
                    return Ok(left);
                };
            } else if !left.truthy() {
                return Ok(left);
            }
            evaluate(*right, environment)
        }
        Expression::Call {
            callee,
            paren,
            args,
        } => {
            let callable = evaluate(*callee, environment)?;
            let mut evaluated_args = Vec::new();
            for arg in args {
                evaluated_args.push(evaluate(arg, environment)?);
            }
            call(callable, paren.line, evaluated_args, environment)
        }
    }
}

fn call(
    callable: LoxObject,
    line: u32,
    args: Vec<LoxObject>,
    env: &mut Environment,
) -> Result<LoxObject> {
    match callable {
        LoxObject::Function(function) => {
            /*
             *  TODO: If global scope then use FunctionScope, otherwise use existing scoping rules
             *  :3
             *
             */
            check_arity(function.params.len(), args.len(), line)?;
            env.add_scope();
            //let mut function_env = env.function_environment();
            for (param, arg) in function.params.iter().zip(args) {
                let smartstr: SmartString = param.clone().into();
                let param: String = smartstr.into();
                env.define(&param, Some(arg))
            }
            let return_value = execute_block(function.body.clone(), env)?;
            env.remove_scope();
            match return_value {
                Some(value) => Ok(value),
                None => Ok(LoxObject::Nil),
            }
        }
        LoxObject::Closure { declaration, env } => {
            check_arity(declaration.params.len(), args.len(), line)?;

            let mut closure_env = env.clone();
            closure_env.add_scope();

            for (param, arg) in declaration.params.iter().zip(args) {
                let smartstr: SmartString = param.clone().into();
                let param: String = smartstr.into();
                closure_env.define(&param, Some(arg))
            }

            let return_value = execute_block(declaration.body.clone(), &mut closure_env)?;
            match return_value {
                Some(value) => Ok(value),
                None => Ok(LoxObject::Nil),
            }
        }

        _ => Err(error(RuntimeError::NotCallable, Some(line))),
    }
}

fn check_arity(expected: usize, got: usize, line: u32) -> Result<()> {
    if expected == got {
        Ok(())
    } else {
        Err(error(RuntimeError::Arity { expected, got }, Some(line)))
    }
}

fn execute_block(
    statements: Vec<Statement>,
    environment: &mut Environment,
) -> Result<Option<LoxObject>> {
    for statement in statements {
        if let Some(return_value) = execute(statement, environment)? {
            return Ok(Some(return_value));
        }
    }
    Ok(None)
}

fn handle_variable(key: &str, line: u32, environment: &mut Environment) -> Result<LoxObject> {
    match environment.get(key) {
        Ok(None) => Ok(LoxObject::Nil),
        Ok(Some(object)) => Ok(object.clone()),
        Err(e) => Err(error(e, Some(line))),
    }
}

fn handle_binary(
    left: Expression,
    operator: BinaryOperator,
    right: Expression,
    environment: &mut Environment,
) -> Result<LoxObject> {
    use BinaryOperator::{
        BANGEQUAL, EQUALEQUAL, GREATER, GREATEREQUAL, LESS, LESSEQUAL, MINUS, PLUS, SLASH, STAR,
    };
    use LoxObject::Bool;

    let left = evaluate(left, environment)?;
    let right = evaluate(right, environment)?;

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

fn handle_unary(
    operator: UnaryOperator,
    inner: Expression,
    environment: &mut Environment,
) -> Result<LoxObject> {
    let inner = evaluate(inner, environment)?;
    match operator {
        UnaryOperator::BANG => !inner,
        UnaryOperator::MINUS => -inner,
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

pub mod environment {
    use super::LoxObject;
    use super::RuntimeError;
    use crate::token::SmartString;
    use std::collections::HashMap;
    use std::fmt;

    use super::Result;
    type Env = HashMap<SmartString, Option<LoxObject>>;

    pub trait Global: fmt::Debug {
        fn get(&self, key: &str) -> Option<&Option<LoxObject>>;

        fn insert(&mut self, key: &str, value: Option<LoxObject>) -> Option<()>;
        fn define(&mut self, key: &str, value: Option<LoxObject>) {
            self.insert(key, value);
        }
        fn assign(&mut self, key: &str, value: LoxObject) -> Result<()> {
            self.insert(key, Some(value))
                .ok_or(RuntimeError::Undefined(key.into()))
        }

        fn as_env(&self) -> &Env;
        fn as_mut_env(&mut self) -> &mut Env;
    }

    #[derive(Clone, Debug, Default)]
    pub struct Environment {
        inner: Option<EnvironmentInner>,
        global: Env,
    }

    impl fmt::Display for Environment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.global)?;
            if let Some(inner) = &self.inner {
                write!(f, "|{}", inner)
            } else {
                Ok(())
            }
        }
    }

    #[derive(Clone, Debug)]
    struct EnvironmentInner {
        env: Env,
        parent: Option<Box<EnvironmentInner>>,
    }

    impl fmt::Display for EnvironmentInner {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if let Some(parent) = &self.parent {
                write!(f, "|{parent}|\n|{env:?}\n", env = self.env)
            } else {
                writeln!(f, "{env:?}", env = self.env)
            }
        }
    }

    impl Global for Env {
        fn get(&self, key: &str) -> Option<&Option<LoxObject>> {
            HashMap::get(self, key)
        }

        fn insert(&mut self, key: &str, value: Option<LoxObject>) -> Option<()> {
            self.insert(key.into(), value)?;
            Some(())
        }

        fn as_env(&self) -> &Self {
            self
        }

        fn as_mut_env(&mut self) -> &mut Self {
            self
        }
    }

    impl Global for &mut Env {
        fn get(&self, key: &str) -> Option<&Option<LoxObject>> {
            HashMap::get(self, key)
        }

        fn insert(&mut self, key: &str, value: Option<LoxObject>) -> Option<()> {
            HashMap::insert(self, key.into(), value)?;
            Some(())
        }

        fn as_env(&self) -> &Env {
            self
        }

        fn as_mut_env(&mut self) -> &mut Env {
            self
        }
    }

    impl Environment {
        pub const fn global(&self) -> bool {
            self.inner.is_none()
        }

        pub fn get(&self, key: &str) -> Result<&Option<LoxObject>> {
            let Some(ref inner) = self.inner else {
                return self.get_global(key);
            };

            if let Some(value) = inner.get(key) {
                Ok(value)
            } else {
                self.get_global(key)
            }
        }

        pub fn get_global(&self, key: &str) -> Result<&Option<LoxObject>> {
            match self.global.get(key) {
                Some(value) => Ok(value),
                None => Err(RuntimeError::Undefined(key.into())),
            }
        }

        pub fn define(&mut self, key: &str, value: Option<LoxObject>) {
            if let Some(ref mut inner) = self.inner {
                inner.define(key, value);
            } else {
                self.global.define(key, value);
            }
        }

        pub fn assign(&mut self, key: &str, value: LoxObject) -> Result<()> {
            if let Some(ref mut inner) = self.inner {
                inner.assign(key, value)?;
            } else {
                self.global.assign(key, value)?;
            }
            Ok(())
        }

        pub fn add_scope(&mut self) {
            if let Some(ref mut inner) = self.inner {
                inner.add_scope();
            } else {
                self.inner = Some(EnvironmentInner::new());
            }
        }

        pub fn remove_scope(&mut self) {
            let Some(inner) = self.inner.clone() else {
                panic!("attempted to remove global scope");
            };
            self.inner = inner.parent.map(|env| *env)
        }
        pub fn as_closure(&self) -> Self {
            self.clone()
        }
    }

    impl EnvironmentInner {
        fn new() -> Self {
            Self {
                env: Env::new(),
                parent: None,
            }
        }

        pub fn get_mut(&mut self, key: &str) -> Option<&mut Option<LoxObject>> {
            if let Some(value) = self.env.get_mut(key) {
                Some(value)
            } else if let Some(parent) = &mut self.parent {
                parent.get_mut(key)
            } else {
                None
            }
        }

        fn get(&self, key: &str) -> Option<&Option<LoxObject>> {
            if let Some(value) = self.env.get(key) {
                Some(value)
            } else if let Some(parent) = &self.parent {
                parent.get(key)
            } else {
                None
            }
        }

        pub fn assign(&mut self, key: &str, value: LoxObject) -> Result<()> {
            if let Some(variable) = self.get_mut(key) {
                *variable = Some(value);
                Ok(())
            } else {
                Err(RuntimeError::Undefined(key.into()))
            }
        }

        pub fn define(&mut self, key: &str, value: Option<LoxObject>) {
            self.env.insert(key.into(), value);
        }

        fn add_scope(&mut self) {
            let env = Env::new();
            let parent = Box::new(self.clone());
            self.env = env;
            self.parent = Some(parent);
        }
    }
}
