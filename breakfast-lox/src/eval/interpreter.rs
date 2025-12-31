use super::{ArithmeticError, InvalidOperandTypeError, RuntimeError, Stringify, Truthy};
use super::{Env, Fuel, Val, VarName};
use crate::ast;
use std::io;

#[cfg(test)]
use std::cell::RefCell;
#[cfg(test)]
use std::rc::Rc;

fn eval_un_expr(
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    op: &ast::UnOp,
    e: &ast::Expr,
) -> Result<Val, RuntimeError> {
    match (op, eval_expr(fuel, env, out, &e)?) {
        (ast::UnOp::Neg, Val::Nil) => Err(InvalidOperandTypeError::UnOpNegOnNil)?,
        (ast::UnOp::Neg, Val::Bool(_)) => Err(InvalidOperandTypeError::UnOpNegOnBool)?,
        (ast::UnOp::Neg, Val::Str(_)) => Err(InvalidOperandTypeError::UnOpNegOnStr)?,
        (ast::UnOp::Neg, Val::Num(x)) => {
            fuel.burn()?;
            // TODO(kostya): check if `-x` is representable
            Ok(Val::Num(-x))
        }
        (ast::UnOp::Not, e) => {
            fuel.burn()?;
            Ok(Val::Bool(!e.truthy()))
        }
    }
}

fn eval_bin_expr(
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    op: &ast::BinOp,
    l: &ast::Expr,
    r: &ast::Expr,
) -> Result<Val, RuntimeError> {
    let l = eval_expr(fuel, env, out, &l)?;
    match op {
        ast::BinOp::Rel(ast::RelOp::Eq(op)) => {
            let r = eval_expr(fuel, env, out, &r)?;
            fuel.burn()?;
            Ok(Val::Bool(match op {
                ast::EqOp::Eq => l == r,
                ast::EqOp::Ne => l != r,
            }))
        }
        ast::BinOp::Rel(ast::RelOp::Cmp(op)) => {
            let r = eval_expr(fuel, env, out, &r)?;
            match (l, r) {
                (Val::Num(l), Val::Num(r)) => {
                    fuel.burn()?;
                    Ok(Val::Bool(match op {
                        ast::CmpOp::Lt => l < r,
                        ast::CmpOp::Le => l <= r,
                        ast::CmpOp::Gt => l > r,
                        ast::CmpOp::Ge => l >= r,
                    }))
                }
                _ => Err(InvalidOperandTypeError::CmpOp)?,
            }
        }
        ast::BinOp::Add(ast::AddOp::Add) => {
            let r = eval_expr(fuel, env, out, &r)?;
            match (l, r) {
                (Val::Num(l), Val::Num(r)) => {
                    fuel.burn()?;
                    Ok(Val::Num(l + r))
                }
                (Val::Str(l), Val::Str(r)) => {
                    fuel.burn()?;
                    Ok(Val::Str(l + &r))
                }
                (Val::Str(l), r @ Val::Num(_)) => {
                    fuel.burn()?;
                    // Challenge 2 from https://craftinginterpreters.com/evaluating-expressions.html#running-the-interpreter
                    // TODO(kostya): Apply some formatting rules to `r`?
                    Ok(Val::Str(format!("{l}{}", r.display())))
                }
                _ => Err(InvalidOperandTypeError::AddOpAdd)?,
            }
        }
        ast::BinOp::Add(ast::AddOp::Sub) => {
            let r = eval_expr(fuel, env, out, &r)?;
            match (l, r) {
                (Val::Num(l), Val::Num(r)) => {
                    fuel.burn()?;
                    Ok(Val::Num(l - r))
                }
                _ => Err(InvalidOperandTypeError::AddOpSub)?,
            }
        }
        ast::BinOp::Mul(op) => {
            let r = eval_expr(fuel, env, out, &r)?;
            match (l, r) {
                (Val::Num(l), Val::Num(r)) => Ok(Val::Num(match op {
                    ast::MulOp::Mul => {
                        fuel.burn()?;
                        Ok(l * r)
                    }
                    ast::MulOp::Div => {
                        if r == 0.0 {
                            Err(ArithmeticError::DivisionByZero)
                        } else {
                            fuel.burn()?;
                            Ok(l / r)
                        }
                    }
                }?)),
                _ => Err(InvalidOperandTypeError::MulOp)?,
            }
        }
        ast::BinOp::Log(op) => match (op, l.truthy()) {
            // https://craftinginterpreters.com/control-flow.html#logical-operators
            // > The other interesting piece here is deciding what actual value to return. Since
            // > Lox is dynamically typed, we allow operands of any type and use truthiness to
            // > determine what each operand represents. We apply similar reasoning to the result.
            // > Instead of promising to literally return true or false, a logic operator merely
            // > guarantees it will return a value with appropriate truthiness.
            (ast::LogOp::Or, true) | (ast::LogOp::And, false) => {
                fuel.burn()?;
                Ok(l)
            }
            _ => {
                let ret = eval_expr(fuel, env, out, &r)?;
                fuel.burn()?;
                Ok(ret)
            }
        },
    }
}

