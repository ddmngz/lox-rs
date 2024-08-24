use super::parser::ast::expression::{self, *};
use super::parser::ast::statement;
use crate::error::LoxRuntimeError;

use std::{cmp, ops};
pub type LoxObject = LiteralValue;
pub struct Interpreter {}

pub type Result<T> = std::result::Result<T, LoxRuntimeError>;

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<LoxObject> {
        walk_expr(self, expr)
    }
    pub fn interpret(self, expr: &Expr) -> Result<()> {
        let value = self.evaluate(expr)?;
        println!("{}", value);
        Ok(())
    }
}

impl statement::Visitor<LoxObject> for Interpreter {
    fn visit_expression(expression: Expr) -> Result<LoxObject> {
        todo!()
    }

    fn visit_print(expression: Expr) -> Result<LoxObject> {
        todo!()
    }
}

impl expression::Visitor<LoxObject> for Interpreter {
    fn visit_binary(&self, expr: &Binary) -> Result<LoxObject> {
        use BinaryOperator::{
            BANGEQUAL, EQUALEQUAL, GREATER, GREATEREQUAL, LESS, LESSEQUAL, MINUS, PLUS, SLASH, STAR,
        };
        use LiteralValue::Bool;

        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        // can_compare does the typecheck so that we throw invalidOperand when comparing instead of
        // returning false
        let can_compare = left.partial_cmp(&right).is_some();

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
            _ => Err(LoxRuntimeError::InvalidOperand),
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

// logic for evaluating is handled through trait implementations, returning Error for invalid type
// conversions

impl std::ops::Neg for LoxObject {
    type Output = Result<LoxObject>;
    fn neg(self) -> Self::Output {
        use LiteralValue::Float;
        if let Float(value) = self {
            Ok(Float(-value))
        } else {
            Err(LoxRuntimeError::InvalidOperand)
        }
    }
}

impl std::ops::Not for LoxObject {
    type Output = Result<LoxObject>;
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
    type Output = Result<LoxObject>;
    fn add(self, other: Self) -> Self::Output {
        use LiteralValue::*;
        match (self, other) {
            (Float(left), Float(right)) => Ok(Float(left + right)),
            (String(left), String(right)) => Ok(String(format!("{left}{right}").into())),
            _ => Err(LoxRuntimeError::InvalidOperand),
        }
    }
}

impl ops::Sub for LoxObject {
    type Output = Result<LoxObject>;
    fn sub(self, other: Self) -> Self::Output {
        use LiteralValue::Float;
        if let (Float(left), Float(right)) = (self, other) {
            Ok(Float(left - right))
        } else {
            Err(LoxRuntimeError::InvalidOperand)
        }
    }
}

impl ops::Mul for LoxObject {
    type Output = Result<LoxObject>;
    fn mul(self, other: Self) -> Self::Output {
        use LiteralValue::Float;
        if let (Float(left), Float(right)) = (self, other) {
            Ok(Float(left * right))
        } else {
            Err(LoxRuntimeError::InvalidOperand)
        }
    }
}

impl ops::Div for LoxObject {
    type Output = Result<LoxObject>;
    fn div(self, other: Self) -> Self::Output {
        use LiteralValue::Float;
        if let (Float(left), Float(right)) = (self, other) {
            Ok(Float(left / right))
        } else {
            Err(LoxRuntimeError::InvalidOperand)
        }
    }
}

impl cmp::PartialEq for LoxObject {
    fn eq(&self, other: &Self) -> bool {
        use LiteralValue::{Bool, Float, Nil, String};
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
        use LiteralValue::Float;
        if let (Float(left), Float(right)) = (self, other) {
            left.partial_cmp(right)
        } else {
            None
        }
    }
}
