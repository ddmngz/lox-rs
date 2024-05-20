use std::fs::File;
use std::io::{stdin, Read};

pub mod error;
mod parser;
mod scanner;
mod token;

use error::LoxError;
use scanner::Scanner;
use token::Token;

#[derive(Default)]
pub struct Lox {}

impl Lox {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn run_file(&mut self, file_name: &str) -> Result<(), LoxError> {
        let mut file = File::open(file_name).unwrap();
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).unwrap();
        self.run(contents)
    }

    pub fn run_prompt(&mut self) -> Result<(), LoxError> {
        let mut contents = String::new();
        loop {
            print!("> ");
            if stdin().read_line(&mut contents).is_ok_and(|x| x == 0) {
                return Ok(());
            }
            self.run(contents)?;
            contents = String::new();
        }
    }
    fn run(&mut self, message: String) -> Result<(), LoxError> {
        let message = Box::leak(message.into_boxed_str());
        let mut scan: Scanner = Scanner::new(message);
        let tokens: Vec<Token> = scan.scan_tokens()?;
        for token in tokens {
            println!("{}", token);
        }
        Ok(())
    }
}
