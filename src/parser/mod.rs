pub mod ast;
#[allow(dead_code)]
pub mod ast_printer;
use super::error::LoxParsingError;
use super::token::TokenType;
use super::token::TokenType::*;
use super::Token;
use ast::expression::{Binary, BinaryOperator, Expr, Grouping, Literal, Unary, UnaryOperator};
use ast::statement::Statement;
use std::iter::Peekable;

pub struct Parser {
    iter: Peekable<<Vec<Token> as IntoIterator>::IntoIter>,
}

type Result<T> = std::result::Result<T, LoxParsingError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let iter = tokens.into_iter().peekable();
        Self { iter }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        while let Some(token) = self.iter.next() {
            statements.push(match token.type_ {
                PRINT => self.print_statement()?,
                _ => self.expression_statement()?,
            });
        }

        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Statement> {
        let value = self.expression()?;
        if self.iter.next_if(|x| x.type_ == SEMICOLON).is_some() {
            Ok(Statement::new_print(value))
        } else {
            Err(LoxParsingError::NoSemi)
        }
    }

    fn expression_statement(&mut self) -> Result<Statement> {
        let value = self.expression()?;
        if self.iter.next_if(|x| x.type_ == SEMICOLON).is_some() {
            Ok(Statement::new_expression(value))
        } else {
            Err(LoxParsingError::NoSemi)
        }
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut types = [BANGEQUAL, EQUALEQUAL];
        self.recursive_descend(Self::comparison, &mut types)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut types = [GREATER, GREATEREQUAL, LESS, LESSEQUAL];
        self.recursive_descend(Self::term, &mut types)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut types = [MINUS, PLUS];
        self.recursive_descend(Self::factor, &mut types)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut types = [SLASH, STAR];
        self.recursive_descend(Self::unary, &mut types)
    }

    fn unary(&mut self) -> Result<Expr> {
        if let Some(token) = self.iter.next_if(|x| [BANG, MINUS].contains(&x.type_)) {
            let operator = if token.type_ == BANG {
                UnaryOperator::BANG
            } else {
                UnaryOperator::MINUS
            };
            let right = self.unary()?;
            return Ok(Unary::new(operator, right));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.iter.peek().is_none() {
            return Err(LoxParsingError::NoExpr);
        }

        match self.iter.next().unwrap().type_ {
            FALSE => Ok(Literal::r#false()),
            TRUE => Ok(Literal::r#true()),
            NIL => Ok(Literal::r#nil()),
            NUMBER {
                lexeme: _,
                value: num,
            } => Ok(Literal::float(num)),
            STRING(str_) => Ok(Literal::string(str_)),
            LEFTPAREN => self.handle_paren(),
            _ => Err(LoxParsingError::NoExpr),
        }
    }
}

impl Parser {
    fn recursive_descend(
        &mut self,
        f: fn(&mut Self) -> Result<Expr>,
        types: &mut [TokenType],
    ) -> Result<Expr> {
        let mut expr: Expr = f(self)?;
        // TODO: see if there's a way we can combine the while let to remove the unwrap
        while let Some(token) = self.iter.next_if(|x| types.contains(&x.type_)) {
            let operator = BinaryOperator::from_token(token).unwrap();
            let right = f(self)?;
            expr = Binary::new(expr, operator, right);
        }
        Ok(expr)
    }

    fn handle_paren(&mut self) -> Result<Expr> {
        let expr = self.expression()?;
        if self.iter.next_if(|x| x.type_ == RIGHTPAREN).is_some() {
            Ok(Grouping::new(expr))
        } else {
            Err(LoxParsingError::UntermParen)
        }
    }

    fn _synchronize(&mut self) {
        const SYNC_POINTS: [TokenType; 8] = [CLASS, FUN, VAR, FOR, IF, WHILE, PRINT, RETURN];

        if self.iter.peek().is_none() {
            return;
        }

        while let Some(token) = self.iter.peek() {
            let token_type = &token.type_;
            if *token_type == SEMICOLON {
                self.iter.next();
                return;
            } else if SYNC_POINTS.contains(token_type) {
                return;
            }
            self.iter.next();
        }
    }
}
