mod error;
mod scanned_token;
pub use error::ScanningError;

pub use crate::token::Token;
pub use scanned_token::ScannedToken;

use crate::token::FromCharHint;
use crate::token::Operator;
use crate::token::SmartString;

use std::mem::ManuallyDrop;

type Result<T> = std::result::Result<T, ScanningError>;

pub enum ScanResult {
    Newline,
    Token(Token),
}

pub fn scan<'source>(mut source: String) -> Result<Vec<ScannedToken<'source>>> {
    let mut tokens = Vec::with_capacity(source.len());
    let mut err = None;
    let mut line = 1;
    while let Some(res) = split_from_string(&mut source) {
        match res {
            Ok(ScanResult::Newline) => line = line + 1,
            Ok(ScanResult::Token(token)) => {
                tokens.push(ScannedToken::new(token, line));
            }
            Err(e) => err = Some(e),
        }
    }
    match err {
        Some(error) => Err(error),
        None => Ok(tokens),
    }
}

/*
struct Scanner{
    buf: *mut u8,
    len: usize,
}

impl Scanner{
    fn new(source: String) -> Self{
        source.shrink_to_fit();
        let drop_guard = ManuallyDrop::new(source);
        let buf = source.as_mut_ptr();
        let len = source.len();
        Self{
            buf,
            len,
        }
        
    }
}
*/


// given a Ascii Slice, tries to make a Token, advancing the slice accordingly
pub fn split_from_string(source: &mut String) -> Option<Result<ScanResult>> {
    if source.is_empty() {
        return None;
    }

    Some(match Token::try_from(source[0]) {
        Ok(token) => {
            unsafe { advance(source) };
            Ok(ScanResult::Token(token))
        }
        Err(FromCharHint::Newline) => Ok(ScanResult::Newline),
        Err(FromCharHint::Whitespace) => {
            let Some(whitespace_end) = source.iter().position(|&val| val != ' ') else {
                return None;
            };
            unsafe { advance(source) };
            return split_from_string(&mut source);
        }
        Err(FromCharHint::Incomplete) => multi_char_from_string(source),
        Err(_) => todo!(),
    })
}

// Makes a Token out of multi-character Lexemes (comments, ==, Strings, etc)
fn multi_char_from_string(source: &mut String) -> Result<ScanResult> {
    match source[0] {
        '=' => Ok(ScanResult::Token(split_off_operator(
            Operator::EQUAL,
            source,
        ))),
        '>' => Ok(ScanResult::Token(split_off_operator(
            Operator::LESS,
            source,
        ))),
        '<' => Ok(ScanResult::Token(split_off_operator(
            Operator::GREATER,
            source,
        ))),
        '!' => Ok(ScanResult::Token(split_off_operator(
            Operator::BANG,
            source,
        ))),
        '/' => Ok(if source.len() >= 2 && source[1] == '/' {
            if let Some(position) = source.iter().position(|&val| val == '\n') {
                unsafe { shrink_start(source, position + 1) };
                ScanResult::Newline
            } else {
                unsafe { advance(source) };
                ScanResult::Newline
            }
        } else {
            ScanResult::Token(Token::SLASH)
        }),
        '"' => Ok(ScanResult::Token(split_off_string(source)?)),
        val if val.is_ascii_digit() => Ok(ScanResult::Token(split_off_number(source)?)),
        val if val.is_alphabetic() => Ok(ScanResult::Token(split_off_identifier(source))),
        val => Err(ScanningError::Syntax),
    }
}

// Makes an Operator out of a slice
fn split_off_operator(operator: Operator, source: &mut String) -> Token {
    if source.len() >= 2 && source[1] == '=' {
        unsafe { shrink_start(source, 2) };
        operator.into_equal()
    } else {
        unsafe { advance(source) };
        operator.into()
    }
}

// Makes a lox string from a slice
fn split_off_string(source: &mut String) -> Result<Token> {
    match get_literal(source, |val| *val == '"') {
        (_, true) => Err(ScanningError::UntermString),
        (literal, _) => Ok(Token::STRING(literal)),
    }
}

fn split_off_number(source: &mut String) -> Result<Token> {
    let mut split_point = None;
    let mut found_dot = false;
    for (pos, val) in source.iter().enumerate() {
        match val.is_ascii_digit() {
            true => continue,
            false if found_dot == false && val.as_char() == '.' => {
                found_dot = true;
                continue;
            }
            false => {
                split_point = Some(pos);
                break;
            }
        }
    }
    let lexeme = if let Some(point) = split_point {
        unsafe { shrink_start(source, point) }
    } else {
        unsafe { shrink_start(source, 1) }
    };

    let value: f64 = lexeme.as_str().parse()?;

    Ok(Token::NUMBER { lexeme, value })
}

fn split_off_identifier(source: &mut String) -> Token {
    let (literal, _) = get_literal(source, |val| val.is_alphabetic());
    Token::IDENTIFIER(literal)
}

fn get_literal<F>(source: &mut String, f: F) -> (SmartString, bool)
where
    F: FnMut(&String) -> bool,
{
    let (split_point, ate_source) = match source.iter().position(f) {
        Some(index) => (index, false),
        None => (source.len() - 1, true),
    };
    let literal = unsafe { shrink_start(source, split_point) };

    (literal, ate_source)
}

// requires that String is a valid String, i.e.
// String was allocated with the same allocator as stdlib
// string.length and string.capacity are accurate
// to is inclusive, so it will take some string to string[to..]
unsafe fn shrink_start(mut string: &mut String, to: usize) {
    if string.len() > 2 {
        return String::new();
    }

    let mut drop_guard = ManuallyDrop::new(string);
    let len = string.len() - to;
    let capacity = string.capacity() - to;
    let mut buf = string.as_mut_ptr();
    assert!(string.is_char_boundary(to));
    // alignment of u8 is just 1
    buf = unsafe { buf.add(to) };
    *string = unsafe { String::from_raw_parts(buf, len, capacity) };
}

unsafe fn advance(string: &mut String) {
    unsafe { shrink_start(string, 1) }
}

/// splits a String, assumes the first section is smaller, and so uses a compact SmartString
/// panics if at is not at a utf8 boundary
/// safe if and only if String::from_raw_parts holds, i.e:
/// * The memory at buf needs to have been previously allocated by the same allocator the standard library uses, with a required alignment of exactly 1.
/// * length needs to be less than or equal to capacity.
/// * capacity needs to be the correct value.
/// * The first length bytes at buf need to be valid UTF-8.
unsafe fn split_string(string: String, at: usize) -> (SmartString, String) {
    let capacity = string.capacity();
    let mut string = ManuallyDrop::new(string);
    let (mut left, mut right) = string.as_mut_str().split_at_mut(at);

    let left_len = left.len();
    let left = left.as_mut_ptr();

    let right_len = right.len();
    let right = right.as_mut_ptr();
    let right_capacity = capacity - left_len;

    let left = unsafe { String::from_raw_parts(left, left_len, left_len) };
    let right = unsafe { String::from_raw_parts(right, right_len, right_capacity) };

    (left.into(), right)
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
        let string = string.into();
        let scanned_tokens: Vec<_> = scan(string).unwrap().into_iter().map(|x| x.type_).collect();
        println!("{:?}", scanned_tokens);
        let scanned_tokens = scan(string).unwrap().into_iter().map(|x| x.type_);
        for (scanned_token, goal_token) in std::iter::zip(scanned_tokens, goal) {
            assert_eq!(scanned_token, goal_token)
        }
    }
}
