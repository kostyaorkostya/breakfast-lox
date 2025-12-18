use super::Value;
use super::{InvalidOperandTypeError, RuntimeError};
use crate::ast::{Expr, Lit, UnExpr, UnOp};

pub struct Interpreter;

impl Interpreter {
    fn eva_lit(self, lit: &Lit) -> Result<Value, RuntimeError> {
        Ok(match lit {
            Lit::Nil(_) => Value::Nil,
            Lit::Bool(x) => Value::Bool(x.0),
            Lit::Num(x) => Value::Num(x.0),
            Lit::Str(x) => Value::Str(x.0.clone()),
        })
    }

    fn eval_un_expr(self, expr: &UnExpr) -> Result<Value, RuntimeError> {
        match expr {
            UnExpr { op, e } => {
                match (op, self.eval(&e)?) {
                    (UnOp::Neg, Value::Nil) => Err(InvalidOperandTypeError::UnOpNegOnNil)?,
                    (UnOp::Neg, Value::Bool(_)) => Err(InvalidOperandTypeError::UnOpNegOnBool)?,
                    (UnOp::Neg, Value::Str(_)) => Err(InvalidOperandTypeError::UnOpNegOnStr)?,
                    (UnOp::Not, Value::Num(_)) => Err(InvalidOperandTypeError::UnOpNotOnNum)?,
                    (UnOp::Not, Value::Str(_)) => Err(InvalidOperandTypeError::UnOpNotOnStr)?,
                    (UnOp::Neg, Value::Num(x)) => {
                        // TODO(kostya): check if `-x` is representable
                        Ok(Value::Num(-x))
                    }
                    (UnOp::Not, Value::Nil) => {
                        // https://craftinginterpreters.com/evaluating-expressions.html#truthiness-and-falsiness
                        Ok(Value::Bool(false))
                    }
                    (UnOp::Not, Value::Bool(x)) => Ok(Value::Bool(!x)),
                }
            }
        }
    }

    pub fn eval(self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr
        }
    }
}
