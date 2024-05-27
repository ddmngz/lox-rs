use super::super::Token;

pub trait Visitor<T> {
    fn visit_binary(&self, expr: &Binary) -> T;
    fn visit_grouping(&self, expr: &Grouping) -> T;
    fn visit_literal(&self, expr: &Literal) -> T;
    fn visit_unary(&self, expr: &Unary) -> T;
}

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

pub fn walk_expr<T>(visitor: &mut dyn Visitor<T>, e: &Expr) -> T {
    match e {
        Expr::Binary(binary) => visitor.visit_binary(binary),
        Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
        Expr::Literal(literal) => visitor.visit_literal(literal),
        Expr::Unary(unary) => visitor.visit_unary(unary),
    }
}

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

pub struct Literal {
    pub value: LiteralType,
}

pub enum LiteralType {
    Float(f64),
    String(String),
    True,
    False,
    Nil,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Expr {
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
            value: LiteralType::Float(value),
        })
    }

    pub fn string(value: &str) -> Expr {
        Expr::Literal(Self {
            value: LiteralType::String(String::from(value)),
        })
    }

    pub fn nil() -> Expr {
        Expr::Literal(Self {
            value: LiteralType::Nil,
        })
    }
    pub fn r#true() -> Expr {
        Expr::Literal(Self {
            value: LiteralType::True,
        })
    }

    pub fn r#false() -> Expr {
        Expr::Literal(Self {
            value: LiteralType::False,
        })
    }
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Expr {
        Expr::Unary(Self {
            operator,
            right: Box::new(right),
        })
    }
}
