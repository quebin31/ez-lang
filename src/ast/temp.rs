use std::fmt::{self, Display, Formatter};

use crate::sym::Type;

#[derive(Debug, Clone)]
pub struct Temp {
    pub id: usize,
    pub tp: Type,
}

impl Display for Temp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "__t{}", self.id)
    }
}
