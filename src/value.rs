use core::fmt;
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String_(String),
    Bool(bool),
    Number(f64),
    Nil,
}

impl Mul for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let lhs = match self {
            Value::Number(f) => f,
            _ => panic!(),
        };
        let rhs = match rhs {
            Value::Number(f) => f,
            _ => panic!(),
        };

        let result = lhs * rhs;

        Self::Number(result)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(b) => write!(f, "{b}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::String_(s) => write!(f, "{s}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}
