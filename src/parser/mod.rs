#[allow(dead_code)]
pub mod error;
pub use error::ParsingError;
//use crate::scanner::{TokenType::{*,self}, Token};
use crate::scanner::ScannedToken;
use crate::syntax_trees::expression::{
    binary::Operator as BinaryOperator,
    unary::Operator as UnaryOperator, 
    Expression,
};
use crate::syntax_trees::statement::Statement;
use crate::token::Token;
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
        while let Some(token) = self.iter.peek() {
            let statement = match token.type_{
                Token::VAR => self.varDeclaration(),
                _ => self.statement(),
            };
            if let Err(e) = statement{
                self._synchronize();
                return Err(e)
            }
            statements.push(statement.unwrap())
            //statements.push(self.declaration()?);        
        }

        Ok(statements)
    }


    fn statement(&mut self) -> Result<Statement>{

        todo!() 
    }
/*match token.type_ {
                Token::PRINT => self.print_statement()?,
                _ => self.expression_statement()?,
            });
*/

    fn print_statement(&mut self) -> Result<Statement> {
        let value = self.expression()?;
        if self.iter.next_if(|x| x.type_ == Token::SEMICOLON).is_some() {
            Ok(Statement::Print(value))
        } else {
            Err(ParsingError::NoSemi)
        }
    }

    fn expression_statement(&mut self) -> Result<Statement> {
        let value = self.expression()?;
        if self.iter.next_if(|x| x.type_ == Token::SEMICOLON).is_some() {
            Ok(Statement::Expression(value))
        } else {
            Err(ParsingError::NoSemi)
        }
    }

    fn expression(&mut self) -> Result<Expression> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression>{
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
            return Ok(Expression::unary(operator, right));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expression> {
        if self.iter.peek().is_none() {
            return Err(ParsingError::NoExpr);
        }

        let Some(ScannedToken { type_: token, .. }) = self.iter.next_if(|x| is_terminal(&x.type_))
        else {
            return Err(ParsingError::NoExpr);
        };
        match token {
            Token::FALSE => Ok(false.into()),
            Token::TRUE => Ok(true.into()),
            Token::NIL => Ok(Expression::nil()),
            Token::NUMBER {
                lexeme: _,
                value: num,
            } => Ok(num.into()),
            Token::STRING(str_) => Ok(str_.into()),
            Token::LEFTPAREN => self.handle_paren(),
            _ => Err(ParsingError::NoExpr),
        }
    }
}

fn is_terminal(token: &Token) -> bool {
    matches!(
        token,
        Token::FALSE
            | Token::TRUE
            | Token::NIL
            | Token::NUMBER {
                lexeme: _,
                value: _
            }
            | Token::STRING(_)
            | Token::LEFTPAREN
    )
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
            expr = Expression::binary(expr, operator, right);
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
            Ok(Expression::grouping(expr))
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
