use super::Expression;
use super::LoxObject;
use super::Result;
use super::Visitor;
use crate::interpreter::RuntimeError;
use crate::scanner::Token;
use strum_macros::Display;
use super::Expr;

pub struct Binary {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}

impl Binary{
    pub fn new(left:Expression, operator:Operator, right:Expression) -> Self{
        Self{
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}


impl Visitor<String> for Binary {
    fn accept(&self) -> String {
        let left = self.left.print();
        let right = self.right.print();
        format!("BINARY OPERATOR({left}{operator}{right})", operator = self.operator)
    }
}

impl Visitor<Result<LoxObject>> for Binary {
    fn accept(&self) -> Result<LoxObject> {
        use LoxObject::Bool;
        use Operator::{
            BANGEQUAL, EQUALEQUAL, GREATER, GREATEREQUAL, LESS, LESSEQUAL, MINUS, PLUS, SLASH, STAR,
        };

        let left = self.left.evaluate()?;
        let right = self.right.evaluate()?;

        // can_compare does the typecheck so that we throw invalidOperand when comparing instead of
        // returning false
        let can_compare = left.partial_cmp(&right).is_some();
        // worst line of code ever written

        match self.operator {
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
}

#[derive(Clone, Display, Debug)]
pub enum Operator {
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

impl Operator {
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
