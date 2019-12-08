use std::fmt::{self, Display, Formatter};

use crate::ast::{Expr, Ident};

#[derive(Debug, Clone)]
pub struct Index {
    pub array: Ident,
    pub index: Box<Expr>,
}

impl Display for Index {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {}", self.index, self.array)
    }
}
