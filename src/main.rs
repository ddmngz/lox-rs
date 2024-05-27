mod lox;
use lox::error::LoxError;
use lox::parser::ast_printer::AstPrinter;
use lox::parser::structs::{Binary, Grouping, Literal, Unary};
use lox::token::Token;
use lox::token::TokenType;
use lox::Lox;
use std::cmp::Ordering;
use std::env;

fn _test_tree() {
    let lit_1 = Literal::float(123.0);
    let minus = Token::new(TokenType::MINUS, "-", 1);
    let times = Token::new(TokenType::STAR, "*", 1);
    let lit_2 = Literal::float(45.67);

    let grouping = Grouping::new(lit_2);
    let unary = Unary::new(minus, lit_1);
    let binary = Binary::new(unary, times, grouping);

    let mut printer = AstPrinter {};
    println!("{}", printer.print(binary));
}

fn main() -> Result<(), LoxError> {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    match args.len().cmp(&2) {
        Ordering::Less => lox.run_prompt(),
        Ordering::Equal => lox.run_file(&args[1]),
        Ordering::Greater => Err(LoxError::usage()),
    }
}
