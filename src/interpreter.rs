mod interpreter;
pub mod error;

use interpreter::Interpreter;
use super::parser::ast::statement::Statement;
pub use error::RuntimeError;

pub type Result<T> = std::result::Result<T, RuntimeError>;

pub fn interpret(statements: impl Iterator<Item = Statement>) -> Result<()>{
    let interpreter = Interpreter{};
    for statement_ in statements{
        interpreter.execute(statement_)?;
    }

    Ok(())
}
