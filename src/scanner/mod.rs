mod error;
mod scanned_token;
pub use error::ScanningError;

pub use crate::token::Token;
pub use scanned_token::ScannedToken;

use crate::token::Operator;
use crate::token::SmartString;
use ouroboros::self_referencing;
use std::str::CharIndices;

use std::mem::ManuallyDrop;

type Result<T> = std::result::Result<T, ScanningError>;

pub enum ScanResult {
    Token(Token),
    Operator(Operator),
    SLASH,
    STRING,
    NUMBER,
    IDENTIFIER,
    WHITESPACE,
    NEWLINE,
    INVALID,
}

pub fn scan(source: Box<str>) -> Result<Vec<ScannedToken>> {
    let mut tokens = Vec::with_capacity(source.len());
    let mut err = None;
    let mut line = 1;
    let mut iter = source.char_indices();
    while let Some((pos, res)) = iter.next() {
        match scan_token(res) {
            ScanResult::Token(t) => tokens.push(token(t, line)),
            ScanResult::Operator(o) => tokens.push(token(operator(o, &source), line)),
            ScanResult::NEWLINE => line += 1,
            ScanResult::NUMBER => {
                tokens.push(token(handle_number(&mut iter)?, line));
            }
            ScanResult::IDENTIFIER => tokens.push(token(handle_identifier(&mut iter), line)),
            ScanResult::SLASH => {
                if let Some('/') = peek(&iter, pos) {
                    while iter.next().is_some_and(|(_, x)| x != '\n') {}
                } else {
                    tokens.push(token(Token::SLASH, line));
                }
            }
            ScanResult::STRING => {
                if let Some(end) = iter.position(|(_, x)| x == '"') {
                    tokens.push(token(handle_string(&source, pos, end), line));
                } else {
                    ScanningError::UntermString.error(line);
                    err = Some(ScanningError::UntermString);
                }
            }
            ScanResult::WHITESPACE => {}
            ScanResult::INVALID => {
                ScanningError::Syntax.error(line);
                err = Some(ScanningError::Syntax);
            }
        }
    }
    match err {
        Some(error) => Err(error),
        None => Ok(tokens),
    }
}

fn operator(operator: Operator, source: &str) -> Token {
    if source.len() >= 2 && source.chars().nth(1).is_some_and(|x| x == '=') {
        operator.into_equal()
    } else {
        operator.into()
    }
}

fn scan_token(char: char) -> ScanResult {
    match char {
        '(' => ScanResult::Token(Token::LEFTPAREN),
        ')' => ScanResult::Token(Token::RIGHTPAREN),
        '{' => ScanResult::Token(Token::LEFTBRACE),
        '}' => ScanResult::Token(Token::RIGHTBRACE),
        ',' => ScanResult::Token(Token::COMMA),
        '.' => ScanResult::Token(Token::DOT),
        '-' => ScanResult::Token(Token::MINUS),
        '+' => ScanResult::Token(Token::PLUS),
        ';' => ScanResult::Token(Token::SEMICOLON),
        '*' => ScanResult::Token(Token::STAR),
        ' ' | '\r' | '\t' => ScanResult::WHITESPACE,
        '\n' => ScanResult::NEWLINE,
        '=' => ScanResult::Operator(Operator::EQUAL),
        '>' => ScanResult::Operator(Operator::LESS),
        '<' => ScanResult::Operator(Operator::GREATER),
        '!' => ScanResult::Operator(Operator::BANG),
        '/' => ScanResult::SLASH,
        '"' => ScanResult::STRING,
        _ if char.is_ascii_digit() => ScanResult::NUMBER,
        _ if char.is_alphabetic() => ScanResult::IDENTIFIER,
        _ => ScanResult::INVALID,
    }
}

fn token(token: Token, line: u32) -> ScannedToken {
    ScannedToken::new(token, line)
}

fn handle_identifier(iter: &mut CharIndices) -> Token {
    let base = iter.as_str();
    let mut end = 1;
    while iter.next().is_some_and(|(_, x)| x.is_ascii_alphabetic()) {
        end += 1;
    }
    Token::IDENTIFIER(SmartString::from(&base[..end]))
}

fn handle_number(iter: &mut CharIndices) -> Result<Token> {
    let base = iter.as_str();
    let mut end = 1;
    while iter.next().is_some_and(|(_, x)| x.is_ascii_digit()) {
        end += 1;
    }
    if let Some('.') = base.chars().nth(end) {
        while iter.next().is_some_and(|(_, x)| x.is_ascii_digit()) {
            end += 1;
        }
    }
    let slice = &base[..end];
    let value: f64 = slice.parse()?;
    let lexeme = SmartString::from(slice);
    Ok(Token::NUMBER { lexeme, value })
}

fn peek(iter: &CharIndices, pos: usize) -> Option<char> {
    iter.as_str().chars().nth(pos + 1)
}

fn handle_string(source: &str, beginning: usize, end: usize) -> Token {
    let string = if end - beginning > smartstring::MAX_INLINE {
        // in case I want to at some point instead try to make it take allocation of that section
        SmartString::from(&source[beginning..end])
    } else {
        SmartString::from(&source[beginning..end])
    };
    Token::STRING(string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    fn one() -> Token {
        NUMBER {
            lexeme: "1".into(),
            value: 1.0,
        }
    }

    #[test]
    fn scan_equation() {
        compare_scan("1+1", vec![one(), PLUS, one()])
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
        compare_scan("(1+1)", vec![LEFTPAREN, one(), PLUS, one(), RIGHTPAREN])
    }

    fn compare_scan(string: &str, goal: Vec<Token>) {
        let string: Box<str> = string.into();
        let scanned_tokens: Vec<_> = scan(string.clone())
            .unwrap()
            .into_iter()
            .map(|x| x.type_)
            .collect();
        println!("{:?}", scanned_tokens);
        let scanned_tokens = scan(string).unwrap().into_iter().map(|x| x.type_);
        for (scanned_token, goal_token) in std::iter::zip(scanned_tokens, goal) {
            assert_eq!(scanned_token, goal_token)
        }
    }
}
