use super::{ArithmeticError, InvalidOperandTypeError, RuntimeError, Stringify, Truthy};
use super::{Env, Value, VarName};
use crate::ast;
use std::io;

#[cfg(test)]
use std::cell::RefCell;
#[cfg(test)]
use std::rc::Rc;

pub struct Interpreter {
    output: Box<dyn io::Write>,
    env: Env,
}

impl Interpreter {
    pub fn new(output: Box<dyn io::Write>) -> Self {
        Self {
            output,
            env: Env::new(),
        }
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
            env: Env::new(),
        }
    }

    pub(super) fn eval_un_expr(
        &mut self,
        op: &ast::UnOp,
        e: &ast::Expr,
    ) -> Result<Value, RuntimeError> {
        match (op, self.eval_expr(&e)?) {
            (ast::UnOp::Neg, Value::Nil) => Err(InvalidOperandTypeError::UnOpNegOnNil)?,
            (ast::UnOp::Neg, Value::Bool(_)) => Err(InvalidOperandTypeError::UnOpNegOnBool)?,
            (ast::UnOp::Neg, Value::Str(_)) => Err(InvalidOperandTypeError::UnOpNegOnStr)?,
            (ast::UnOp::Neg, Value::Num(x)) => {
                // TODO(kostya): check if `-x` is representable
                Ok(Value::Num(-x))
            }
            (ast::UnOp::Not, e) => Ok(Value::Bool(!e.truthy())),
        }
    }

    pub(super) fn eval_bin_expr(
        &mut self,
        op: &ast::BinOp,
        l: &ast::Expr,
        r: &ast::Expr,
    ) -> Result<Value, RuntimeError> {
        let (l, r) = (self.eval_expr(&l)?, self.eval_expr(&r)?);
        match op {
            ast::BinOp::Rel(ast::RelOp::Eq(op)) => Ok(Value::Bool(match op {
                ast::EqOp::Eq => l == r,
                ast::EqOp::Ne => l != r,
            })),
            ast::BinOp::Rel(ast::RelOp::Cmp(cmp)) => match (l, r) {
                (Value::Num(l), Value::Num(r)) => Ok(Value::Bool(match cmp {
                    ast::CmpOp::Lt => l < r,
                    ast::CmpOp::Le => l <= r,
                    ast::CmpOp::Gt => l > r,
                    ast::CmpOp::Ge => l >= r,
                })),
                _ => Err(InvalidOperandTypeError::CmpOp)?,
            },
            ast::BinOp::Add(ast::AddOp::Add) => match (l, r) {
                (Value::Num(l), Value::Num(r)) => Ok(Value::Num(l + r)),
                (Value::Str(l), Value::Str(r)) => Ok(Value::Str(l + &r)),
                (Value::Str(l), r @ Value::Num(_)) => {
                    // Challenge 2 from https://craftinginterpreters.com/evaluating-expressions.html#running-the-interpreter
                    // TODO(kostya): Apply some formatting rules to `r`?
                    Ok(Value::Str(format!("{l}{}", r.display())))
                }
                _ => Err(InvalidOperandTypeError::AddOpAdd)?,
            },
            ast::BinOp::Add(ast::AddOp::Sub) => match (l, r) {
                (Value::Num(l), Value::Num(r)) => Ok(Value::Num(l - r)),
                _ => Err(InvalidOperandTypeError::AddOpSub)?,
            },
            ast::BinOp::Mul(mul) => match (l, r) {
                (Value::Num(l), Value::Num(r)) => Ok(Value::Num(match mul {
                    ast::MulOp::Mul => Ok(l * r),
                    ast::MulOp::Div => {
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

    fn eval_assign(&mut self, assign: &ast::Assign) -> Result<Value, RuntimeError> {
        let ast::Assign { name, val } = assign;
        let val = self.eval_expr(val)?;
        self.env.assign(name, val.clone())?;
        Ok(val)
    }

    pub(super) fn eval_expr(&mut self, expr: &ast::Expr) -> Result<Value, RuntimeError> {
        match expr {
            ast::Expr::Lit(lit) => Ok(match lit {
                ast::Lit::Nil(_) => Value::Nil,
                ast::Lit::Bool(x) => Value::Bool(x.0),
                ast::Lit::Num(x) => Value::Num(x.0),
                ast::Lit::Str(x) => Value::Str(x.0.clone()),
            }),
            ast::Expr::Un(ast::UnExpr { op, e }) => self.eval_un_expr(op, e),
            ast::Expr::Bin(ast::BinExpr { op, l, r }) => self.eval_bin_expr(op, l, r),
            ast::Expr::Var(x) => Ok(self.env.get(&**x)?),
            ast::Expr::Assign(x) => self.eval_assign(x),
        }
    }

    fn eval_var_decl(&mut self, var_decl: &ast::VarDecl) -> Result<(), RuntimeError> {
        let ast::VarDecl { name, init } = var_decl;
        match init {
            None => self.env.define(VarName::new((**name).clone()), Value::Nil),
            Some(init) => {
                let init = self.eval_expr(init)?;
                self.env.define(VarName::new((**name).clone()), init)
            }
        }
        Ok(())
    }

    fn eval_stmt(&mut self, stmt: &ast::Stmt) -> Result<(), RuntimeError> {
        match stmt {
            ast::Stmt::Expr(ast::ExprStmt(x)) => {
                // https://craftinginterpreters.com/statements-and-state.html#executing-statements
                // > We evaluate the inner expression using our existing evaluate() method and
                // > discard the value.
                let _ = self.eval_expr(x)?;
                Ok(())
            }
            ast::Stmt::Print(ast::PrintStmt(x)) => {
                let x = self.eval_expr(x)?;
                writeln!(self.output, "{}", x.display())?;
                Ok(())
            }
            ast::Stmt::VarDecl(x) => self.eval_var_decl(x),
        }
    }

    pub fn eval_prog(&mut self, prog: &ast::Prog) -> Result<(), RuntimeError> {
        let ast::Prog(stmts) = prog;
        for stmt in stmts {
            self.eval_stmt(stmt)?
        }
        Ok(())
    }
}
