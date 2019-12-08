use std::fmt::{self, Display, Formatter};

use crate::ast::Expr;
use crate::lex::Token;
use crate::sym::Type;

#[derive(Debug, Clone)]
pub struct Unary {
    pub op: Token,
    pub tp: Type,
    pub expr: Box<Expr>,
}

impl Unary {
    pub fn new(op: &Token, expr: &Expr) -> Self {
        let tp = Type::Int64
            .upcast(&expr.get_tp())
            .expect("Failed to coherce");

        Self {
            op: op.clone(),
            tp,
            expr: Box::new(expr.clone()),
        }
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}
