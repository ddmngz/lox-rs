use super::error::LoxError;
use super::error::LoxError::*;
use super::token::{Token, TokenType};
use byteyarn::ByteYarn;
use phf::phf_map;
use std::iter::{Iterator, Peekable};
use std::str::Chars;

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

pub fn scan<'a>(source: &'a str) -> Result<Vec<Token>, LoxError> {
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
        Some(e) => Err(e),
        None => Ok(tokens),
    }
}

struct Scanner<'a> {
    iter: Peekable<Chars<'a>>,
    line: u32,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            iter: source.chars().peekable(),
            line: 1,
        }
    }

    fn can_scan(&mut self) -> bool {
        self.peek().is_some()
    }

    fn scan_token(&mut self) -> Result<Option<Token>, LoxError> {
        use TokenType::*;
        let Some(value) = self.next() else {
            return Ok(None);
        };

        match value {
            '(' => Ok(Some(Token::new(LEFTPAREN, self.line))),
            ')' => Ok(Some(Token::new(RIGHTPAREN, self.line))),
            '{' => Ok(Some(Token::new(LEFTBRACE, self.line))),
            '}' => Ok(Some(Token::new(RIGHTBRACE, self.line))),
            ',' => Ok(Some(Token::new(COMMA, self.line))),
            '.' => Ok(Some(Token::new(DOT, self.line))),
            '-' => Ok(Some(Token::new(MINUS, self.line))),
            '+' => Ok(Some(Token::new(PLUS, self.line))),
            ';' => Ok(Some(Token::new(SEMICOLON, self.line))),
            '*' => Ok(Some(Token::new(STAR, self.line))),
            '!' => self.add_operator(Operator::BANG),
            '=' => self.add_operator(Operator::EQUAL),
            '<' => self.add_operator(Operator::LESS),
            '>' => self.add_operator(Operator::GREATER),
            '/' => {
                if self.next_if_eq(&'/').is_some() {
                    self.advance_while(|x| *x != '\n');
                    Ok(None)
                } else {
                    Ok(Some(Token::new(SLASH, self.line)))
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

    fn add_operator(&mut self, type_: Operator) -> Result<Option<Token>, LoxError> {
        if self.next_if_eq(&'=').is_some() {
            Ok(Some(Token::new(
                match type_ {
                    Operator::BANG => TokenType::BANGEQUAL,
                    Operator::EQUAL => TokenType::EQUALEQUAL,
                    Operator::LESS => TokenType::LESSEQUAL,
                    Operator::GREATER => TokenType::GREATEREQUAL,
                },
                self.line,
            )))
        } else {
            Ok(Some(Token::new(type_.into(), self.line)))
        }
    }

    fn handle_string(&mut self) -> Result<Option<Token>, LoxError> {
        let slice = self.advance_and_get_literal(|x| *x != '"');

        if self.iter.peek().is_none() {
            UntermString.error(self.line);
            Err(LoxError::UntermString)
        } else {
            // the closing '"'
            self.iter.next();
            Ok(Some(Token::new_string(slice, self.line)))
        }
    }

    fn handle_complex_lexeme(&mut self, val: char) -> Result<Option<Token>, LoxError> {
        if val.is_ascii_digit() {
            self.handle_number(val)
        } else if val.is_ascii_alphanumeric() {
            self.handle_identifier(val)
        } else {
            Err(Syntax.error(self.line))
        }
    }
    fn handle_number(&mut self, val: char) -> Result<Option<Token>, LoxError> {
        let mut cur_str = String::from(val);
        cur_str.push_str(&self.advance_and_get_literal(char::is_ascii_digit));

        if self.iter.next_if(|x| *x == '.').is_some() {
            cur_str.push('.');
            cur_str.push_str(&self.advance_and_get_literal(char::is_ascii_digit));
        }
        Ok(Some(Token::new_number(cur_str, self.line)?))
    }

    fn handle_identifier(&mut self, val: char) -> Result<Option<Token>, LoxError> {
        let mut cur_str = String::from(val);
        cur_str.push_str(&self.advance_and_get_literal(char::is_ascii_alphanumeric));
        let str_pointer = cur_str.as_str();
        let token;
        if let Some(tok) = KEYWORDS.get(str_pointer) {
            token = tok.clone();
        } else {
            token = TokenType::IDENTIFIER(ByteYarn::from_string(cur_str));
        }
        Ok(Some(Token::new(token, self.line)))
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

impl Into<TokenType> for Operator {
    fn into(self) -> TokenType {
        match self {
            Self::BANG => TokenType::BANG,
            Self::EQUAL => TokenType::EQUAL,
            Self::LESS => TokenType::LESS,
            Self::GREATER => TokenType::GREATER,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenType::*;

    static ONE: TokenType = NUMBER {
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

    fn compare_scan(string: &str, goal: Vec<TokenType>) {
        let scanned_tokens: Vec<_> = scan(string).unwrap().into_iter().map(|x| x.type_).collect();
        println!("{:?}", scanned_tokens);
        let scanned_tokens = scan(string).unwrap().into_iter().map(|x| x.type_);
        for (scanned_token, goal_token) in std::iter::zip(scanned_tokens, goal) {
            assert_eq!(scanned_token, goal_token)
        }
    }
}
