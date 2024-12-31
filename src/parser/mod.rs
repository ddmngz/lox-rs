#[allow(dead_code)]
pub mod error;
pub use error::ParsingError;
//use crate::scanner::{TokenType::{*,self}, Token};
use crate::scanner::ScannedToken;
use crate::syntax_trees::expression::{BinaryOperator, Expression, LogicalOperator, UnaryOperator};
use crate::syntax_trees::lox_object::LoxObject;
use crate::syntax_trees::statement::Function;
use crate::syntax_trees::statement::Statement;
use crate::token::Identifier;
use crate::token::Token;
use crate::token::TokenDiscriminant;
use std::fmt;
use std::iter::Peekable;

pub struct Parser {
    iter: Peekable<<Vec<ScannedToken> as IntoIterator>::IntoIter>,
}

#[derive(Copy, Clone, Debug)]
pub enum FunctionKind {
    Function,
    Method,
}

impl fmt::Display for FunctionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Function => "function",
                Self::Method => "method",
            }
        )
    }
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
        let result = if self.iter.next_if(|x| x.type_ == Token::VAR).is_some() {
            self.var_declaration()
        } else if self.iter.next_if(|x| x.type_ == Token::FUN).is_some() {
            self.function(FunctionKind::Function)
        } else {
            self.statement()
        };
        if result.is_err() {
            self._synchronize();
        }
        result
    }

    fn get_identifier(&mut self) -> Result<Identifier> {
        let Some(next) = self.iter.next() else {
            return Err(ParsingError::NoIdentifier);
        };
        let res: std::result::Result<Identifier, ()> = next.type_.try_into();
        res.map_err(|_| ParsingError::NoIdentifier)
    }

    fn function(&mut self, kind: FunctionKind) -> Result<Statement> {
        let name = self.get_identifier()?;
        self.consume(Token::LEFTPAREN, ParsingError::FnParenOpen(kind))?;
        let mut params = Vec::new();
        if self
            .iter
            .peek()
            .is_some_and(|x| x.type_ != Token::RIGHTPAREN)
        {
            loop {
                if params.len() >= 255 {
                    println!("Can't have more than 255 parameters.");
                };
                params.push(self.get_identifier()?);
                if self.iter.next_if(|x| x.type_ == Token::COMMA).is_none() {
                    break;
                }
            }
        }

        self.consume(Token::RIGHTPAREN, ParsingError::FnParenClosed(kind))?;
        self.consume(Token::LEFTBRACE, ParsingError::FnNoBraceOpen(kind))?;
        let body = self.block()?;
        Ok(Statement::Function(Function { name, params, body }))
    }

    fn consume(
        &mut self,
        token: impl PartialEq<Token>,
        error: ParsingError,
    ) -> Result<ScannedToken> {
        self.iter.next_if(|x| token.eq(&x.type_)).ok_or(error)
    }

    fn var_declaration(&mut self) -> Result<Statement> {
        let Ok(ScannedToken {
            type_: Token::IDENTIFIER(name),
            ..
        }) = self.consume(TokenDiscriminant::IDENTIFIER, ParsingError::NoIdentifier)
        else {
            return Err(Self::error(
                ParsingError::NoIdentifier,
                self.iter.peek().map(|x| x.line),
            ));
        };

        let initializer = if self.iter.next_if(|x| x.type_ == Token::EQUAL).is_some() {
            Some(self.expression()?)
        } else {
            None
        };

        self.semicolon()
            .map(|()| Statement::Var { name, initializer })
    }

    fn semicolon(&mut self) -> Result<()> {
        if self.iter.next_if(|x| x.type_ == Token::SEMICOLON).is_some() {
            Ok(())
        } else {
            Err(Self::error(
                ParsingError::NoSemi,
                self.iter.peek().map(|x| x.line),
            ))
        }
    }

    fn statement(&mut self) -> Result<Statement> {
        if self.iter.next_if(|x| x.type_ == Token::PRINT).is_some() {
            self.print_statement()
        } else if self.iter.next_if(|x| x.type_ == Token::FOR).is_some() {
            self.for_statement()
        } else if self.iter.next_if(|x| x.type_ == Token::WHILE).is_some() {
            self.while_statement()
        } else if self.iter.next_if(|x| x.type_ == Token::LEFTBRACE).is_some() {
            Ok(Statement::Block(self.block()?))
        } else if self.iter.next_if(|x| x.type_ == Token::IF).is_some() {
            self.if_statement()
        } else if let Some(keyword) = self.iter.next_if(|x| x.type_ == Token::RETURN) {
            self.return_statement(keyword)
        } else {
            self.expression_statement()
        }
    }

    fn return_statement(&mut self, keyword: ScannedToken) -> Result<Statement> {
        if self.iter.next_if(|x| x.type_ == Token::SEMICOLON).is_some() {
            Ok(Statement::Return {
                token: keyword,
                value: None,
            })
        } else {
            let value = Some(self.expression()?);
            self.consume(Token::SEMICOLON, ParsingError::NoSemi)?;
            Ok(Statement::Return {
                token: keyword,
                value,
            })
        }
    }

    fn for_statement(&mut self) -> Result<Statement> {
        self.consume(TokenDiscriminant::LEFTPAREN, ParsingError::ForParenOpen)?;

        let initializer = if self.iter.next_if(|x| x.type_ == Token::SEMICOLON).is_some() {
            None
        } else if self.iter.next_if(|x| x.type_ == Token::VAR).is_some() {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if self
            .iter
            .peek()
            .is_some_and(|x| x.type_ == Token::SEMICOLON)
        {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(Token::SEMICOLON, ParsingError::ConditionNoSemi)?;

        let increment = if self
            .iter
            .peek()
            .is_some_and(|x| x.type_ != Token::RIGHTPAREN)
        {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenDiscriminant::RIGHTPAREN, ParsingError::ForParenClosed)?;

        let body = if let Some(increment) = increment {
            Statement::Block(vec![self.statement()?, Statement::Expression(increment)])
        } else {
            self.statement()?
        };

        let condition = condition.unwrap_or(Expression::Literal(LoxObject::Bool(true)));

        let body = Statement::While {
            condition,
            body: Box::new(body),
        };

        if let Some(initializer) = initializer {
            Ok(Statement::Block(vec![initializer, body]))
        } else {
            Ok(body)
        }
    }

    fn while_statement(&mut self) -> Result<Statement> {
        self.consume(TokenDiscriminant::LEFTPAREN, ParsingError::WhileParenOpen)?;
        let condition = self.expression()?;
        self.consume(
            TokenDiscriminant::RIGHTPAREN,
            ParsingError::WhileParenClosed,
        )?;
        let body = Box::new(self.statement()?);
        Ok(Statement::While { condition, body })
    }

    fn if_statement(&mut self) -> Result<Statement> {
        self.consume(TokenDiscriminant::LEFTPAREN, ParsingError::IfParenOpen)?;
        let condition = self.expression()?;
        eprintln!("if condition: {condition}");
        self.consume(TokenDiscriminant::RIGHTPAREN, ParsingError::IfParenOpen)?;

        let then = Box::new(self.statement()?);

        let else_case = if self.iter.next_if(|x| x.type_ == Token::ELSE).is_some() {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then,
            else_case,
        })
    }

    fn block(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        while self
            .iter
            .peek()
            .is_some_and(|x| x.type_ != Token::RIGHTBRACE)
        {
            statements.push(self.declaration()?)
        }

        if self
            .iter
            .next_if(|x| x.type_ == Token::RIGHTBRACE)
            .is_some()
        {
            Ok(statements)
        } else {
            Err(Self::error(ParsingError::UntermBrace, None))
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

    fn assignment(&mut self) -> Result<Expression> {
        let expression = self.or()?;
        if let Some(ScannedToken { line, .. }) = self.iter.next_if(|x| x.type_ == Token::EQUAL) {
            let value = Box::new(self.assignment()?);
            if let Expression::Variable { name, .. } = expression {
                return Ok(Expression::Assign { name, value });
            } else {
                return Err(Self::error(ParsingError::InvalidAssignment, Some(line)));
            }
        }
        Ok(expression)
    }

    fn or(&mut self) -> Result<Expression> {
        let mut types = [Token::AND];
        self.descent_logical(Self::and, &mut types)
    }

    fn and(&mut self) -> Result<Expression> {
        let mut types = [Token::OR];
        self.descent_logical(Self::equality, &mut types)
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

        let res = self.recursive_descend(Self::term, &mut types);
        res
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
        self.call()
    }

    fn call(&mut self) -> Result<Expression> {
        let mut expr = self.primary();
        while self.iter.next_if(|x| x.type_ == Token::LEFTPAREN).is_some() {
            expr = self.finish_call(expr?);
        }
        expr
    }

    fn next_if(&mut self, token: impl PartialEq<Token>) -> Option<ScannedToken> {
        self.iter.next_if(|x| token.eq(&x.type_))
    }

    fn next_unless(&mut self, token: impl PartialEq<Token>) -> Option<ScannedToken> {
        self.iter.next_if(|x| !token.eq(&x.type_))
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression> {
        let mut args = Vec::new();
        if self
            .iter
            .peek()
            .is_some_and(|x| x.type_ != Token::RIGHTPAREN)
        {
            loop {
                if args.len() >= 255 {
                    Self::error(
                        ParsingError::TooManyArgs,
                        Some(self.iter.peek().unwrap().line),
                    );
                }
                args.push(self.expression()?);
                if self.next_if(Token::COMMA).is_none() {
                    break;
                };
            }
        };
        let paren = self.consume(Token::RIGHTPAREN, ParsingError::FnNoCloseParen)?;
        Ok(Expression::Call {
            callee: Box::new(callee),
            paren,
            args,
        })
    }

    fn primary(&mut self) -> Result<Expression> {
        let Some(ScannedToken { type_: token, line }) = self.iter.next() else {
            return Err(Self::error(ParsingError::NoExpr, None));
        };
        match token {
            Token::FALSE => Ok(false.into()),
            Token::TRUE => Ok(true.into()),
            Token::NIL => Ok(Expression::nil()),
            Token::NUMBER(num) => Ok(num.into()),
            Token::STRING(string) => Ok(string.into()),
            Token::LEFTPAREN => self.handle_paren(),
            Token::IDENTIFIER(name) => Ok(Expression::Variable { name, line }),
            _ => Err(Self::error(ParsingError::NoExpr, Some(line))),
        }
    }
}

impl Parser {
    fn descent_logical(
        &mut self,
        f: fn(&mut Self) -> Result<Expression>,
        types: &mut [Token],
    ) -> Result<Expression> {
        let mut expr = f(self)?;
        while let Some(token) = self.iter.next_if(|x| types.contains(&x.type_)) {
            let left = Box::new(expr);
            let operator = LogicalOperator::try_from(token.type_).unwrap();
            let right = Box::new(f(self)?);
            expr = Expression::Logical {
                left,
                operator,
                right,
            };
        }
        Ok(expr)
    }

    fn recursive_descend(
        &mut self,
        f: fn(&mut Self) -> Result<Expression>,
        types: &mut [Token],
    ) -> Result<Expression> {
        let mut expr = f(self)?;
        // TODO: see if there's a way we can combine the while let to remove the unwrap

        while let Some(token) = self.iter.next_if(|x| types.contains(&x.type_)) {
            let operator = BinaryOperator::from_token(token).unwrap();
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
            Err(Self::error(
                ParsingError::UntermParen,
                self.iter.peek().map(|x| x.line),
            ))
        }
    }

    fn error(error: ParsingError, line: Option<u32>) -> ParsingError {
        match line {
            Some(line) => {
                println!("Error on line {}: {}", line, error)
            }
            None => {
                println!("Error at end of file: {}", error)
            }
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
