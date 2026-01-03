use super::{
    ControlFlow, InternalCompilerError, NativeFn, RuntimeError, Stringify, Truthy, TypeError,
    UserFn, VarName,
};
use super::{Env, Fn, Fuel, Val, native_fns};
use crate::ast;
use std::cell::RefCell;
use std::io;
use std::iter::zip;
use std::rc::Rc;

fn eval_un_expr(
    glob_env: &mut Env,
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    op: &ast::UnOp,
    e: &ast::Expr,
) -> Result<Val, RuntimeError> {
    let val = eval_expr(glob_env, fuel, env, out, &e)?;
    fuel.burn()?;
    Ok(val.eval_un_op(op)?)
}

fn eval_bin_expr(
    glob_env: &mut Env,
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    op: &ast::BinOp,
    l: &ast::Expr,
    r: &ast::Expr,
) -> Result<Val, RuntimeError> {
    let l = eval_expr(glob_env, fuel, env, out, &l)?;
    match op {
        ast::BinOp::Rel(ast::RelOp::Eq(op)) => {
            let r = eval_expr(glob_env, fuel, env, out, &r)?;
            fuel.burn()?;
            Ok(Val::Bool(l.eval_eq_op(op, &r)))
        }
        ast::BinOp::Rel(ast::RelOp::Cmp(op)) => {
            let r = eval_expr(glob_env, fuel, env, out, &r)?;
            fuel.burn()?;
            Ok(Val::Bool(l.eval_cmp_op(op, &r)?))
        }
        ast::BinOp::Add(op) => {
            let r = eval_expr(glob_env, fuel, env, out, &r)?;
            fuel.burn()?;
            Ok(l.eval_add_op(op, &r)?)
        }
        ast::BinOp::Mul(op) => {
            let r = eval_expr(glob_env, fuel, env, out, &r)?;
            fuel.burn()?;
            Ok(Val::Num(l.eval_mul_op(op, &r)?))
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
                let ret = eval_expr(glob_env, fuel, env, out, &r)?;
                fuel.burn()?;
                Ok(ret)
            }
        },
    }
}

fn eval_assign(
    glob_env: &mut Env,
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    assign: &ast::Assign,
) -> Result<Val, RuntimeError> {
    let ast::Assign { name, val } = assign;
    let val = eval_expr(glob_env, fuel, env, out, val)?;
    fuel.burn()?;
    env.assign(name, val.clone())?;
    Ok(val)
}

fn eval_call(
    glob_env: &mut Env,
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    call: &ast::Call,
) -> Result<Val, RuntimeError> {
    let ast::Call { callee, args } = call;
    match eval_expr(glob_env, fuel, env, out, callee)? {
        x @ (Val::Nil | Val::Bool(_) | Val::Num(_) | Val::Str(_)) => Err(TypeError {
            msg: format!("is not a function `{x:?}`"),
        })?,
        Val::Fn(fn_) => {
            if fn_.arity() != args.len() {
                Err(TypeError {
                    msg: format!(
                        "`{}` takes {} arguments, {} provided",
                        fn_.name(),
                        fn_.arity(),
                        args.len()
                    ),
                })?
            }

            let args = args
                .iter()
                .map(|arg| eval_expr(glob_env, fuel, env, out, arg))
                .collect::<Result<Vec<_>, _>>()?;

            match &*fn_ {
                Fn::Native(NativeFn { fn_, .. }) => {
                    fuel.burn()?;
                    fn_(glob_env, &args)
                }
                Fn::User(UserFn {
                    name: _,
                    params,
                    body,
                }) => {
                    let mut env = env.extend();
                    for (name, arg) in zip(params, args) {
                        env.define(name.clone(), arg);
                    }
                    match eval_block(glob_env, fuel, &mut env, out, body)? {
                        ControlFlow::Cont => Ok(Val::Nil),
                        ControlFlow::Break => Err(InternalCompilerError {
                            msg: format!("break outside a loop, should have been a syntax error"),
                        })?,
                    }
                }
            }
        }
    }
}

fn eval_expr(
    glob_env: &mut Env,
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    expr: &ast::Expr,
) -> Result<Val, RuntimeError> {
    match expr {
        ast::Expr::Lit(lit) => {
            fuel.burn()?;
            Ok(lit.clone().into())
        }
        ast::Expr::Un(ast::UnExpr { op, e }) => eval_un_expr(glob_env, fuel, env, out, op, e),
        ast::Expr::Bin(ast::BinExpr { op, l, r }) => {
            eval_bin_expr(glob_env, fuel, env, out, op, l, r)
        }
        ast::Expr::Var(x) => {
            fuel.burn()?;
            Ok(env.get(&**x)?)
        }
        ast::Expr::Assign(x) => eval_assign(glob_env, fuel, env, out, x),
        ast::Expr::Call(x) => eval_call(glob_env, fuel, env, out, x),
    }
}

fn eval_var_decl(
    glob_env: &mut Env,
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    var_decl: &ast::VarDecl,
) -> Result<ControlFlow, RuntimeError> {
    let ast::VarDecl { name, init } = var_decl;
    match init {
        None => {
            fuel.burn()?;
            // Challenge 2 from https://craftinginterpreters.com/statements-and-state.html#challenges
            env.declare(name.clone().into())
        }
        Some(init) => {
            let init = eval_expr(glob_env, fuel, env, out, init)?;
            env.define(name.clone().into(), init)
        }
    }
    Ok(ControlFlow::Cont)
}