fn eval_assign(
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    assign: &ast::Assign,
) -> Result<Val, RuntimeError> {
    let ast::Assign { name, val } = assign;
    let val = eval_expr(fuel, env, out, val)?;
    fuel.burn()?;
    env.assign(name, val.clone())?;
    Ok(val)
}

fn eval_expr(
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    expr: &ast::Expr,
) -> Result<Val, RuntimeError> {
    match expr {
        ast::Expr::Lit(lit) => {
            fuel.burn()?;
            Ok(match lit {
                ast::Lit::Nil(_) => Val::Nil,
                ast::Lit::Bool(x) => Val::Bool(x.0),
                ast::Lit::Num(x) => Val::Num(x.0),
                ast::Lit::Str(x) => Val::Str(x.0.clone()),
            })
        }
        ast::Expr::Un(ast::UnExpr { op, e }) => eval_un_expr(fuel, env, out, op, e),
        ast::Expr::Bin(ast::BinExpr { op, l, r }) => eval_bin_expr(fuel, env, out, op, l, r),
        ast::Expr::Var(x) => {
            fuel.burn()?;
            Ok(env.get(&**x)?)
        }
        ast::Expr::Assign(x) => eval_assign(fuel, env, out, x),
    }
}

fn eval_var_decl(
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    var_decl: &ast::VarDecl,
) -> Result<(), RuntimeError> {
    let ast::VarDecl { name, init } = var_decl;
    match init {
        None => {
            fuel.burn()?;
            // Challenge 2 from https://craftinginterpreters.com/statements-and-state.html#challenges
            env.declare(VarName::new((**name).clone()))
        }
        Some(init) => {
            let init = eval_expr(fuel, env, out, init)?;
            env.define(VarName::new((**name).clone()), init)
        }
    }
    Ok(())
}

fn eval_block(
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    block: &ast::Block,
) -> Result<(), RuntimeError> {
    let ast::Block(stmts) = block;
    let mut env = env.extend();
    for stmt in stmts {
        eval_stmt(fuel, &mut env, out, stmt)?
    }
    Ok(())
}

fn eval_if(
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    if_: &ast::IfStmt,
) -> Result<(), RuntimeError> {
    let ast::IfStmt { cond, then, else_ } = if_;
    if eval_expr(fuel, env, out, cond)?.truthy() {
        eval_stmt(fuel, env, out, then)
    } else if let Some(else_) = else_ {
        eval_stmt(fuel, env, out, else_)
    } else {
        Ok(())
    }
}

fn eval_while(
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    while_: &ast::WhileStmt,
) -> Result<(), RuntimeError> {
    let ast::WhileStmt { cond, body } = while_;
    while eval_expr(fuel, env, out, cond)?.truthy() {
        eval_stmt(fuel, env, out, body)?
    }
    Ok(())
}

fn eval_stmt(
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    stmt: &ast::Stmt,
) -> Result<(), RuntimeError> {
    match stmt {
        ast::Stmt::Expr(ast::ExprStmt(x)) => {
            // https://craftinginterpreters.com/statements-and-state.html#executing-statements
            // > We evaluate the inner expression using our existing evaluate() method and
            // > discard the value.
            let _ = eval_expr(fuel, env, out, x)?;
            Ok(())
        }
        ast::Stmt::Print(ast::PrintStmt(x)) => {
            let x = eval_expr(fuel, env, out, x)?;
            writeln!(out, "{}", x.display())?;
            Ok(())
        }
        ast::Stmt::VarDecl(x) => eval_var_decl(fuel, env, out, x),
        ast::Stmt::Block(x) => eval_block(fuel, env, out, x),
        ast::Stmt::If(x) => eval_if(fuel, env, out, x),
        ast::Stmt::While(x) => eval_while(fuel, env, out, x),
    }
}

fn eval_prog(
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    prog: &ast::Prog,
) -> Result<(), RuntimeError> {
    let ast::Prog(stmts) = prog;
    for stmt in stmts {
        eval_stmt(fuel, env, out, stmt)?
    }
    Ok(())
}

pub struct Interpreter {
    fuel: Fuel,
    env: Env,
    out: Box<dyn io::Write>,
}

impl Interpreter {
    pub fn new(out: Box<dyn io::Write>) -> Self {
        Self {
            fuel: Fuel::Infinite,
            out,
            env: Env::new(),
        }
    }

    #[cfg(test)]
    pub(super) fn new_for_test(out: Option<Rc<RefCell<Vec<u8>>>>, fuel: u64) -> Self {
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
            fuel: Fuel::Finite(fuel),
            out: match out {
                Some(x) => Box::new(SharedWriter(x)),
                None => Box::new(io::sink()),
            },
            env: Env::new(),
        }
    }

    #[cfg(test)]
    pub(super) fn eval_expr(&mut self, expr: &ast::Expr) -> Result<Val, RuntimeError> {
        eval_expr(&mut self.fuel, &mut self.env, &mut *self.out, expr)
    }

    pub fn eval_prog(&mut self, prog: &ast::Prog) -> Result<(), RuntimeError> {
        eval_prog(&mut self.fuel, &mut self.env, &mut *self.out, prog)
    }
}
