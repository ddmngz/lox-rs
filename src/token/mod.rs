use byteyarn::ByteYarn;
use std::fmt;
use phf::phf_map;

/// Every Possible Type of Token
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Default, PartialEq)]
pub enum Token{
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
    IDENTIFIER(ByteYarn),
    /// String and Number store their own
    /// Internal representation
    STRING(ByteYarn),
    NUMBER {
        lexeme: ByteYarn,
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

impl fmt::Display for Token {
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

impl Token {
    pub fn from_keyword(keyword: &str) -> Option<Self>{
        KEYWORDS.get(keyword).cloned()
    }
}
static KEYWORDS: phf::Map<&'static str, Token> = {
    use Token::*;
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



