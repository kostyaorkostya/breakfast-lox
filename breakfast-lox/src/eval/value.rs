use super::Stringify;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Num(f64),
    Str(String),
}

impl Stringify for Value {
    fn stringify(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Bool(x) => write!(f, "{x}"),
            Self::Num(x) => write!(f, "{x}"),
            Self::Str(x) => write!(f, "{x}"),
        }
    }
}
