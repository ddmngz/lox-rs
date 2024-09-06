use super::lox_object::LoxObject;
use crate::token::SmartString;
use strum_macros::Display;

pub type Result<T> = std::result::Result<T, crate::interpreter::RuntimeError>;
use crate::interpreter::RuntimeError;

pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Grouping(Box<Expression>),
    Literal(LoxObject),
    Unary {
        operator: UnaryOperator,
        inner: Box<Expression>,
    },
    Variable(SmartString),
}

impl From<f64> for Expression {
    fn from(float: f64) -> Self {
        Self::Literal(LoxObject::Float(float))
    }
}

impl From<bool> for Expression {
    fn from(boolean: bool) -> Self {
        Self::Literal(LoxObject::Bool(boolean))
    }
}

impl From<SmartString> for Expression {
    fn from(string: SmartString) -> Self {
        Self::Literal(LoxObject::String(string))
    }
}

impl Expression {
    pub fn evaluate(self) -> Result<LoxObject> {
        match self {
            Self::Binary {
                left,
                operator,
                right,
            } => Self::handle_binary(left, operator, right),
            Self::Grouping(inner) => inner.evaluate(),
            Self::Literal(inner) => Ok(inner),
            Self::Unary { operator, inner } => Self::handle_unary(operator, inner),
            Self::Variable(name) => todo!(),
        }
    }

    pub fn nil() -> Self {
        Self::Literal(LoxObject::Nil)
    }

    fn handle_binary(
        left: Box<Self>,
        operator: BinaryOperator,
        right: Box<Self>,
    ) -> Result<LoxObject> {
        use BinaryOperator::{
            BANGEQUAL, EQUALEQUAL, GREATER, GREATEREQUAL, LESS, LESSEQUAL, MINUS, PLUS, SLASH, STAR,
        };
        use LoxObject::Bool;

        let left = left.evaluate()?;
        let right = right.evaluate()?;

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

    fn handle_unary(operator: UnaryOperator, inner: Box<Self>) -> Result<LoxObject> {
        let inner = inner.evaluate()?;
        match operator {
            UnaryOperator::BANG => !inner,
            UnaryOperator::MINUS => -inner,
        }
    }
}

#[derive(Clone, Display, Debug)]
pub enum UnaryOperator {
    #[strum(serialize = "!")]
    BANG,
    #[strum(serialize = "-")]
    MINUS,
}

#[derive(Clone, Display, Debug)]
pub enum BinaryOperator {
    #[strum(serialize = "==")]
    EQUALEQUAL,
    #[strum(serialize = "!=")]
    BANGEQUAL,
    #[strum(serialize = ">")]
    GREATER,
    #[strum(serialize = ">=")]
    GREATEREQUAL,
    #[strum(serialize = "<")]
    LESS,
    #[strum(serialize = "<=")]
    LESSEQUAL,
    #[strum(serialize = "+")]
    PLUS,
    #[strum(serialize = "-")]
    MINUS,
    #[strum(serialize = "*")]
    STAR,
    #[strum(serialize = "/")]
    SLASH,
}

use crate::token::Token;
impl BinaryOperator {
    // trying really hard to prefer duplication to the wrong abstraction here
    pub fn from_token(token: Token) -> Option<Self> {
        match token {
            Token::EQUALEQUAL => Some(Self::EQUALEQUAL),
            Token::BANGEQUAL => Some(Self::BANGEQUAL),
            Token::GREATER => Some(Self::GREATER),
            Token::GREATEREQUAL => Some(Self::GREATEREQUAL),
            Token::LESS => Some(Self::LESS),
            Token::LESSEQUAL => Some(Self::LESSEQUAL),
            Token::PLUS => Some(Self::PLUS),
            Token::MINUS => Some(Self::MINUS),
            Token::STAR => Some(Self::STAR),
            Token::SLASH => Some(Self::SLASH),
            _ => None,
        }
    }
}
