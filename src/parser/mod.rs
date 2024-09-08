#[allow(dead_code)]
pub mod error;
pub use error::ParsingError;
//use crate::scanner::{TokenType::{*,self}, Token};
use crate::scanner::ScannedToken;
use crate::syntax_trees::expression::{BinaryOperator, Expression, UnaryOperator};
use crate::syntax_trees::statement::Statement;
use crate::token::Token;
use crate::token::TokenDiscriminant;
use std::iter::Peekable;

pub struct Parser {
    iter: Peekable<<Vec<ScannedToken> as IntoIterator>::IntoIter>,
}

type Result<T> = std::result::Result<T, ParsingError>;

impl Parser {
    pub fn new(tokens: Vec<ScannedToken>) -> Self {
        let iter = tokens.into_iter().peekable();
        Self { iter }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        let mut last_error = None;
        while self.iter.peek().is_some() {
            // discriminant so we can cheaply pass it around
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(e) => {
                    last_error = Some(e);
                }
            }
        }

        match last_error {
            Some(e) => Err(e),
            None => Ok(statements),
        }
    }

    fn declaration(&mut self) -> Result<Statement> {
        let result = if self.iter.next_if(|x| x.type_ == Token::VAR).is_some(){
            self.var_declaration()
        }else{
            self.statement()
        };
        if result.is_err() {
            self._synchronize();
        }
        result
    }

    fn consume(&mut self, token: TokenDiscriminant) -> Option<ScannedToken> {
        self.iter
            .next_if(|x| TokenDiscriminant::from(&x.type_) == token)
    }

    fn var_declaration(&mut self) -> Result<Statement> {
        let Some(ScannedToken {
            type_: Token::IDENTIFIER(name),
            ..
        }) = self.consume(TokenDiscriminant::IDENTIFIER)
        else {
            Self::error(ParsingError::NoIdentifier, self.iter.peek().map(|x| x.line));
            return Err(ParsingError::NoIdentifier);
        };

        let initializer = if self.iter.next_if(|x| x.type_ == Token::EQUAL).is_some() {
            Some(self.expression()?)
        } else {
            None
        };

        self.semicolon().map(|()| Statement::Var{name, initializer})
    }

    fn semicolon(&mut self) -> Result<()>{
        if self.iter.next_if(|x| x.type_ == Token::SEMICOLON).is_some() {
            Ok(())
        } else {
            Err(Self::error(ParsingError::NoSemi, self.iter.peek().map(|x| x.line)))
        }
    }

    fn statement(&mut self) -> Result<Statement> {
        if self.iter.next_if(|x| x.type_ == Token::PRINT).is_some(){
            self.print_statement()
        }else{
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Statement> {
        let value = self.expression()?;
        self.semicolon().map(|()| Statement::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Statement> {
        let value = self.expression()?;
        self.semicolon().map(|()| Statement::Expression(value))
    }

    fn expression(&mut self) -> Result<Expression> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression>{
        let expression = self.equality()?;
        if let Some(ScannedToken{line,..}) = self.iter.next_if(|x| x.type_ == Token::EQUAL){
            let value = Box::new(self.assignment()?);
            if let Expression::Variable(name) = expression {
                return Ok(Expression::Assign{
                    name,
                    value
                })
            }else{
                return Err(Self::error(ParsingError::InvalidAssignment, Some(line)))
            }
        }
        Ok(expression)
    }

    fn equality(&mut self) -> Result<Expression> {
        let mut types = [Token::BANGEQUAL, Token::EQUALEQUAL];
        self.recursive_descend(Self::comparison, &mut types)
    }

    fn comparison(&mut self) -> Result<Expression> {
        let mut types = [
            Token::GREATER,
            Token::GREATEREQUAL,
            Token::LESS,
            Token::LESSEQUAL,
        ];
        self.recursive_descend(Self::term, &mut types)
    }

    fn term(&mut self) -> Result<Expression> {
        let mut types = [Token::MINUS, Token::PLUS];
        self.recursive_descend(Self::factor, &mut types)
    }

    fn factor(&mut self) -> Result<Expression> {
        let mut types = [Token::SLASH, Token::STAR];
        self.recursive_descend(Self::unary, &mut types)
    }

    fn unary(&mut self) -> Result<Expression> {
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
            return Ok(Expression::Unary {
                operator,
                inner: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expression> {
        let Some(ScannedToken{type_:token, line}) = self.iter.next()else{
            return Err(Self::error(ParsingError::NoExpr, None))
        };

        match token{
            Token::FALSE => Ok(false.into()),
            Token::TRUE => Ok(true.into()),
            Token::NIL => Ok(Expression::nil()),
            Token::NUMBER {
                lexeme: _,
                value: num,
            } => Ok(num.into()),
            Token::STRING(str_) => Ok(str_.into()),
            Token::LEFTPAREN => self.handle_paren(),
            Token::IDENTIFIER(name) => Ok(Expression::Variable(name)),
            _ => Err(Self::error(ParsingError::NoExpr, Some(line))),
        }
    }
}



impl Parser {
    fn recursive_descend(
        &mut self,
        f: fn(&mut Self) -> Result<Expression>,
        types: &mut [Token],
    ) -> Result<Expression> {
        let mut expr = f(self)?;
        // TODO: see if there's a way we can combine the while let to remove the unwrap
        while let Some(token) = self.iter.next_if(|x| types.contains(&x.type_)) {
            let operator = BinaryOperator::from_token(token.type_).unwrap();
            let right = f(self)?;
            let left = Box::new(expr);
            let right = Box::new(right);
            expr = Expression::Binary {
                left,
                operator,
                right,
            };
        }
        Ok(expr)
    }

    fn handle_paren(&mut self) -> Result<Expression> {
        let expr = self.expression()?;
        if self
            .iter
            .next_if(|x| x.type_ == Token::RIGHTPAREN)
            .is_some()
        {
            Ok(Expression::Grouping(Box::new(expr)))
        } else {
            Err(Self::error(ParsingError::UntermParen, self.iter.peek().map(|x| x.line)))
        }
    }

    fn error(error:ParsingError, line: Option<u32>) -> ParsingError{
        match line{
            Some(line) => {println!("Error on line {}: {}", line, error)},
            None => {println!("Error at end of file: {}",error)}
        };
        error
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
