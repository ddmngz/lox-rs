mod interpreter;

use interpreter::Interpreter;
use super::parser::ast::statement::Statement;
use crate::error::LoxRuntimeError;

pub type Result<T> = std::result::Result<T, LoxRuntimeError>;

pub fn interpret(statements: impl Iterator<Item = Statement>) -> Result<()>{
    let interpreter = Interpreter{};
    for statement_ in statements{
        interpreter.execute(statement_)?;
    }

    Ok(())
}
