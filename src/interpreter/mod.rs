mod error;
use crate::syntax_trees::statement::Statement;
pub use error::RuntimeError;

pub struct Interpreter {}
pub type Result<T> = std::result::Result<T, RuntimeError>;

pub fn interpret(statements: Vec<impl Statement>) -> Result<()> {
    let mut _interpreter = Interpreter {};
    for statement_ in statements {
        statement_.execute()?;
    }

    Ok(())
}