fn eval_block(
    glob_env: &mut Env,
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    block: &ast::Block,
) -> Result<ControlFlow, RuntimeError> {
    let ast::Block(stmts) = block;
    let mut env = env.extend();
    for stmt in stmts {
        match eval_stmt(glob_env, fuel, &mut env, out, stmt)? {
            ControlFlow::Cont => (),
            ControlFlow::Break => return Ok(ControlFlow::Break),
        }
    }
    Ok(ControlFlow::Cont)
}

fn eval_if(
    glob_env: &mut Env,
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    if_: &ast::IfStmt,
) -> Result<ControlFlow, RuntimeError> {
    let ast::IfStmt { cond, then, else_ } = if_;
    if eval_expr(glob_env, fuel, env, out, cond)?.truthy() {
        eval_stmt(glob_env, fuel, env, out, then)
    } else if let Some(else_) = else_ {
        eval_stmt(glob_env, fuel, env, out, else_)
    } else {
        Ok(ControlFlow::Cont)
    }
}

fn eval_while(
    glob_env: &mut Env,
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    while_: &ast::WhileStmt,
) -> Result<ControlFlow, RuntimeError> {
    let ast::WhileStmt { cond, body } = while_;
    while eval_expr(glob_env, fuel, env, out, cond)?.truthy() {
        match eval_stmt(glob_env, fuel, env, out, body)? {
            ControlFlow::Cont => (),
            ControlFlow::Break => break,
        }
    }
    Ok(ControlFlow::Cont)
}

fn eval_stmt(
    glob_env: &mut Env,
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    stmt: &ast::Stmt,
) -> Result<ControlFlow, RuntimeError> {
    match stmt {
        ast::Stmt::Expr(ast::ExprStmt(x)) => {
            // https://craftinginterpreters.com/statements-and-state.html#executing-statements
            // > We evaluate the inner expression using our existing evaluate() method and
            // > discard the value.
            let _ = eval_expr(glob_env, fuel, env, out, x)?;
            Ok(ControlFlow::Cont)
        }
        ast::Stmt::Print(ast::PrintStmt(x)) => {
            let x = eval_expr(glob_env, fuel, env, out, x)?;
            fuel.burn()?;
            writeln!(out, "{}", x.display())?;
            Ok(ControlFlow::Cont)
        }
        ast::Stmt::VarDecl(x) => eval_var_decl(glob_env, fuel, env, out, x),
        ast::Stmt::Block(x) => eval_block(glob_env, fuel, env, out, x),
        ast::Stmt::If(x) => eval_if(glob_env, fuel, env, out, x),
        ast::Stmt::While(x) => eval_while(glob_env, fuel, env, out, x),
        ast::Stmt::Break => Ok(ControlFlow::Break),
    }
}

fn eval_prog(
    glob_env: &mut Env,
    fuel: &mut Fuel,
    env: &mut Env,
    out: &mut dyn io::Write,
    prog: &ast::Prog,
) -> Result<(), RuntimeError> {
    let ast::Prog(stmts) = prog;
    for stmt in stmts {
        match eval_stmt(glob_env, fuel, env, out, stmt)? {
            ControlFlow::Cont => (),
            ControlFlow::Break => Err(InternalCompilerError {
                msg: format!("break outside a loop, should have been a syntax error"),
            })?,
        }
    }
    Ok(())
}

pub struct Interpreter {
    fuel: Fuel,
    glob_env: Env,
    out: Box<dyn io::Write>,
}

impl Interpreter {
    fn new_global_env(clock: Option<Rc<RefCell<f64>>>) -> Env {
        let mut env = Env::new();
        for x in native_fns(clock).into_iter().map(Fn::Native) {
            env.define(VarName::new(x.name()), Val::Fn(Rc::new(x)))
        }
        env
    }

    pub fn new(out: Box<dyn io::Write>) -> Self {
        Self {
            fuel: Fuel::Infinite,
            out,
            glob_env: Self::new_global_env(None),
        }
    }

    #[cfg(test)]
    pub(super) fn new_for_test(
        out: Option<Rc<RefCell<Vec<u8>>>>,
        fuel: u64,
        clock: Option<Rc<RefCell<f64>>>,
    ) -> Self {
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
            glob_env: Self::new_global_env(clock),
        }
    }

    #[cfg(test)]
    pub(super) fn eval_expr(&mut self, expr: &ast::Expr) -> Result<Val, RuntimeError> {
        let mut env = self.glob_env.extend();
        eval_expr(
            &mut self.glob_env,
            &mut self.fuel,
            &mut env,
            &mut *self.out,
            expr,
        )
    }

    pub fn eval_prog(&mut self, prog: &ast::Prog) -> Result<(), RuntimeError> {
        let mut env = self.glob_env.extend();
        eval_prog(
            &mut self.glob_env,
            &mut &mut self.fuel,
            &mut env,
            &mut *self.out,
            prog,
        )
    }
}
