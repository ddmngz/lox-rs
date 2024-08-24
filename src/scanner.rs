mod scanner;
pub mod error;
pub mod token;

pub use error::ScanningError;
pub use token::Token;

use crate::error::Error;
use scanner::Scanner;

pub fn scan<'a>(source: &'a str) -> Result<Vec<Token>, Error> {
    let mut scanner = Scanner::new(source);
    let mut err = None;
    let mut tokens = Vec::with_capacity(source.len());
    while scanner.can_scan() {
        match scanner.scan_token() {
            Ok(Some(token)) => tokens.push(token),
            Err(e) => err = Some(e),
            Ok(None) => {}
        }
    }

    match err {
        Some(e) => Err(e.into()),
        None => Ok(tokens),
    }
}


