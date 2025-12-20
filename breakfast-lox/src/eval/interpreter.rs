use super::Value;
use super::{InvalidOperandTypeError, RuntimeError};
use crate::ast::{AddOp, BinExpr, BinOp, CmpOp, EqOp, Expr, Lit, MulOp, RelOp, UnExpr, UnOp};

pub struct Interpreter;

impl Interpreter {
    fn eval_un_expr(&self, op: &UnOp, e: &Expr) -> Result<Value, RuntimeError> {
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
                Ok(Value::Bool(true))
            }
            (UnOp::Not, Value::Bool(x)) => Ok(Value::Bool(!x)),
        }
    }

    fn eval_bin_expr(&self, op: &BinOp, l: &Expr, r: &Expr) -> Result<Value, RuntimeError> {
        let (l, r) = (self.eval(&l)?, self.eval(&r)?);
        match op {
            BinOp::Rel(RelOp::Eq(op)) => Ok(Value::Bool(match op {
                EqOp::Eq => l == r,
                EqOp::Ne => l != r,
            })),
            BinOp::Rel(RelOp::Cmp(cmp)) => match (l, r) {
                (Value::Num(l), Value::Num(r)) => Ok(Value::Bool(match cmp {
                    CmpOp::Lt => l < r,
                    CmpOp::Le => l <= r,
                    CmpOp::Gt => l > r,
                    CmpOp::Ge => l >= r,
                })),
                _ => Err(InvalidOperandTypeError::CmpOp)?,
            },
            BinOp::Add(AddOp::Add) => match (l, r) {
                (Value::Num(l), Value::Num(r)) => Ok(Value::Num(l + r)),
                (Value::Str(l), Value::Str(r)) => Ok(Value::Str(l + &r)),
                _ => Err(InvalidOperandTypeError::AddOpAdd)?,
            },
            BinOp::Add(AddOp::Sub) => match (l, r) {
                (Value::Num(l), Value::Num(r)) => Ok(Value::Num(l - r)),
                _ => Err(InvalidOperandTypeError::AddOpSub)?,
            },
            BinOp::Mul(mul) => match (l, r) {
                (Value::Num(l), Value::Num(r)) => Ok(Value::Num(match mul {
                    MulOp::Mul => l * r,
                    MulOp::Div => l / r,
                })),
                _ => Err(InvalidOperandTypeError::MulOp)?,
            },
        }
    }

    pub fn eval(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Lit(lit) => Ok(match lit {
                Lit::Nil(_) => Value::Nil,
                Lit::Bool(x) => Value::Bool(x.0),
                Lit::Num(x) => Value::Num(x.0),
                Lit::Str(x) => Value::Str(x.0.clone()),
            }),
            Expr::Un(UnExpr { op, e }) => self.eval_un_expr(op, e),
            Expr::Bin(BinExpr { op, l, r }) => self.eval_bin_expr(op, l, r),
        }
    }
}
