use std::fmt;
use ascii::AsciiStr;
use ascii::AsciiChar;

/// Every Possible Type of Token
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Default, PartialEq)]
pub enum Token<'source> {
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,
    IDENTIFIER(&'source AsciiStr),
    /// String and Number store their own
    /// Internal representation
    STRING(&'source AsciiStr),
    NUMBER {
        lexeme: &'source AsciiStr,
        value: f64,
    },

    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    #[default]
    EOF,
}


impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::STRING(yarn) => write!(f, "STRING(\"{}\")", yarn.to_string()),
            Self::NUMBER { value, lexeme } => write!(
                f,
                "NUMBER(value = {value}, literal = {})",
                lexeme.to_string()
            ),
            _ => write!(f, "{:?}", &self),
        }
    }
}

// operators that can have an equals after them
enum Operator {
    BANG,
    EQUAL,
    LESS,
    GREATER,
}

impl<'a> Into<Token<'a>> for Operator {
    fn into(self) -> Token<'a>{
        match self {
            Self::BANG => Token::BANG,
            Self::EQUAL => Token::EQUAL,
            Self::LESS => Token::LESS,
            Self::GREATER => Token::GREATER,
        }
    }
}

impl<'a> Operator{
    fn into_equal(self) -> Token<'a>{
        match self {
            Self::BANG => Token::BANGEQUAL,
            Self::EQUAL => Token::EQUALEQUAL,
            Self::LESS => Token::LESSEQUAL,
            Self::GREATER => Token::GREATEREQUAL,
        }
    }
}


use crate::error::ScanningError;
impl<'source, 'lender > Token<'source>{
    // given a Ascii Slice, tries to make a Token, advancing the slice accordingly
    pub fn split_from_slice(slice: &'lender mut &'source AsciiStr) -> Option<Result<ScanResult<'source>, ScanningError>>{
        if slice.len() == 0{
            return None;
        }
        Some(match Self::try_from(slice[0]){
            Ok(token) => {
                *slice = &slice[1..];
                Ok(ScanResult::Token(token))
            },
            Err(CharHint::Newline) => {
                *slice = &slice[1..];
                Ok(ScanResult::Newline)
            },
            Err(CharHint::Whitespace) => {
                let Some(whitespace_end) = slice.into_iter().position(|&val| val != ' ')else{
                    return None;
                };
                *slice = &slice[whitespace_end..];
                return Self::split_from_slice(slice)
            },
            Err(CharHint::Incomplete) => Self::multi_char_from_slice(slice),
        })
    }

    // Makes a Token out of multi-character Lexemes (comments, ==, Strings, etc)
    fn multi_char_from_slice(slice: &'lender mut &'source AsciiStr) -> Result<ScanResult<'source>,ScanningError>{
        match slice[0].as_char(){
            '=' => Ok(ScanResult::Token(Self::operator_from_slice(Operator::EQUAL, slice))),
            '>' => Ok(ScanResult::Token(Self::operator_from_slice(Operator::LESS, slice))),
            '<' => Ok(ScanResult::Token(Self::operator_from_slice(Operator::GREATER, slice))),
            '!' => Ok(ScanResult::Token(Self::operator_from_slice(Operator::BANG, slice))),
            '/' => {Ok(
                if slice.len() >= 2 && slice[1] == '/'{
                    if let Some(position) = slice.into_iter().position(|&val| val == '\n'){
                        *slice = &slice[position..];
                        ScanResult::Newline
                    }else{
                        *slice = &slice[slice.len()-1..];
                        ScanResult::Newline
                    }
                }else{
                    ScanResult::Token(Token::SLASH)
                }
            )},
            '"' => Ok(ScanResult::Token(Self::lox_string_from_slice(slice)?)), 
            _ if slice[0].is_ascii_digit() => Ok(ScanResult::Token(Self::lox_number_from_slice(slice)?)),
            _ if slice[0].is_alphabetic() => Ok(ScanResult::Token(Self::lox_identifier_from_slice(slice))),
            _ => Err(ScanningError::Syntax),
        }
    }

    // Makes an Operator out of a slice
    fn operator_from_slice(operator:Operator, slice: &'lender mut &'source AsciiStr) -> Token<'source> {
        if slice.len() >=2 && slice[1] == '='{
            *slice = &slice[1..];
            operator.into_equal()
        }else{
            *slice = &slice[1..];
            operator.into()
        }
    }

    // Makes a lox string from a slice
    fn lox_string_from_slice(slice: &'lender mut &'source AsciiStr) -> Result<Token<'source>, ScanningError> {
        match Self::get_literal(slice,|val| *val == '"'){
            (_, true) => Err(ScanningError::UntermString),
            (literal, _) => Ok(Token::STRING(literal))
        }
    }

    fn lox_number_from_slice(slice: &'lender mut &'source AsciiStr) -> Result<Token<'source>, ScanningError> {
        let mut split_point = None;
        let mut found_dot = false;
        for (pos, val) in slice.into_iter().enumerate(){
            match val.is_ascii_digit(){
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
        let (remaining_slice, lexeme) = if let Some(point) = split_point {
            Self::split_ascii_at(slice, point)
        }else{
            Self::split_ascii_at(slice, 1)
        };

        *slice = remaining_slice;

        let value:f64 = lexeme.as_str().parse()?;

        Ok(Token::NUMBER{lexeme, value})
    }

    fn lox_identifier_from_slice(slice: &'lender mut &'source AsciiStr) -> Token<'source>{
        let (literal, _) = Self::get_literal(slice, |val| val.is_alphabetic());
        Self::IDENTIFIER(literal)
    }

    fn get_literal<F>(slice: &'lender mut &'source AsciiStr, f:F) -> (&'source AsciiStr, bool)
        where F: FnMut(&AsciiChar) -> bool
    {
        let (split_point, ate_entire_slice) = match slice.into_iter().position(f){
            Some(index) => (index, false),
            None => (slice.len()-1, true),
        };
        let (literal, remaining_slice) = Self::split_ascii_at(slice, split_point);
        *slice = remaining_slice;

        (literal, ate_entire_slice)
    }

    fn split_ascii_at(slice: &AsciiStr, at:usize) -> (&AsciiStr,&AsciiStr){
        let (first, second) = slice.as_bytes().split_at(at);
        // safe since it's previously validated ascii
        unsafe{(AsciiStr::from_ascii_unchecked(first), AsciiStr::from_ascii_unchecked(second))}
    }

}

