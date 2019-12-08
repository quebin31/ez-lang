use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int32,
    Int64,
    Flt32,
    Flt64,
    Char,
    Bool,
    String(usize),
    Array { of: Box<Type>, size: usize },
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        match self {
            Self::Int32 | Self::Int64 | Self::Flt32 | Self::Flt64 | Self::Char => true,
            _ => false,
        }
    }

    pub fn upcast(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (tp1, tp2) if !tp1.is_numeric() || !tp2.is_numeric() => None,
            (Self::Flt64, _) | (_, Self::Flt64) => Some(Self::Flt64),
            (Self::Flt32, _) | (_, Self::Flt32) => Some(Self::Flt32),
            (Self::Int64, _) | (_, Self::Int64) => Some(Self::Int64),
            (Self::Int32, _) | (_, Self::Int32) => Some(Self::Int32),
            _ => Some(Self::Char),
        }
    }

    pub fn get_width(&self) -> usize {
        match self {
            Self::Int32 => 4,
            Self::Int64 => 8,
            Self::Flt32 => 4,
            Self::Flt64 => 8,
            Self::Bool => 1,
            Self::Char => 1,
            Self::String(size) => *size,
            Self::Array { of, size } => of.get_width() * size,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let out = match self {
            Self::Int32 => "i32".to_owned(),
            Self::Int64 => "i64".to_owned(),
            Self::Flt32 => "f32".to_owned(),
            Self::Flt64 => "f64".to_owned(),
            Self::Bool => "bool".to_owned(),
            Self::Char => "char".to_owned(),
            Self::String(_) => "string".to_owned(),
            Self::Array { of, size } => format!("[{}]{}", size, of.to_string()),
        };

        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upcasting() {
        assert_eq!(Some(Type::Int32), Type::Int32.upcast(&Type::Char));
        assert_eq!(Some(Type::Flt64), Type::Flt32.upcast(&Type::Flt64));
        assert_eq!(Some(Type::Char), Type::Char.upcast(&Type::Char));
        assert_eq!(None, Type::String(2).upcast(&Type::Bool));
    }

    #[test]
    fn display() {
        assert_eq!(
            "[3][2]i32".to_owned(),
            Type::Array {
                of: Box::new(Type::Array {
                    of: Box::new(Type::Int32),
                    size: 2
                }),
                size: 3
            }
            .to_string()
        );
    }
}
