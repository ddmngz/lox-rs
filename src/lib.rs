use std::fs::File;
use std::io::{stdin, Read};

pub mod error;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod token;

use error::LoxError;
use interpreter::Interpreter;
use parser::Parser;
use token::Token;

pub fn run_file(file_name: &str) -> Result<(), LoxError> {
    let mut file = File::open(file_name).unwrap();
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();
    run(contents)
}

pub fn run_prompt() -> Result<(), LoxError> {
    let mut contents = String::new();
    loop {
        print!("> ");
        if stdin().read_line(&mut contents).is_ok_and(|x| x == 0) {
            return Ok(());
        }
        run(contents)?;
        contents = String::new();
    }
}

fn run(message: String) -> Result<(), LoxError> {
    let tokens: Vec<Token> = scanner::scan(&message)?;
    let mut parser = Parser::new(tokens);
    let expr = parser.parse()?;
    let interpreter = Interpreter {};

    /*
    match interpreter.interpret(&expr) {
        Ok(()) => Ok(()),
        Err(e) => Err(LoxError::RuntimeError(e)),
    }*/
    todo!()
}
