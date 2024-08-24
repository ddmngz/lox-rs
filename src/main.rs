use lox::error::Error;
use std::cmp::Ordering;
use std::env;



fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    match args.len().cmp(&2) {
        Ordering::Less => lox::run_prompt(),
        Ordering::Equal => lox::run_file(&args[1]),
        Ordering::Greater => Err(Error::Usage),
    }
}
