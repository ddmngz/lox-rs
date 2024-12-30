use super::lox_object::LoxObject;
use crate::Interpreter;
use thiserror::Error;

pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<LoxObject>) -> LoxObject;
    fn arity(&self) -> usize;
}

pub type Result<T> = std::result::Result<T, NotCallable>;

pub trait MaybeCallable {
    fn try_call(&self, interpreter: &mut Interpreter, args: Vec<LoxObject>) -> Result<LoxObject>;
    fn try_arity(&self) -> Result<usize>;
}

impl<T: Callable> MaybeCallable for T {
    fn try_call(&self, interpreter: &mut Interpreter, args: Vec<LoxObject>) -> Result<LoxObject> {
        Ok(self.call(interpreter, args))
    }
    fn try_arity(&self) -> Result<usize> {
        Ok(self.arity())
    }
}

#[derive(Error, Debug, Clone)]
pub enum NotCallable {
    #[error("Can only call functions and classes.")]
    Unit,
}
