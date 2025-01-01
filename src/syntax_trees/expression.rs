use super::lox_object::LoxObject;
use crate::scanner::ScannedToken;
use crate::token::SmartString;
use std::fmt;
use strum_macros::Display;

pub type Result<T> = std::result::Result<T, crate::interpreter::RuntimeError>;

#[derive(Clone, Debug)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Logical {
        left: Box<Expression>,
        operator: LogicalOperator,
        right: Box<Expression>,
    },
    Grouping(Box<Expression>),
    Literal(LoxObject),
    Call {
        callee: Box<Expression>,
        paren: ScannedToken,
        args: Vec<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        inner: Box<Expression>,
    },
    Variable {
        name: SmartString,
        line: u32,
    },
    Assign {
        name: SmartString,
        value: Box<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Binary {
                left,
                operator,
                right,
            } => write!(f, "({}{}{})", left, operator, right),
            Self::Logical {
                left,
                operator,
                right,
            } => write!(f, "({}{}{})", left, operator, right),
            Self::Grouping(e) => write!(f, "({})", e),
            Self::Literal(o) => write!(f, "{}", o),
            Self::Call { callee, args, .. } => {
                write!(f, "{callee}(")?;
                if let Some(last) = args.last() {
                    for arg in &args[..args.len() - 1] {
                        write!(f, "{arg},")?;
                    }
                    write!(f, "{last}")?;
                }
                write!(f, ")")
            }
            Self::Unary { operator, inner } => write!(f, "({operator}{inner})"),
            Self::Variable { name: var, .. } => write!(f, "({var})"),
            Self::Assign { name, value } => write!(f, "({name} = {value})"),
        }
    }
}

/*
struct Call_able {}

impl TryInto<Call_able> for Expression {}
*/

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

#[derive(Clone, Copy, Display, Debug)]
pub enum UnaryOperator {
    #[strum(serialize = "!")]
    BANG,
    #[strum(serialize = "-")]
    MINUS,
}

#[derive(Copy, Clone, Display, Debug)]
pub enum BinaryOperator {
    #[strum(serialize = "==")]
    EQUALEQUAL(u32),
    #[strum(serialize = "!=")]
    BANGEQUAL(u32),
    #[strum(serialize = ">")]
    GREATER(u32),
    #[strum(serialize = ">=")]
    GREATEREQUAL(u32),
    #[strum(serialize = "<")]
    LESS(u32),
    #[strum(serialize = "<=")]
    LESSEQUAL(u32),
    #[strum(serialize = "+")]
    PLUS(u32),
    #[strum(serialize = "-")]
    MINUS(u32),
    #[strum(serialize = "*")]
    STAR(u32),
    #[strum(serialize = "/")]
    SLASH(u32),
}

#[derive(Copy, Clone, Display, Debug, PartialEq, Eq)]
pub enum LogicalOperator {
    #[strum(serialize = "and")]
    AND,
    #[strum(serialize = "or")]
    OR,
}

impl TryFrom<Token> for LogicalOperator {
    type Error = ();
    fn try_from(token: Token) -> std::result::Result<Self, Self::Error> {
        match token {
            Token::AND => Ok(Self::AND),
            Token::OR => Ok(Self::OR),
            _ => Err(()),
        }
    }
}

use crate::token::Token;
impl BinaryOperator {
    // trying really hard to prefer duplication to the wrong abstraction here

    pub fn from_token(token: ScannedToken) -> Option<Self> {
        match token.type_ {
            Token::EQUALEQUAL => Some(Self::EQUALEQUAL(token.line)),
            Token::BANGEQUAL => Some(Self::BANGEQUAL(token.line)),
            Token::GREATER => Some(Self::GREATER(token.line)),
            Token::GREATEREQUAL => Some(Self::GREATEREQUAL(token.line)),
            Token::LESS => Some(Self::LESS(token.line)),
            Token::LESSEQUAL => Some(Self::LESSEQUAL(token.line)),
            Token::PLUS => Some(Self::PLUS(token.line)),
            Token::MINUS => Some(Self::MINUS(token.line)),
            Token::STAR => Some(Self::STAR(token.line)),
            Token::SLASH => Some(Self::SLASH(token.line)),
            _ => None,
        }
    }
}
