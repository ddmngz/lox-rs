use super::expression::Expression;
use crate::token::Identifier;
use crate::token::SmartString;
use std::fmt;
#[derive(Clone, Debug)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    Var {
        name: SmartString,
        initializer: Option<Expression>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
    Function(Function),
    If {
        condition: Expression,
        then: Box<Statement>,
        else_case: Option<Box<Statement>>,
    },
    Block(Vec<Statement>),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Expression(e) => write!(f, "{}", e),
            Self::Print(e) => write!(f, "print {}", e),
            Self::Var {
                name,
                initializer: Some(initializer),
            } => write!(f, "var {name} = {initializer}"),
            Self::Var {
                name,
                initializer: None,
            } => write!(f, "var {name}"),
            Self::While { condition, body } => write!(f, "while {condition} {{{body}}}"),
            Self::Function(fun) => write!(f, "{}", fun),
            Self::If {
                condition,
                then,
                else_case: Some(else_case),
            } => {
                write!(f, "if {condition} {{\n{then}\n}}else{{\n{else_case}\n}})")
            }
            Self::If {
                condition,
                then,
                else_case: None,
            } => write!(f, "if {condition} {{\n{then}\n}}"),
            Self::Block(block) => format_body(f, &block),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: Identifier,
    pub params: Vec<Identifier>,
    pub body: Vec<Statement>,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(", self.name)?;
        format_params(f, &self.params)?;
        format_body(f, &self.body)
    }
}

fn format_params(f: &mut fmt::Formatter, args: &[Identifier]) -> fmt::Result {
    let Some(last) = args.last() else {
        return write!(f, ")");
    };

    for arg in &args[..args.len() - 1] {
        write!(f, "{}, ", arg)?;
    }
    write!(f, "{})", last)
}

fn format_body(f: &mut fmt::Formatter, args: &[Statement]) -> fmt::Result {
    write!(f, "{{")?;

    for arg in args {
        writeln!(f, "{}", arg)?;
    }
    write!(f, "}}")
}
