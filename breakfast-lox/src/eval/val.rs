use super::{Stringify, Truthy};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Val {
    Nil,
    Bool(bool),
    Num(f64),
    Str(String),
}

impl Stringify for Val {
    fn stringify(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Bool(x) => write!(f, "{x}"),
            Self::Num(x) => write!(f, "{x}"),
            Self::Str(x) => write!(f, "{x}"),
        }
    }
}

// https://craftinginterpreters.com/evaluating-expressions.html#truthiness-and-falsiness
impl Truthy for Val {
    fn truthy(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Bool(x) => *x,
            Self::Num(_) | Self::Str(_) => true,
        }
    }
}
