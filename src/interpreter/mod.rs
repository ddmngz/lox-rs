mod error;
use crate::syntax_trees::expression::Expr;
use crate::syntax_trees::statement::Statement;
pub use error::RuntimeError;

pub struct Interpreter {}
pub type Result<T> = std::result::Result<T, RuntimeError>;

pub fn interpret(statements: Vec<Statement>) -> Result<()> {
    let mut _interpreter = Interpreter {};
    for statement_ in statements {
        execute(statement_)?;
    }

    Ok(())
}

fn execute(statement: Statement) -> Result<()> {
    match statement {
        Statement::Expression(statement) => {
            statement.evaluate()?;
            Ok(())
        }
        Statement::Print(statement) => {
            println!("{}", statement.evaluate()?);
            Ok(())
        }
    }
}
