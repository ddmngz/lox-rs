use crate::scanner::Token;
use super::lox_object::LoxObject;
use byteyarn::ByteYarn;
use strum_macros::Display;

pub type Result<T> = std::result::Result<T, crate::interpreter::RuntimeError>;

pub trait Visitor<T> {
    fn visit_binary(&self, expr: &Binary) -> Result<T>;
    fn visit_grouping(&self, expr: &Grouping) -> Result<T>;
    fn visit_literal(&self, expr: &Literal) -> Result<T>;
    fn visit_unary(&self, expr: &Unary) -> Result<T>;
}



#[derive(Clone, Debug)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}


pub fn walk_expr<T>(visitor: &dyn Visitor<T>, e: &Expr) -> Result<T> {
    match e {
        Expr::Binary(binary) => visitor.visit_binary(binary),
        Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
        Expr::Literal(literal) => visitor.visit_literal(literal),
        Expr::Unary(unary) => visitor.visit_unary(unary),
    }
}

#[derive(Clone, Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: BinaryOperator,
    pub right: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct Literal {
    pub value: LoxObject,
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

#[derive(Clone, Debug)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Expr, operator: BinaryOperator, right: Expr) -> Expr {
        Expr::Binary(Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}

impl Grouping {
    pub fn new(expression: Expr) -> Expr {
        Expr::Grouping(Self {
            expression: Box::new(expression),
        })
    }
}

impl Literal {
    pub fn float(value: f64) -> Expr {
        Expr::Literal(Self {
            value: LoxObject::Float(value),
        })
    }

    pub fn string(value: ByteYarn) -> Expr {
        Expr::Literal(Self {
            value: LoxObject::String(value),
        })
    }

    pub fn nil() -> Expr {
        Expr::Literal(Self {
            value: LoxObject::Nil,
        })
    }
    pub fn r#true() -> Expr {
        Expr::Literal(Self {
            value: LoxObject::Bool(true),
        })
    }

    pub fn r#false() -> Expr {
        Expr::Literal(Self {
            value: LoxObject::Bool(false),
        })
    }
}

impl Unary {
    pub fn new(operator: UnaryOperator, right: Expr) -> Expr {
        Expr::Unary(Self {
            operator,
            right: Box::new(right),
        })
    }
}
