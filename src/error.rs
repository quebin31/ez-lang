use std::error::Error;
use std::fmt;

use crate::lex::token::Token;

#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub got: Token,
    pub expected: Vec<Token>,
}

impl Error for SyntaxError {}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut expected = "".to_owned();
        for tok in &self.expected {
            expected.push_str(&format!("'{}', ", tok));
        }
        expected.pop();

        write!(f, "expected {}\ngot {}", expected, self.got)
    }
}
