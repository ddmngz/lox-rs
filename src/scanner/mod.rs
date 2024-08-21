use super::error::LoxError;
use super::error::LoxError::*;
use super::token::{Token, TokenType};
use phf::phf_map;
use std::iter::{IntoIterator, Peekable};

mod helpers;

static KEYWORDS: phf::Map<&'static str, TokenType> = {
    use TokenType::*;
    phf_map! {
        "and" => AND,
        "class" => CLASS,
        "else" => ELSE,
        "false" => FALSE,
        "for" => FOR,
        "fun" => FUN,
        "if" => IF,
        "nil" => NIL,
        "or" => OR,
        "print" => PRINT,
        "return" => RETURN,
        "super" => SUPER,
        "this" => THIS,
        "true" => TRUE,
        "var" => VAR,
        "while" => WHILE,
    }
};

pub struct Scanner {
    tokens: Vec<Token>,
    iter: Peekable<<Vec<char> as IntoIterator>::IntoIter>,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let chars: Vec<char> = source.chars().collect();
        Self {
            iter: chars.into_iter().peekable(),
            tokens: Vec::<Token>::new(),
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, LoxError> {
        let mut ret: Result<(), LoxError> = Ok(());
        while !self.is_at_end() {
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
        let value = self.iter.next().unwrap();
        use TokenType::*;
        match value {
            '(' => self.add_char(LEFTPAREN, value),
            ')' => self.add_char(RIGHTPAREN, value),
            '{' => self.add_char(LEFTBRACE, value),
            '}' => self.add_char(RIGHTBRACE, value),
            ',' => self.add_char(COMMA, value),
            '.' => self.add_char(DOT, value),
            '-' => self.add_char(MINUS, value),
            '+' => self.add_char(PLUS, value),
            ';' => self.add_char(SEMICOLON, value),
            '*' => self.add_char(STAR, value),
            '!' => self.add_operator(BANGEQUAL, BANG, value),
            '=' => self.add_operator(EQUALEQUAL, EQUAL, value),
            '<' => self.add_operator(LESSEQUAL, LESS, value),
            '>' => self.add_operator(GREATEREQUAL, GREATER, value),
            '/' => {
                if self.check_next('/') {
                    self.advance_while(|x| *x != '\n');
                    Ok(())
                } else {
                    self.add_char(SLASH, value)
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

    fn add_char(&mut self, __type: TokenType, token: char) -> Result<(), LoxError> {
        self.add_token(__type, &token.to_string())
    }

    fn add_operator(
        &mut self,
        double_operator: TokenType,
        single_operator: TokenType,
        value: char,
    ) -> Result<(), LoxError> {
        if self.check_next('=') {
            self.iter.next();
            let mut lexeme = String::with_capacity(2);
            lexeme.push(value);
            lexeme.push('=');
            self.add_token(double_operator, &lexeme)
        } else {
            self.add_char(single_operator, value)
        }
    }

    fn handle_string(&mut self) -> Result<(), LoxError> {
        let cur_str = self.advance_and_get_literal(|x| *x != '"');
        // ref cell thing

        if self.iter.peek().is_none() {
            UntermString.error(self.line);
            return Ok(());
        }

        self.iter.next();
        let token = Token::new_string(&cur_str, self.line);
        self.tokens.push(token);
        Ok(())
    }

    fn handle_complex_lexemes(&mut self, val: char) -> Result<(), LoxError> {
        if val.is_ascii_digit() {
            self.handle_number(val)
        } else if val.is_ascii_alphanumeric() {
            self.handle_identifier(val)
        } else {
            Err(Syntax.error(self.line))
        }
    }

    fn handle_number(&mut self, val: char) -> Result<(), LoxError> {
        let mut cur_str = String::from(val);
        cur_str.push_str(&self.advance_and_get_literal(char::is_ascii_digit));

        if self.iter.next_if(|x| *x == '.').is_some() {
            cur_str.push('.');
            cur_str.push_str(&self.advance_and_get_literal(char::is_ascii_digit));
        }
        let token = Token::new_number(&cur_str, self.line)?;
        self.tokens.push(token);
        Ok(())
    }

    fn handle_identifier(&mut self, val: char) -> Result<(), LoxError> {
        let mut cur_str = String::from(val);
        cur_str.push_str(&self.advance_and_get_literal(char::is_ascii_alphanumeric));
        let str_pointer = cur_str.as_str();
        let token;
        if let Some(tok) = KEYWORDS.get(str_pointer) {
            token = tok.clone();
        } else {
            token = TokenType::IDENTIFIER;
        }
        self.add_token(token, &cur_str)
    }

    fn add_token(&mut self, token: TokenType, literal: &str) -> Result<(), LoxError> {
        let token = Token::new(token, literal, self.line);
        self.tokens.push(token);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenType::*;
    #[test]
    fn scan_equation() {
        compare_scan("1+1", vec![NUMBER(1.0), PLUS, NUMBER(1.0)])
    }

    #[test]
    fn scan_quotes() {
        compare_scan("\"hiiii\"", vec![STRING("hiiii".into())])
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
        compare_scan("(1+1)", vec![NUMBER(1.0), PLUS, NUMBER(1.0)])
    }

    fn compare_scan(string: &str, goal: Vec<TokenType>) {
        let scanned_tokens: Vec<_> = Scanner::new(string.to_string())
            .scan_tokens()
            .unwrap()
            .into_iter()
            .map(|x| x.r#type)
            .collect();
        println!("{:?}", scanned_tokens);
        let scanned_tokens = Scanner::new(string.to_string())
            .scan_tokens()
            .unwrap()
            .into_iter()
            .map(|x| x.r#type);
        for (scanned_token, goal_token) in std::iter::zip(scanned_tokens, goal) {
            assert_eq!(scanned_token, goal_token)
        }
    }
}
