use crate::interpreter::RuntimeError;
use std::cmp;
use std::ops;
use strum_macros::Display;
pub type Result<T> = std::result::Result<T, RuntimeError>;
use super::lox_callable;
use super::lox_callable::Callable;
use super::lox_callable::MaybeCallable;
use super::statement::Function;
use crate::token::SmartString;

#[derive(Clone, Debug, Display)]
pub enum LoxObject {
    #[strum(serialize = "{0}")]
    Float(f64),
    #[strum(serialize = "{0}")]
    String(SmartString),
    #[strum(serialize = "{0}")]
    Bool(bool),
    #[strum(serialize = "nil")]
    Nil,
    #[strum(serialize = "{0}")]
    VarName(SmartString),
    #[strum(serialize = "{0}")]
    LoxFunction(Function),
}

/*
#[derive(Debug, Clone)]
struct LoxFunction {
    declaration: super::statement::Statement,

}

impl std::fmt::Display for LoxFunction {}
*/

impl MaybeCallable for LoxObject {
    fn try_call(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        args: Vec<LoxObject>,
    ) -> crate::syntax_trees::lox_callable::Result<LoxObject> {
        if let Self::LoxFunction(f) = self {
            Ok(f.call(interpreter, args))
        } else {
            Err(lox_callable::NotCallable::Unit)
        }
    }

    fn try_arity(&self) -> lox_callable::Result<usize> {
        if let Self::LoxFunction(f) = self {
            Ok(f.arity())
        } else {
            Err(lox_callable::NotCallable::Unit)
        }
    }
}

impl LoxObject {
    pub fn truthy(&self) -> bool {
        !matches!(self, Self::Bool(false) | Self::Nil)
    }
}

// logic for evaluating is handled through trait implementations, returning Error for invalid type
// conversions

impl ops::Neg for LoxObject {
    type Output = Result<Self>;
    fn neg(self) -> Self::Output {
        use LoxObject::Float;
        if let Float(value) = self {
            Ok(Float(-value))
        } else {
            Err(RuntimeError::InvalidOperand)
        }
    }
}

impl ops::Not for LoxObject {
    type Output = Result<Self>;
    // Lox semantics are that Nil is false, so !Nil = true for some reason even though that's kind
    // of evil
    fn not(self) -> Self::Output {
        Ok(LoxObject::Bool(match self {
            LoxObject::Bool(b) => !b,
            LoxObject::Nil => true,
            _ => false,
        }))
    }
}

impl ops::Add for LoxObject {
    type Output = Result<Self>;
    fn add(self, other: Self) -> Self::Output {
        use LoxObject::*;
        match (self, other) {
            (Float(left), Float(right)) => Ok(Float(left + right)),
            (String(left), String(right)) => Ok(String(format!("{left}{right}").into())),
            _ => Err(RuntimeError::InvalidOperand),
        }
    }
}

impl ops::Sub for LoxObject {
    type Output = Result<Self>;
    fn sub(self, other: Self) -> Self::Output {
        use LoxObject::Float;
        if let (Float(left), Float(right)) = (self, other) {
            Ok(Float(left - right))
        } else {
            Err(RuntimeError::InvalidOperand)
        }
    }
}

impl ops::Mul for LoxObject {
    type Output = Result<Self>;
    fn mul(self, other: Self) -> Self::Output {
        use LoxObject::Float;
        if let (Float(left), Float(right)) = (self, other) {
            Ok(Float(left * right))
        } else {
            Err(RuntimeError::InvalidOperand)
        }
    }
}

impl ops::Div for LoxObject {
    type Output = Result<Self>;
    fn div(self, other: Self) -> Self::Output {
        use LoxObject::Float;
        if let (Float(left), Float(right)) = (self, other) {
            Ok(Float(left / right))
        } else {
            Err(RuntimeError::InvalidOperand)
        }
    }
}

impl cmp::PartialEq for LoxObject {
    fn eq(&self, other: &Self) -> bool {
        use LoxObject::{Bool, Float, Nil, String};
        match (self, other) {
            (Nil, Nil) => true,
            (Bool(a), Bool(b)) => a == b,
            (String(a), String(b)) => a == b,
            (Float(a), Float(b)) => a == b,
            (_, _) => false,
        }
    }
}

impl cmp::PartialOrd for LoxObject {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        use LoxObject::Float;
        if let (Float(left), Float(right)) = (self, other) {
            left.partial_cmp(right)
        } else {
            None
        }
    }
}
