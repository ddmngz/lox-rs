mod error;
mod scanned_token;
pub use error::ScanningError;

pub use crate::token::Token;
pub use scanned_token::ScannedToken;

use crate::token::Operator;
use crate::token::SmartString;
use std::str::Chars;

type Result<T> = std::result::Result<T, ScanningError>;

pub enum ScanResult {
    Token(Token),
    Operator(Operator),
    SLASH,
    STRING,
    NUMBER(char),
    IDENTIFIER(char),
    WHITESPACE,
    NEWLINE,
    INVALID,
}

pub fn scan(source: Box<str>) -> Result<Vec<ScannedToken>> {
    let mut tokens = Vec::with_capacity(source.len());
    let mut err = None;
    let mut line = 1;
    let mut iter = source.chars();
    while let Some(char) = iter.next() {
        match scan_token(char) {
            ScanResult::Token(t) => tokens.push(token(t, line)),
            ScanResult::Operator(o) => tokens.push(token(operator(o, &source), line)),
            ScanResult::NEWLINE => line += 1,
            ScanResult::NUMBER(number) => {
                tokens.push(token(handle_number(&mut iter, number)?, line));
            }
            ScanResult::IDENTIFIER(letter) => {
                tokens.push(token(handle_identifier(&mut iter, letter), line))
            }
            ScanResult::SLASH => {
                if let Some('/') = peek(&iter) {
                    advance_while(&mut iter, |&x| x != '\n');
                } else {
                    tokens.push(token(Token::SLASH, line));
                }
            }
            ScanResult::STRING => {
                let remaining = iter.as_str().len();
                match slice_while(&mut iter, |&x| x != '"') {
                    None => tokens.push(token(Token::STRING(SmartString::new()), line)),
                    Some(slice) if slice.len() == remaining => {
                        ScanningError::UntermString.error(line);
                        err = Some(ScanningError::UntermString);
                    }
                    Some(slice) => tokens.push(string(slice, line)),
                }
                iter.next();
                // if it's "" or "*" go past the last quote, otherwise it'll just be None
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

fn string(source: &str, line: u32) -> ScannedToken {
    let token = handle_string(source);
    ScannedToken::new(token, line)
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
        '>' => ScanResult::Operator(Operator::GREATER),
        '<' => ScanResult::Operator(Operator::LESS),
        '!' => ScanResult::Operator(Operator::BANG),
        '/' => ScanResult::SLASH,
        '"' => ScanResult::STRING,
        number if number.is_ascii_digit() => ScanResult::NUMBER(number),
        letter if letter.is_alphabetic() => ScanResult::IDENTIFIER(letter),
        _ => ScanResult::INVALID,
    }
}

fn token(token: Token, line: u32) -> ScannedToken {
    ScannedToken::new(token, line)
}

fn handle_identifier(iter: &mut Chars, letter: char) -> Token {
    let mut literal = SmartString::new();
    literal.push(letter);
    if let Some(slice) = slice_while(iter, char::is_ascii_alphabetic) {
        literal.push_str(slice);
    }
    if let Some(keyword) = Token::from_keyword(&literal) {
        keyword
    } else {
        Token::IDENTIFIER(literal)
    }
}

fn handle_number(iter: &mut Chars, number: char) -> Result<Token> {
    let mut lexeme = SmartString::new();
    lexeme.push(number);
    let base = &iter.as_str();
    let mut end = advance_while(iter, char::is_ascii_digit);
    if let Some('.') = peek(iter) {
        iter.next();
        end += 1;

        end += advance_while(iter, char::is_ascii_digit);
    }
    lexeme.push_str(&base[..end]);
    let value: f64 = lexeme.parse()?;
    Ok(Token::NUMBER(value))
}

fn advance_while<F>(iter: &mut Chars, f: F) -> usize
where
    F: Fn(&char) -> bool,
{
    let mut amount = 0;
    while peek(iter).is_some_and(|x| f(&x)) {
        amount += 1;
        iter.next();
    }
    amount
}

fn slice_while<'a, F>(iter: &mut Chars<'a>, f: F) -> Option<&'a str>
where
    F: Fn(&char) -> bool,
{
    let start = iter.as_str();
    let end = advance_while(iter, f);

    if end == 0 {
        None
    } else if end == start.len() {
        Some(start)
    } else {
        Some(&start[..end])
    }
}

fn peek(iter: &Chars) -> Option<char> {
    iter.as_str().chars().next()
}

fn handle_string(source: &str) -> Token {
    let string = if source.len() > smartstring::MAX_INLINE {
        // in case I want to at some point instead try to make it take allocation of that section
        SmartString::from(source)
    } else {
        SmartString::from(source)
    };
    Token::STRING(string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn scan_equation() {
        compare_scan("1+1", vec![NUMBER(1.0), PLUS, NUMBER(1.0)])
    }

    #[test]
    fn scan_identifier() {
        compare_one("woaw", IDENTIFIER("woaw".into()))
    }

    #[test]
    fn scan_quotes() {
        compare_scan("\"hiiii\"", vec![STRING("hiiii".into())])
    }

    #[test]
    fn scan_empty() {
        compare_one("\"\"", STRING(SmartString::new()));
    }

    #[test]
    fn skip_whitespace() {
        assert!(scan("\t \n\n \t \r".into()).is_ok_and(|x| x.is_empty()))
    }

    #[test]
    fn scan_parens() {
        compare_scan(
            "({<>)}",
            vec![LEFTPAREN, LEFTBRACE, LESS, GREATER, RIGHTPAREN, RIGHTBRACE],
        )
    }

    #[test]
    fn scan_integer() {
        compare_one("123456", NUMBER(123456.0))
    }

    #[test]
    fn scan_decimal() {
        compare_one("123456.123456", NUMBER(123456.123456))
    }

    #[test]
    fn scan_paren_equation() {
        compare_scan(
            "(1+1)",
            vec![LEFTPAREN, NUMBER(1.0), PLUS, NUMBER(1.0), RIGHTPAREN],
        )
    }

    #[test]
    fn ignore_comments() {
        compare_scan(
            "// this is a comment should be ignored \n 123",
            vec![NUMBER(123.0)],
        );
    }

    #[test]
    fn identifier() {
        compare_one("ababa", IDENTIFIER("ababa".into()))
    }

    #[test]
    fn comparison() {
        compare_one("<", LESS)
    }

    #[test]
    fn comparison_equal() {
        compare_one("<=", LESSEQUAL)
    }

    #[test]
    fn unterm_string() {
        assert!(
            scan("\"unterminated moment".into()).is_err_and(|e| e == ScanningError::UntermString)
        )
    }

    #[test]
    fn unterm_statement() {
        compare_scan(
            "print \"hello world\";",
            vec![PRINT, STRING("hello world".into()), SEMICOLON],
        );
    }

    fn compare_one(string: &str, target: Token) {
        let string: Box<str> = string.into();
        let token = scan(string).unwrap()[0].type_.clone();
        assert_eq!(token, target)
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
