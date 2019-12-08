use std::fmt::{self, Display, Formatter};

use crate::ast::Visitor;
use crate::ast::{arithm, cons, ident, index, temp, unary, util};
use crate::sym::Type;

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(ident::Ident),
    Cons(cons::Cons),
    Temp(temp::Temp),
    Arithm(arithm::Arithm),
    Unary(unary::Unary),
    Index(index::Index),
}

impl Expr {
    pub fn generate(&self, visitor: &mut Visitor) -> Self {
        match self {
            Self::Arithm(arithm) => Self::Arithm(arithm::Arithm::new(
                &arithm.op,
                &arithm.expr1.reduce(visitor),
                &arithm.expr2.reduce(visitor),
            )),

            Self::Unary(unary) => {
                Self::Unary(unary::Unary::new(&unary.op, &unary.expr.reduce(visitor)))
            }

            Self::Index(index) => Self::Index(index::Index {
                index: Box::new(index.index.reduce(visitor)),
                ..index.clone()
            }),

            _ => self.clone(),
        }
    }

    pub fn reduce(&self, visitor: &mut Visitor) -> Self {
        match self {
            op @ Self::Arithm(_) | op @ Self::Unary(_) | op @ Self::Index(_) => {
                let expr = self.generate(visitor);
                let temp = temp::Temp {
                    id: util::new_temp_id(),
                    tp: expr.get_tp().clone(),
                };

                let op_code = match op {
                    Self::Arithm(arithm) => arithm.get_opcode(),
                    Self::Unary(_) => "inv".to_owned(),
                    Self::Index(_) => "idx".to_owned(),
                    _ => unreachable!(),
                };

                visitor.emit_inst(&format!("{} {} {}", op_code, temp, expr));
                Self::Temp(temp)
            }

            _ => self.clone(),
        }
    }

    pub fn jumping(&self, visitor: &mut Visitor, true_label: usize, false_label: usize) {
        visitor.emit_jump(&self.to_string(), true_label, false_label)
    }

    pub fn get_tp(&self) -> Type {
        match self {
            Self::Ident(ident) => ident.tp.clone(),
            Self::Cons(cons) => cons.tp.clone(),
            Self::Temp(temp) => temp.tp.clone(),
            Self::Arithm(arithm) => arithm.tp.clone(),
            Self::Unary(unary) => unary.tp.clone(),
            Self::Index(index) => index.array.tp.clone(),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let out = match self {
            Self::Ident(ident) => ident.to_string(),
            Self::Cons(cons) => cons.to_string(),
            Self::Temp(temp) => temp.to_string(),
            Self::Arithm(arithm) => arithm.to_string(),
            Self::Unary(unary) => unary.to_string(),
            Self::Index(index) => index.to_string(),
        };

        write!(f, "{}", out)
    }
}
