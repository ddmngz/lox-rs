
use std::fs::File;
use std::io::{stdin, Read};

pub mod error;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod interpreter;

use error::LoxError;
use parser::Parser;
use interpreter::Interpreter;
use scanner::Scanner;
use token::Token;
use parser::ast_printer::AstPrinter;


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
        let mut printer = AstPrinter::new();
    
        let ret = match interpreter.interpret(&expr){
            Ok(()) => Ok(()),
            Err(e) => Err(LoxError::RuntimeError(e)),
        };

        println!("{}", printer.print(expr));
        ret
    
    }
}
