use super::error::LoxError;
use super::error::LoxError::*;
use crate::lox::token::{Token, TokenType};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::iter::{Enumerate, Peekable};
use std::str::Chars;
use std::str::FromStr;

mod helpers;

static KEYWORDS: Lazy<HashMap<&str, TokenType>> = Lazy::new(|| {
    use TokenType::*;
    HashMap::from([
        ("and", AND),
        ("class", CLASS),
        ("else", ELSE),
        ("false", FALSE),
        ("for", FOR),
        ("fun", FUN),
        ("if", IF),
        ("nil", NIL),
        ("or", OR),
        ("print", PRINT),
        ("return", RETURN),
        ("super", SUPER),
        ("this", THIS),
        ("true", TRUE),
        ("var", VAR),
        ("while", WHILE),
    ])
});

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    iter: Peekable<Enumerate<Chars<'a>>>,
    lex_start: usize,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::<Token>::new(),
            iter: source.chars().enumerate().peekable(),
            lex_start: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxError> {
        let mut ret: Result<(), LoxError> = Ok(());
        while !self.is_at_end() {
            self.lex_start = self.get_pos();
            ret = self.scan_token();
        }

        self.tokens.push(Token::eof(self.line));

        if let Err(err) = ret {
            Err(err)
        } else {
            Ok(std::mem::take(&mut self.tokens))
        }
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let (_, value) = self.iter.next().unwrap();
        use TokenType::*;

        match value {
            '(' => self.add_token(LEFTPAREN),
            ')' => self.add_token(RIGHTPAREN),
            '{' => self.add_token(LEFTBRACE),
            '}' => self.add_token(RIGHTBRACE),
            ',' => self.add_token(COMMA),
            '.' => self.add_token(DOT),
            '-' => self.add_token(MINUS),
            '+' => self.add_token(PLUS),
            ';' => self.add_token(SEMICOLON),
            '*' => self.add_token(STAR),
            '!' => self.add_operator(BANGEQUAL, BANG, '='),
            '=' => self.add_operator(EQUALEQUAL, EQUAL, '='),
            '<' => self.add_operator(LESSEQUAL, LESS, '='),
            '>' => self.add_operator(GREATEREQUAL, LESS, '='),
            '/' => {
                if self.check_next('/') {
                    self.advance_while(|(_, x)| *x != '\n');
                    Ok(())
                } else {
                    self.add_token(SLASH)
                }
            }
            ' ' => Ok(()),
            '\r' => Ok(()),
            '\t' => Ok(()),
            '\n' => {
                self.line += 1;
                Ok(())
            }
            '"' => self.handle_string(),
            _ => self.handle_complex_lexemes(value),
        }
    }

    fn add_token(&mut self, __type: TokenType<'a>) -> Result<(), LoxError> {
        let lexeme = self.get_lexeme();
        self.tokens.push(Token::new(__type, lexeme, self.line));
        Ok(())
    }

    fn add_operator(
        &mut self,
        double_operator: TokenType<'a>,
        single_operator: TokenType<'a>,
        next: char,
    ) -> Result<(), LoxError> {
        let operator = if self.check_next(next) {
            double_operator
        } else {
            single_operator
        };
        self.add_token(operator)
    }

    fn handle_string(&mut self) -> Result<(), LoxError> {
        while let Some((_, x)) = self.iter.next_if(|(_, x)| *x != '"') {
            if x == '\n' {
                self.line += 1;
            }
        }

        if self.iter.peek().is_none() {
            UntermString.error(self.line);
            return Ok(());
        }

        self.iter.next();

        let pos = self.get_pos();
        let literal = &self.source[self.lex_start + 1..pos - 1];
        self.add_token(TokenType::STRING(literal))
    }

    fn handle_complex_lexemes(&mut self, val: char) -> Result<(), LoxError> {
        if val.is_ascii_digit() {
            self.handle_number()
        } else if val.is_ascii_alphanumeric() {
            self.handle_identifier()
        } else {
            Err(Syntax.error(self.line))
        }
    }

    fn handle_number(&mut self) -> Result<(), LoxError> {
        self.advance_while(|(_, x)| x.is_ascii_digit());
        self.handle_decimal();
        let num = f64::from_str(self.get_lexeme()).unwrap();
        self.add_token(TokenType::NUMBER(num))
    }

    fn handle_decimal(&mut self) {
        // if we have a . and then decimals keep going
        if self.iter.next_if(|(_, x)| *x == '.').is_some() {
            self.advance_while(|(_, x)| x.is_ascii_digit());
        }
    }

    fn handle_identifier(&mut self) -> Result<(), LoxError> {
        self.advance_while(|(_, x)| x.is_ascii_alphanumeric());
        let text = self.get_lexeme();
        self.add_token(*(*KEYWORDS).get(text).unwrap_or(&TokenType::IDENTIFIER))
    }
}
