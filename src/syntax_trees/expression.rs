use crate::scanner::Token;
use super::lox_object::LoxObject;
use strum_macros::Display;
use beef::lean::Cow;

pub type Result<T> = std::result::Result<T, crate::interpreter::RuntimeError>;

pub trait Expr<'source>{
    fn evaluate(&self) -> Result<LoxObject<'source>>;
    fn print(&self) -> Result<String>;
}



#[derive(Clone, Debug)]
pub struct Binary<'source> {
    pub left: Box<dyn Expr<'source>>,
    pub operator: BinaryOperator,
    pub right: Box<dyn Expr<'source>>,
}

#[derive(Clone, Debug)]
pub struct Grouping<'source> {
    pub expression: Box<dyn Expr<'source>>,
}

#[derive(Clone, Debug)]
pub struct Unary<'source> {
    pub operator: UnaryOperator,
    pub right: Box<dyn Expr<'source>>,
}

#[derive(Clone, Debug)]
pub struct Literal<'source> {
    pub value: LoxObject<'source>,
}

impl<'source> Expr<'source> for Literal<'source>{
    fn evaluate(&self) -> Result<LoxObject<'source>>{
        Ok(self.value)
    }

    fn print(&self) -> Result<String>{
        Ok(format!("{}",self.value))
    }
}

impl<'source> Expr<'source> for Unary<'source>{
    fn evaluate(&self) -> Result<LoxObject<'source>>{
        use UnaryOperator::{BANG,MINUS};
        let right = self.right.evaluate();
        match self.operator{
            BANG => !right,
            MINUS => -right,
        }
        Ok(self.value)
    }

    fn print(&self) -> Result<String>{
        let right = self.right.print();
        Ok(format!("{}{}",self.operator,right))
    }
}


// TODO deduplicate from token_type
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


impl BinaryOperator {
    // trying really hard to prefer duplication to the wrong abstraction here
    pub fn from_token(token: Token) -> Option<Self> {
        match token{
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

#[derive(Clone, Display, Debug)]
pub enum UnaryOperator {
    #[strum(serialize = "!")]
    BANG,
    #[strum(serialize = "-")]
    MINUS,
}



impl Binary<'_> {
    pub fn new(left: dyn Expr, operator: BinaryOperator, right: dyn Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

impl Grouping<'_> {
    pub fn new(expression: dyn Expr) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

impl<'source> Literal<'source>{
    pub fn string(value: &'source str) -> Self {
        Self {
            value: LoxObject::String(Cow::borrowed(value)),
        }
    }

}


impl Literal<'_> {
    pub fn float(value: f64) -> Self {
        Expr::Literal(Self {
            value: LoxObject::Float(value),
        })
    }


    pub fn nil() -> Self{
        Self {
            value: LoxObject::Nil,
        }
    }

    pub fn r#true() -> Self {
        Self {
            value: LoxObject::Bool(true),
        }
    }

    pub fn r#false() -> Self {
        Self(Self {
            value: LoxObject::Bool(false),
        })
    }
}

impl Unary<'_> {
    pub fn new(operator: UnaryOperator, right: dyn Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}
