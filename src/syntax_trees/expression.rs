use super::lox_object::LoxObject;
use crate::token::SmartString;
use strum_macros::Display;

pub type Result<T> = std::result::Result<T, crate::interpreter::RuntimeError>;

#[derive(Clone)]
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
    Assign{
        name:SmartString,
        value: Box<Expression>,
    }
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


    pub fn nil() -> Self {
        Self::Literal(LoxObject::Nil)
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
