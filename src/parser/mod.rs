#[allow(dead_code)]
pub mod ast_printer;
pub mod error;
pub use error::ParsingError;
//use crate::scanner::{TokenType::{*,self}, Token};
use crate::scanner::ScannedToken;
use crate::syntax_trees::expression::{
    Binary, BinaryOperator, Expr, Grouping, Literal, Unary, UnaryOperator,
};
use crate::syntax_trees::statement::Statement;
use crate::token::Token;
use std::iter::Peekable;

pub struct Parser<'s> {
    iter: Peekable<<Vec<ScannedToken<'s>> as IntoIterator>::IntoIter>,
}

type Result<T> = std::result::Result<T, ParsingError>;

impl<'s> Parser<'s> {
    pub fn new(tokens: Vec<ScannedToken>) -> Self {
        let iter = tokens.into_iter().peekable();
        Self { iter }
    }

    pub fn parse(&mut self) -> Result<Vec<&dyn Statement<'s>>> {
        let mut statements = Vec::new();
        while let Some(token) = self.iter.next() {
            statements.push(match token.type_ {
                Token::PRINT => self.print_statement()?,
                _ => self.expression_statement()?,
            });
        }

        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<&dyn Statement> {
        let value = self.expression()?;
        if self.iter.next_if(|x| x.type_ == Token::SEMICOLON).is_some() {
            Ok(Statement::new_print(value))
        } else {
            Err(ParsingError::NoSemi)
        }
    }

    fn expression_statement(&mut self) -> Result<&dyn Statement> {
        let value = self.expression()?;
        if self.iter.next_if(|x| x.type_ == Token::SEMICOLON).is_some() {
            Ok(Statement::new_expression(value))
        } else {
            Err(ParsingError::NoSemi)
        }
    }

    fn expression(&mut self) -> Result<&dyn Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<&dyn Expr> {
        let mut types = [Token::BANGEQUAL, Token::EQUALEQUAL];
        self.recursive_descend(Self::comparison, &mut types)
    }

    fn comparison(&mut self) -> Result<&dyn Expr> {
        let mut types = [
            Token::GREATER,
            Token::GREATEREQUAL,
            Token::LESS,
            Token::LESSEQUAL,
        ];
        self.recursive_descend(Self::term, &mut types)
    }

    fn term(&mut self) -> Result<&dyn Expr> {
        let mut types = [Token::MINUS, Token::PLUS];
        self.recursive_descend(Self::factor, &mut types)
    }

    fn factor(&mut self) -> Result<&dyn Expr> {
        let mut types = [Token::SLASH, Token::STAR];
        self.recursive_descend(Self::unary, &mut types)
    }

    fn unary(&mut self) -> Result<&dyn Expr> {
        if let Some(token) = self
            .iter
            .next_if(|x| [Token::BANG, Token::MINUS].contains(&x.type_))
        {
            let operator = if token.type_ == Token::BANG {
                UnaryOperator::BANG
            } else {
                UnaryOperator::MINUS
            };
            let right = self.unary()?;
            return Ok(&Unary::new(operator, right));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Literal> {
        if self.iter.peek().is_none() {
            return Err(ParsingError::NoExpr);
        }

        let Some(ScannedToken{type_:token, ..}) = self.iter.next_if(|x| is_terminal(&x.type_)) else{
            return Err(ParsingError::NoExpr)
        };
        match token{
            Token::FALSE => Ok(Literal::r#false()),
            Token::TRUE => Ok(Literal::r#true()),
            Token::NIL => Ok(Literal::r#nil()),
            Token::NUMBER {
                lexeme: _,
                value: num,
            } => Ok(Literal::float(num)),
            Token::STRING(str_) => Ok(Literal::string(str_.clone())),
            Token::LEFTPAREN => self.handle_paren(),
            _ => Err(ParsingError::NoExpr),
        }
    }
}

fn is_terminal(token:&Token) -> bool{
    matches!(token, Token::FALSE|Token::TRUE|Token::NIL|Token::NUMBER{lexeme:_, value:_}|Token::STRING(_)|Token::LEFTPAREN)

}

impl<'s> Parser<'s> {
    fn recursive_descend(
        &mut self,
        f: fn(&mut Self) -> Result<&dyn Expr>,
        types: &mut [Token],
    ) -> Result<&dyn Expr> {
        let mut expr: Expr = f(self)?;
        // TODO: see if there's a way we can combine the while let to remove the unwrap
        while let Some(token) = self.iter.next_if(|x| types.contains(&x.type_)) {
            let operator = BinaryOperator::from_token(token.type_).unwrap();
            let right = f(self)?;
            expr = Binary::new(expr, operator, right);
        }
        Ok(&expr)
    }

    fn handle_paren(&mut self) -> Result<&dyn Expr> {
        let expr = self.expression()?;
        if self
            .iter
            .next_if(|x| x.type_ == Token::RIGHTPAREN)
            .is_some()
        {
            Ok(Grouping::new(expr))
        } else {
            Err(ParsingError::UntermParen)
        }
    }

    fn _synchronize(&mut self) {
        const SYNC_POINTS: [Token; 8] = [
            Token::CLASS,
            Token::FUN,
            Token::VAR,
            Token::FOR,
            Token::IF,
            Token::WHILE,
            Token::PRINT,
            Token::RETURN,
        ];

        if self.iter.peek().is_none() {
            return;
        }

        while let Some(token) = self.iter.peek() {
            let token_type = &token.type_;
            if *token_type == Token::SEMICOLON {
                self.iter.next();
                return;
            } else if SYNC_POINTS.contains(token_type) {
                return;
            }
            self.iter.next();
        }
    }
}
