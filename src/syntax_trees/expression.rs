use super::lox_object::LoxObject;
use crate::scanner::ScannedToken;
use crate::syntax_trees::lox_callable::MaybeCallable;
use crate::token::SmartString;
use crate::Interpreter;
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
    Variable(SmartString),
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
            Self::Variable(var) => write!(f, "({var})"),
            Self::Assign { name, value } => write!(f, "({name} = {value})"),
        }
    }
}

impl MaybeCallable for Expression {
    fn try_call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<LoxObject>,
    ) -> crate::syntax_trees::lox_callable::Result<LoxObject> {
        todo!()
    }

    fn try_arity(&self) -> crate::syntax_trees::lox_callable::Result<usize> {
        todo!()
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

#[derive(Copy, Clone, Display, Debug, PartialEq, Eq)]
pub enum LogicalOperator {
    #[strum(serialize = "and")]
    AND,
    #[strum(serialize = "or")]
    OR,
}

impl Into<Token> for LogicalOperator {
    fn into(self) -> Token {
        match self {
            Self::AND => Token::AND,
            Self::OR => Token::OR,
        }
    }
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
