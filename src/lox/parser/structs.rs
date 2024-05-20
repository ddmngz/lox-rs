use super::super::Token;

pub trait Visitor<T> {
    fn visit_binary(&self, expr: &Binary) -> T;
    fn visit_grouping(&self, expr: &Grouping) -> T;
    fn visit_literal(&self, expr: &Literal) -> T;
    fn visit_unary(&self, expr: &Unary) -> T;
}

pub enum Expr<'a> {
    Binary(Binary<'a>),
    Grouping(Grouping<'a>),
    Literal(Literal),
    Unary(Unary<'a>),
}

pub fn walk_expr<T>(visitor: &mut dyn Visitor<T>, e: &Expr) -> T {
    match e {
        Expr::Binary(Binary) => visitor.visit_binary(&Binary),
        Expr::Grouping(Grouping) => visitor.visit_grouping(&Grouping),
        Expr::Literal(Literal) => visitor.visit_literal(&Literal),
        Expr::Unary(Unary) => visitor.visit_unary(&Unary),
    }
}

pub struct Binary<'a> {
    pub left: Box<Expr<'a>>,
    pub operator: Token<'a>,
    pub right: Box<Expr<'a>>,
}

pub struct Grouping<'a> {
    pub expression: Box<Expr<'a>>,
}

pub struct Literal {
    pub value: Option<LiteralEnum>,
}

pub enum LiteralEnum {
    Float(f64),
    String(String),
}

pub struct Unary<'a> {
    pub operator: Token<'a>,
    pub right: Box<Expr<'a>>,
}

/*
 * Expr trait just has accept()
 * ExpressionVisitor trait has visit_<each_expr>
 *
 */
