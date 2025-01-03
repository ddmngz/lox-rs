use std::fs::File;
use std::io::{stdin, stdout, Read, Write};

pub mod error;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod syntax_trees;
pub mod token;

use interpreter::Environment;

use error::Error;
use parser::Parser;

pub fn run_file(file_name: &str) -> Result<(), Error> {
    let mut file = File::open(file_name).unwrap();
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();
    let contents = contents.into_boxed_str();
    let mut env = Environment::default();
    run(contents, &mut env)
}

pub fn run_prompt() -> Result<(), Error> {
    let mut workhorse = String::new();
    let mut contents: Box<str>;
    let mut environment = Environment::default();
    loop {
        print!("> ");
        stdout().flush()?;
        if stdin().read_line(&mut workhorse).is_ok_and(|x| x == 0) {
            return Ok(());
        }
        contents = workhorse.clone().into();
        if let Err(e) = run(contents, &mut environment) {
            println!("{}", e);
            stdout().flush()?;
        }
        workhorse.clear();
    }
}

fn run(code: Box<str>, env: &mut Environment) -> Result<(), Error> {
    if !validate(&code) {
        return Err(Error::NotAscii);
    };
    let tokens = scanner::scan(&code)?;
    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    match interpreter::interpret(statements, env) {
        Ok(()) => Ok(()),
        Err(e) => Err(Error::RuntimeError(e)),
    }
}

// TODO
fn validate(_code: &str) -> bool {
    true
}
