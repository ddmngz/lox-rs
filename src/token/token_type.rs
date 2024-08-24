use byteyarn::ByteYarn;
use std::fmt;
/// Every Possible Type of Token
#[allow(clippy::upper_case_acronyms, dead_code)]
#[derive(Debug, Clone, Default, PartialEq)]
pub enum TokenType {
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

impl fmt::Display for TokenType {
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
