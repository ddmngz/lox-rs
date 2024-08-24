mod error;
mod scanned_token;
pub use error::ScanningError;



pub use crate::token::Token;
pub use scanned_token::ScannedToken;

use std::iter::{Iterator, Peekable};
use byteyarn::ByteYarn;
use std::str::Chars;


type Result<T> = std::result::Result<T, ScanningError>;

pub fn scan<'a>(source: &'a str) -> Result<Vec<ScannedToken>> {
    let mut scanner = Scanner::new(source);
    let mut err = None;
    let mut tokens = Vec::with_capacity(source.len());
    while scanner.can_scan() {
        match scanner.scan_token() {
            Ok(Some(token)) => tokens.push(token),
            Err(e) => err = Some(e),
            Ok(None) => {}
        }
    }

    match err {
        Some(e) => Err(e.into()),
        None => Ok(tokens),
    }
}


pub struct Scanner<'a> {
    iter: Peekable<Chars<'a>>,
    line: u32,
}


impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            iter: source.chars().peekable(),
            line: 1,
        }
    }

    pub fn can_scan(&mut self) -> bool {
        self.peek().is_some()
    }

    pub fn scan_token(&mut self) -> Result<Option<ScannedToken>> {
        use Token::*;
        let Some(value) = self.next() else {
            return Ok(None);
        };

        match value {
            '(' => Ok(Some(ScannedToken::new(LEFTPAREN, self.line))),
            ')' => Ok(Some(ScannedToken::new(RIGHTPAREN, self.line))),
            '{' => Ok(Some(ScannedToken::new(LEFTBRACE, self.line))),
            '}' => Ok(Some(ScannedToken::new(RIGHTBRACE, self.line))),
            ',' => Ok(Some(ScannedToken::new(COMMA, self.line))),
            '.' => Ok(Some(ScannedToken::new(DOT, self.line))),
            '-' => Ok(Some(ScannedToken::new(MINUS, self.line))),
            '+' => Ok(Some(ScannedToken::new(PLUS, self.line))),
            ';' => Ok(Some(ScannedToken::new(SEMICOLON, self.line))),
            '*' => Ok(Some(ScannedToken::new(STAR, self.line))),
            '!' => self.add_operator(Operator::BANG),
            '=' => self.add_operator(Operator::EQUAL),
            '<' => self.add_operator(Operator::LESS),
            '>' => self.add_operator(Operator::GREATER),
            '/' => {
                if self.next_if_eq(&'/').is_some() {
                    self.advance_while(|x| *x != '\n');
                    Ok(None)
                } else {
                    Ok(Some(ScannedToken::new(SLASH, self.line)))
                }
            }
            ' ' => Ok(None),
            '\r' => Ok(None),
            '\t' => Ok(None),
            '\n' => {
                self.line += 1;
                Ok(None)
            }
            '"' => self.handle_string(),
            _ => self.handle_complex_lexeme(value),
        }
    }

    fn add_operator(&mut self, type_: Operator) -> Result<Option<ScannedToken>> {
        if self.next_if_eq(&'=').is_some() {
            Ok(Some(ScannedToken::new(
                match type_ {
                    Operator::BANG => Token::BANGEQUAL,
                    Operator::EQUAL => Token::EQUALEQUAL,
                    Operator::LESS => Token::LESSEQUAL,
                    Operator::GREATER => Token::GREATEREQUAL,
                },
                self.line,
            )))
        } else {
            Ok(Some(ScannedToken::new(type_.into(), self.line)))
        }
    }

    fn handle_string(&mut self) -> Result<Option<ScannedToken>> {
        let slice = self.advance_and_get_literal(|x| *x != '"');

        if self.iter.peek().is_none() {
            ScanningError::UntermString.error(self.line);
            Err(ScanningError::UntermString)
        } else {
            // the closing '"'
            self.iter.next();
            Ok(Some(ScannedToken::new_string(slice, self.line)))
        }
    }

    fn handle_complex_lexeme(&mut self, val: char) -> Result<Option<ScannedToken>> {
        if val.is_ascii_digit() {
            self.handle_number(val)
        } else if val.is_ascii_alphanumeric() {
            self.handle_identifier(val)
        } else {
            Err(ScanningError::Syntax.error(self.line))
        }
    }
    fn handle_number(&mut self, val: char) -> Result<Option<ScannedToken>> {
        let mut cur_str = String::from(val);
        cur_str.push_str(&self.advance_and_get_literal(char::is_ascii_digit));

        if self.iter.next_if(|x| *x == '.').is_some() {
            cur_str.push('.');
            cur_str.push_str(&self.advance_and_get_literal(char::is_ascii_digit));
        }
        Ok(Some(ScannedToken::new_number(cur_str, self.line)?))
    }

    fn handle_identifier(&mut self, val: char) -> Result<Option<ScannedToken>> {
        let mut cur_str = String::from(val);
        cur_str.push_str(&self.advance_and_get_literal(char::is_ascii_alphanumeric));
        let str_pointer = cur_str.as_str();
        let token;
        if let Some(tok) = Token::from_keyword(str_pointer) {
            token = tok;
        } else {
            token = Token::IDENTIFIER(ByteYarn::from_string(cur_str));
        }
        Ok(Some(ScannedToken::new(token, self.line)))
    }

    pub fn advance_while<F>(&mut self, f: F)
    where
        F: Fn(&char) -> bool,
    {
        while self.iter.next_if(&f).is_some() {}
    }

    pub fn advance_and_get_literal<F>(&mut self, f: F) -> String
    where
        F: Fn(&char) -> bool,
    {
        let mut cur_str = String::new();
        while let Some(x) = self.iter.next_if(&f) {
            if x == '\n' {
                self.line += 1;
            }
            cur_str.push(x);
        }
        cur_str
    }

}