pub enum ScanResult<'source>{
    Newline,
    Token(Token<'source>),
}

enum CharHint{
    Whitespace,
    Incomplete,
    Newline,
}

impl<'a> TryFrom<AsciiChar> for Token<'a>{
    type Error = CharHint;
    fn try_from(char:AsciiChar) -> Result<Self, CharHint>{
        match char.as_char(){
            ' ' | '\r' | '\t'  => Err(CharHint::Whitespace),
            '\n' => {
                Err(CharHint::Newline)
            },
            '(' => Ok(Self::LEFTPAREN),
            ')' => Ok(Self::RIGHTPAREN),
            '{' => Ok(Self::LEFTBRACE),
            '}' => Ok(Self::RIGHTBRACE),
            ',' => Ok(Self::COMMA),
            '.' => Ok(Self::DOT),
            '-' => Ok(Self::MINUS),
            '+' => Ok(Self::PLUS),
            ';' => Ok(Self::SEMICOLON),
            '*' => Ok(Self::STAR),
            _ => Err(CharHint::Incomplete),
        }
    }
}

impl Token<'_> {
    pub fn from_keyword(keyword: &str) -> Option<Self>{
        match keyword{
            "and" => Some(Self::AND),
            "class" => Some(Self::CLASS),
            "else" => Some(Self::ELSE),
            "false" => Some(Self::FALSE),
            "for" => Some(Self::FOR),
            "fun" => Some(Self::FUN),
            "if" => Some(Self::IF),
            "nil" => Some(Self::NIL),
            "or" => Some(Self::OR),
            "print" => Some(Self::PRINT),
            "return" => Some(Self::RETURN),
            "super" => Some(Self::SUPER),
            "this" => Some(Self::THIS),
            "true" => Some(Self::TRUE),
            "var" => Some(Self::VAR),
            "while" => Some(Self::WHILE),
            _ => None,
        }
    }
}
