use super::Value;
use super::{ArithmeticError, InvalidOperandTypeError, RuntimeError, Stringify, Truthy};
use crate::ast::{
    AddOp, BinExpr, BinOp, CmpOp, EqOp, Expr, ExprStmt, Lit, MulOp, PrintStmt, Prog, RelOp, Stmt,
    UnExpr, UnOp,
};
use std::io;

#[cfg(test)]
use std::cell::RefCell;
#[cfg(test)]
use std::rc::Rc;

pub struct Interpreter {
    output: Box<dyn io::Write>,
}

impl Interpreter {
    pub fn new(output: Box<dyn io::Write>) -> Self {
        Self { output }
    }

    #[cfg(test)]
    pub(super) fn new_for_test(output: Option<Rc<RefCell<Vec<u8>>>>) -> Self {
        struct SharedWriter(Rc<RefCell<Vec<u8>>>);

        impl io::Write for SharedWriter {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.0.borrow_mut().write(buf)
            }

            fn flush(&mut self) -> io::Result<()> {
                self.0.borrow_mut().flush()
            }
        }
        Self {
            output: match output {
                Some(x) => Box::new(SharedWriter(x)),
                None => Box::new(io::sink()),
            },
        }
    }

    pub(super) fn eval_un_expr(&self, op: &UnOp, e: &Expr) -> Result<Value, RuntimeError> {
        match (op, self.eval_expr(&e)?) {
            (UnOp::Neg, Value::Nil) => Err(InvalidOperandTypeError::UnOpNegOnNil)?,
            (UnOp::Neg, Value::Bool(_)) => Err(InvalidOperandTypeError::UnOpNegOnBool)?,
            (UnOp::Neg, Value::Str(_)) => Err(InvalidOperandTypeError::UnOpNegOnStr)?,
            (UnOp::Neg, Value::Num(x)) => {
                // TODO(kostya): check if `-x` is representable
                Ok(Value::Num(-x))
            }
            (UnOp::Not, e) => Ok(Value::Bool(!e.truthy())),
        }
    }

    pub(super) fn eval_bin_expr(
        &self,
        op: &BinOp,
        l: &Expr,
        r: &Expr,
    ) -> Result<Value, RuntimeError> {
        let (l, r) = (self.eval_expr(&l)?, self.eval_expr(&r)?);
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
                (Value::Str(l), r @ Value::Num(_)) => {
                    // Challenge 2 from https://craftinginterpreters.com/evaluating-expressions.html#running-the-interpreter
                    // TODO(kostya): Apply some formatting rules to `r`?
                    Ok(Value::Str(format!("{l}{}", r.display())))
                }
                _ => Err(InvalidOperandTypeError::AddOpAdd)?,
            },
            BinOp::Add(AddOp::Sub) => match (l, r) {
                (Value::Num(l), Value::Num(r)) => Ok(Value::Num(l - r)),
                _ => Err(InvalidOperandTypeError::AddOpSub)?,
            },
            BinOp::Mul(mul) => match (l, r) {
                (Value::Num(l), Value::Num(r)) => Ok(Value::Num(match mul {
                    MulOp::Mul => Ok(l * r),
                    MulOp::Div => {
                        if r == 0.0 {
                            Err(ArithmeticError::DivisionByZero)
                        } else {
                            Ok(l / r)
                        }
                    }
                }?)),
                _ => Err(InvalidOperandTypeError::MulOp)?,
            },
        }
    }

    pub(super) fn eval_expr(&self, expr: &Expr) -> Result<Value, RuntimeError> {
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

    fn eval_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expr(ExprStmt(x)) => {
                let _ = self.eval_expr(x)?;
                Ok(())
            }
            Stmt::Print(PrintStmt(x)) => {
                let x = self.eval_expr(x)?;
                writeln!(self.output, "{}", x.display())?;
                Ok(())
            }
        }
    }

    pub fn eval_prog(&mut self, prog: &Prog) -> Result<(), RuntimeError> {
        let Prog(stmts) = prog;
        for stmt in stmts {
            self.eval_stmt(stmt)?
        }
        Ok(())
    }
}
