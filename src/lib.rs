use std::fs::File;
use std::io::{stdin,stdout, Read, Write};

pub mod error;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod syntax_trees;

use error::Error;
use parser::Parser;

pub fn run_file(file_name: &str) -> Result<(), Error> {
    let mut file = File::open(file_name).unwrap();
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();
    run(&contents)
}

pub fn run_prompt() -> Result<(), Error> {
    let mut contents = String::new();
    loop {
        print!("> ");
        stdout().flush()?;
        if stdin().read_line(&mut contents).is_ok_and(|x| x == 0) {
            return Ok(());
        }
        run(&contents)?;
        contents.clear();
    }
}

fn run(message: &str) -> Result<(), Error> {
    let tokens = scanner::scan(&message)?;
    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?.into_iter();

    match interpreter::interpret(statements) {
        Ok(()) => Ok(()),
        Err(e) => Err(Error::RuntimeError(e)),
    }
}
