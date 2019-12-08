use std::fmt::{self, Display, Formatter};

use crate::sym::Type;

#[derive(Debug, Clone)]
pub struct Ident {
    pub id: String,
    pub tp: Type,
    pub offset: usize,
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
