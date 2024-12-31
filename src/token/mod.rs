use std::fmt;
use strum_macros::EnumDiscriminants;
pub type SmartString = smartstring::SmartString<smartstring::Compact>;
/// Every Possible Type of Token
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Default, PartialEq, EnumDiscriminants)]
#[strum_discriminants(name(TokenDiscriminant))]
pub enum Token {
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
    IDENTIFIER(SmartString),
    /// String and Number store their own
    /// Internal representation
    STRING(SmartString),
    NUMBER(f64),
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

#[derive(Clone, Debug)]
pub struct Identifier(SmartString);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<Token> for Identifier {
    type Error = ();

    fn try_from(token: Token) -> std::result::Result<Self, Self::Error> {
        if let Token::IDENTIFIER(i) = token {
            Ok(Self(i))
        } else {
            Err(())
        }
    }
}

impl From<Identifier> for Token {
    fn from(ident: Identifier) -> Self {
        Self::IDENTIFIER(ident.0)
    }
}

impl From<SmartString> for Identifier {
    fn from(string: SmartString) -> Self {
        Self(string)
    }
}

impl Into<SmartString> for Identifier {
    fn into(self) -> SmartString {
        self.0
    }
}

impl PartialEq<Token> for TokenDiscriminant {
    fn eq(&self, other: &Token) -> bool {
        let discrim: TokenDiscriminant = other.into();
        *self == discrim
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::STRING(yarn) => write!(f, "STRING(\"{}\")", yarn),
            Self::NUMBER(value) => {
                write!(f, "NUMBER({value})")
            }
            _ => write!(f, "{:?}", &self),
        }
    }
}

// operators that can have an equals after them
pub enum Operator {
    BANG,
    EQUAL,
    LESS,
    GREATER,
}

impl From<Operator> for Token {
    fn from(operator: Operator) -> Self {
        match operator {
            Operator::BANG => Self::BANG,
            Operator::EQUAL => Self::EQUAL,
            Operator::LESS => Self::LESS,
            Operator::GREATER => Self::GREATER,
        }
    }
}

impl Operator {
    pub fn into_equal(self) -> Token {
        match self {
            Self::BANG => Token::BANGEQUAL,
            Self::EQUAL => Token::EQUALEQUAL,
            Self::LESS => Token::LESSEQUAL,
            Self::GREATER => Token::GREATEREQUAL,
        }
    }
}

impl TryFrom<char> for Token {
    type Error = ();
    fn try_from(character: char) -> Result<Self, ()> {
        match character {
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
            ' ' | '\r' | '\t' | '\n' => Err(()), // could squish with under but this is more
            // explicit
            _ => Err(()),
        }
    }
}

impl Token {
    pub fn from_keyword(keyword: &str) -> Option<Self> {
        match keyword {
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
