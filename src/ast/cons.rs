use std::fmt::{self, Display, Formatter};

use crate::lex::Token;
use crate::sym::Type;

#[derive(Debug, Clone)]
pub struct Cons {
    pub tok: Token,
    pub tp: Type,
}

impl Display for Cons {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.tok)
    }
}
