use std::fmt::{self, Display, Formatter};

use crate::ast::Expr;
use crate::lex::Token;
use crate::sym::Type;

#[derive(Debug, Clone)]
pub struct Arithm {
    pub op: Token,
    pub tp: Type,
    pub expr1: Box<Expr>,
    pub expr2: Box<Expr>,
}

impl Arithm {
    pub fn new(op: &Token, expr1: &Expr, expr2: &Expr) -> Self {
        let tp = expr1
            .get_tp()
            .upcast(&expr2.get_tp())
            .expect("Failed to coherce");

        Self {
            op: op.clone(),
            tp,
            expr1: Box::new(expr1.clone()),
            expr2: Box::new(expr2.clone()),
        }
    }

    pub fn get_opcode(&self) -> String {
        match self.op {
            Token::Plus => "add".to_owned(),
            Token::Minus => "sub".to_owned(),
            Token::Divide => "div".to_owned(),
            Token::Asterisk => "mul".to_owned(),
            _ => panic!("Bad operator"),
        }
    }
}

impl Display for Arithm {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {}", self.expr1, self.expr2)
    }
}
