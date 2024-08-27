use strum_macros::Display;
use crate::interpreter::RuntimeError;
use std::ops;
use std::cmp;
pub type Result<T> = std::result::Result<T, RuntimeError>;
use beef::lean::Cow;

#[derive(Clone, Debug, Display)]
pub enum LoxObject<'source> {
    #[strum(serialize = "{0}")]
    Float(f64),
    #[strum(serialize = "{0}")]
    String(Cow<'source,str>),
    #[strum(serialize = "{0}")]
    Bool(bool),
    #[strum(serialize = "nil")]
    Nil,
}

// logic for evaluating is handled through trait implementations, returning Error for invalid type
// conversions

impl ops::Neg for LoxObject<'_> {
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

impl ops::Not for LoxObject<'_> {
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

impl<'source> ops::Add for LoxObject<'source> {
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

impl ops::Sub for LoxObject<'_> {
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

impl ops::Mul for LoxObject<'_>{
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

impl ops::Div for LoxObject<'_> {
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

impl cmp::PartialEq for LoxObject<'_> {
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

impl cmp::PartialOrd for LoxObject<'_>{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        use LoxObject::Float;
        if let (Float(left), Float(right)) = (self, other) {
            left.partial_cmp(right)
        } else {
            None
        }
    }
}
