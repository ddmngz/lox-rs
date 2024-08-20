use std::fs::File;
use std::io::{stdin, Read};

pub mod error;
pub mod parser;
mod scanner;
pub mod token;

use error::LoxError;
use parser::ast_printer::AstPrinter;
use parser::Parser;
use parser::interpreter::Interpreter;
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
        let scan = Scanner::new(message);
        let tokens: Vec<Token> = scan.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        let expr = parser.parse()?;

        let interpreter = Interpreter{};
        
        match interpreter.interpret(&expr){
            Ok(()) => Ok(()),
            Err(e) => Err(LoxError::RuntimeError(e)),
        }
        /*
        let mut printer = AstPrinter::new();
        println!("{}", printer.print(expr));
        */

    }
}
