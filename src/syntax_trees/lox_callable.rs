use super::lox_object::LoxObject;
use crate::Interpreter;

pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<LoxObject>) -> CallableResult;
    fn arity(&self) -> usize;
}

pub type CallableResult = std::result::Result<LoxObject, crate::interpreter::error::RuntimeError>;
