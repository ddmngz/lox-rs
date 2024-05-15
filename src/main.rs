mod lox;

use lox::error::LoxError;
use lox::Lox;
use std::env;
use std::cmp::Ordering;


fn main() -> Result<(), LoxError> {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    match args.len().cmp(&2){
        Ordering::Less => lox.run_prompt(),
        Ordering::Equal => lox.run_file(&args[1]), 
        Ordering::Greater => Err(LoxError::usage()),
    }

}
