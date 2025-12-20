use super::Value;
use super::{InvalidOperandTypeError, RuntimeError};
use crate::ast::{AddOp, BinExpr, BinOp, CmpOp, EqOp, Expr, Lit, MulOp, RelOp, UnExpr, UnOp};

pub struct Interpreter;

impl Interpreter {
    fn eval_lit(&self, lit: &Lit) -> Result<Value, RuntimeError> {
        Ok(match lit {
            Lit::Nil(_) => Value::Nil,
            Lit::Bool(x) => Value::Bool(x.0),
            Lit::Num(x) => Value::Num(x.0),
            Lit::Str(x) => Value::Str(x.0.clone()),
        })
    }

    fn eval_un_expr(&self, expr: &UnExpr) -> Result<Value, RuntimeError> {
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

    fn eval_bin_expr(&self, expr: &BinExpr) -> Result<Value, RuntimeError> {
        match expr {
            BinExpr { op, l, r } => match (op, self.eval(&l)?, self.eval(&r)?) {
                (BinOp::Rel(RelOp::Eq(EqOp::Eq)), Value::Nil, Value::Nil) => Ok(Value::Bool(true)),
                (BinOp::Rel(RelOp::Eq(EqOp::Eq)), Value::Nil, _)
                | (BinOp::Rel(RelOp::Eq(EqOp::Eq)), _, Value::Nil) => Ok(Value::Bool(false)),
                (BinOp::Rel(RelOp::Eq(EqOp::Eq)), Value::Bool(l), Value::Bool(r)) => {
                    Ok(Value::Bool(l == r))
                }
                (BinOp::Rel(RelOp::Eq(EqOp::Eq)), Value::Num(l), Value::Num(r)) => {
                    Ok(Value::Bool(l == r))
                }
                (BinOp::Rel(RelOp::Eq(EqOp::Eq)), Value::Str(l), Value::Str(r)) => {
                    Ok(Value::Bool(l == r))
                }
                // Ne
                (BinOp::Rel(RelOp::Eq(EqOp::Ne)), Value::Nil, Value::Nil) => Ok(Value::Bool(false)),
                (BinOp::Rel(RelOp::Eq(EqOp::Ne)), Value::Nil, _)
                | (BinOp::Rel(RelOp::Eq(EqOp::Ne)), _, Value::Nil) => Ok(Value::Bool(true)),
                (BinOp::Rel(RelOp::Eq(EqOp::Ne)), Value::Bool(l), Value::Bool(r)) => {
                    Ok(Value::Bool(l != r))
                }
                (BinOp::Rel(RelOp::Eq(EqOp::Ne)), Value::Num(l), Value::Num(r)) => {
                    Ok(Value::Bool(l != r))
                }
                (BinOp::Rel(RelOp::Eq(EqOp::Ne)), Value::Str(l), Value::Str(r)) => {
                    Ok(Value::Bool(l != r))
                }
                (BinOp::Rel(RelOp::Eq(_)), _, _) => Err(InvalidOperandTypeError::EqOp)?,
                // Cmp
                (BinOp::Rel(RelOp::Cmp(CmpOp::Lt)), Value::Num(l), Value::Num(r)) => {
                    Ok(Value::Bool(l < r))
                }
                (BinOp::Rel(RelOp::Cmp(CmpOp::Le)), Value::Num(l), Value::Num(r)) => {
                    Ok(Value::Bool(l <= r))
                }
                (BinOp::Rel(RelOp::Cmp(CmpOp::Gt)), Value::Num(l), Value::Num(r)) => {
                    Ok(Value::Bool(l > r))
                }
                (BinOp::Rel(RelOp::Cmp(CmpOp::Ge)), Value::Num(l), Value::Num(r)) => {
                    Ok(Value::Bool(l >= r))
                }
                (BinOp::Rel(RelOp::Cmp(_)), _, _) => Err(InvalidOperandTypeError::CmpOp)?,
                // Add
                (BinOp::Add(AddOp::Add), Value::Str(l), Value::Str(r)) => Ok(Value::Str(l + &r)),
                (BinOp::Add(AddOp::Add), Value::Num(l), Value::Num(r)) => Ok(Value::Num(l + r)),
                (BinOp::Add(AddOp::Add), _, _) => Err(InvalidOperandTypeError::AddOpAdd)?,
                // Sub
                (BinOp::Add(AddOp::Sub), Value::Num(l), Value::Num(r)) => Ok(Value::Num(l - r)),
                (BinOp::Add(AddOp::Sub), _, _) => Err(InvalidOperandTypeError::AddOpSub)?,
                // Sub, Mul, Div
                (BinOp::Mul(MulOp::Mul), Value::Num(l), Value::Num(r)) => Ok(Value::Num(l * r)),
                (BinOp::Mul(MulOp::Div), Value::Num(l), Value::Num(r)) => Ok(Value::Num(l / r)),
                (BinOp::Mul(_), _, _) => Err(InvalidOperandTypeError::MulOp)?,
            },
        }
    }

    pub fn eval(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Lit(lit) => self.eval_lit(&lit),
            Expr::Un(un) => self.eval_un_expr(&un),
            Expr::Bin(bin) => self.eval_bin_expr(&bin),
        }
    }
}