impl Iterator for Scanner<'_> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl Scanner<'_> {
    fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    fn next_if_eq(&mut self, expected: &char) -> Option<char> {
        self.iter.next_if_eq(expected)
    }
}

enum Operator {
    BANG,
    EQUAL,
    LESS,
    GREATER,
}

impl Into<Token> for Operator {
    fn into(self) -> Token {
        match self {
            Self::BANG => Token::BANG,
            Self::EQUAL => Token::EQUAL,
            Self::LESS => Token::LESS,
            Self::GREATER => Token::GREATER,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    static ONE: Token = NUMBER {
        lexeme: ByteYarn::from_static("1".as_bytes()),
        value: 1.0,
    };
    #[test]
    fn scan_equation() {
        compare_scan("1+1", vec![ONE.clone(), PLUS, ONE.clone()])
    }

    #[test]
    fn scan_quotes() {
        compare_scan(
            "\"hiiii\"",
            vec![STRING(ByteYarn::from_static("hiiii".as_bytes()).into())],
        )
    }

    #[test]
    fn scan_parens() {
        compare_scan(
            "({<>)}",
            vec![LEFTPAREN, LEFTBRACE, LESS, GREATER, RIGHTPAREN, RIGHTBRACE],
        )
    }

    #[test]
    fn scan_paren_equation() {
        compare_scan(
            "(1+1)",
            vec![LEFTPAREN, ONE.clone(), PLUS, ONE.clone(), RIGHTPAREN],
        )
    }

    fn compare_scan(string: &str, goal: Vec<Token>) {
        let scanned_tokens: Vec<_> = scan(string).unwrap().into_iter().map(|x| x.type_).collect();
        println!("{:?}", scanned_tokens);
        let scanned_tokens = scan(string).unwrap().into_iter().map(|x| x.type_);
        for (scanned_token, goal_token) in std::iter::zip(scanned_tokens, goal) {
            assert_eq!(scanned_token, goal_token)
        }
    }
}
