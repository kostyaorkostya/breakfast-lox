use super::RuntimeError;
use crate::ast::Expr;

pub struct Interpreter;

impl Interpreter {
    pub fn eval(expr: &Expr) -> Result<bool, RuntimeError> {}
}
