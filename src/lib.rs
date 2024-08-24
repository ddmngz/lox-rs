use std::fs::File;
use std::io::{stdin, Read};

pub mod error;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod token;

use error::Error;
use parser::Parser;
use token::Token;

pub fn run_file(file_name: &str) -> Result<(), Error> {
    let mut file = File::open(file_name).unwrap();
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();
    run(contents)
}

pub fn run_prompt() -> Result<(), Error> {
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

fn run(message: String) -> Result<(), Error> {
    let tokens: Vec<Token> = scanner::scan(&message)?;
    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?.into_iter();

    match interpreter::interpret(statements) {
        Ok(()) => Ok(()),
        Err(e) => Err(Error::RuntimeError(e)),
    }
}
