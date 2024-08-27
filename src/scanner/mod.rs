mod error;
mod scanned_token;
pub use error::ScanningError;

pub use crate::token::Token;
pub use scanned_token::ScannedToken;

use ascii::AsciiStr;

type Result<T> = std::result::Result<T, ScanningError>;

pub fn scan<'source>(mut source: &'source AsciiStr) -> Result<Vec<ScannedToken<'source>>> {
    use crate::token::ScanResult;
    let mut tokens = Vec::with_capacity(source.len());
    let mut err = None;
    let mut line = 1;
    while let Some(res) = Token::split_from_slice(&mut source){
        match res{
            Ok(ScanResult::Newline) => line = line + 1,
            Ok(ScanResult::Token(token)) => {
                tokens.push(ScannedToken::new(token, line));
            },
            Err(e) => err = Some(e),
        }
    }
    match err{
        Some(error) => Err(error),
        None => Ok(tokens)
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    fn one() -> Token<'static>{
        NUMBER {
        lexeme: ascii("1"),
        value: 1.0,
    }}


    fn ascii(input: &'static str) -> &'static AsciiStr{
        // safe because we only use it here with valid ascii
        unsafe{
            AsciiStr::from_ascii_unchecked(input.as_bytes())
        }
    }

    #[test]
    fn scan_equation() {
        compare_scan(ascii("1+1"), vec![one(), PLUS, one()])
    }

    #[test]
    fn scan_quotes() {
        compare_scan(
            ascii("\"hiiii\""),
            vec![STRING(ascii("hiiii").into())],
        )
    }

    #[test]
    fn scan_parens() {
        compare_scan(
            ascii("({<>)}"),
            vec![LEFTPAREN, LEFTBRACE, LESS, GREATER, RIGHTPAREN, RIGHTBRACE],
        )
    }

    #[test]
    fn scan_paren_equation() {
        compare_scan(
            ascii("(1+1)"),
            vec![LEFTPAREN, one(), PLUS, one(), RIGHTPAREN],
        )
    }

    fn compare_scan(string: &AsciiStr, goal: Vec<Token>) {
        let scanned_tokens: Vec<_> = scan(string).unwrap().into_iter().map(|x| x.type_).collect();
        println!("{:?}", scanned_tokens);
        let scanned_tokens = scan(string).unwrap().into_iter().map(|x| x.type_);
        for (scanned_token, goal_token) in std::iter::zip(scanned_tokens, goal) {
            assert_eq!(scanned_token, goal_token)
        }
    }
}
