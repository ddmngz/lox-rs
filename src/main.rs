use lox::error::LoxError;
use lox::parser::ast_printer::AstPrinter;
use lox::parser::expr::{Binary, Grouping, Literal, Unary, UnaryOperator, BinaryOperator};
use lox::parser::interpreter::Interpreter;
use lox::Lox;
use std::cmp::Ordering;
use std::env;

fn _test_tree() {
    let lit_1 = Literal::float(123.0);
    let minus = UnaryOperator::MINUS;
    let times = BinaryOperator::SLASH;
    let lit_2 = Literal::float(45.67);

    let grouping = Grouping::new(lit_2);
    let unary = Unary::new(minus, lit_1);
    let binary = Binary::new(unary, times, grouping);

    let mut printer = AstPrinter {};
    let mut interpreter = Interpreter {};
    interpreter.interpret(&binary).unwrap();
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
