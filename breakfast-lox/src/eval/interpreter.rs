use super::{
    ControlFlow, InternalCompilerError, NativeFn, RuntimeError, Stringify, Truthy, TypeError,
    UserFn, VarName,
};
use super::{Env, EnvRef, Fn, Fuel, Val, native_fns};
use crate::ast;
use std::cell::RefCell;
use std::io;
use std::iter;
use std::rc::Rc;

fn eval_un_expr(
    glob_env: &EnvRef,
    fuel: &mut Fuel,
    env: &EnvRef,
    out: &mut dyn io::Write,
    op: &ast::UnOp,
    e: &ast::Expr,
) -> Result<Val, RuntimeError> {
    let val = eval_expr(glob_env, fuel, env, out, &e)?;
    fuel.burn()?;
    Ok(val.eval_un_op(op)?)
}

fn eval_bin_expr(
    glob_env: &EnvRef,
    fuel: &mut Fuel,
    env: &EnvRef,
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
    glob_env: &EnvRef,
    fuel: &mut Fuel,
    env: &EnvRef,
    out: &mut dyn io::Write,
    assign: &ast::Assign,
) -> Result<Val, RuntimeError> {
    let ast::Assign { name, val } = assign;
    let val = eval_expr(glob_env, fuel, env, out, val)?;
    fuel.burn()?;
    env.borrow_mut().assign(name, val.clone())?;
    Ok(val)
}

fn eval_call(
    glob_env: &EnvRef,
    fuel: &mut Fuel,
    env: &EnvRef,
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
                    let env = Env::extend(&glob_env);
                    fn_(&env, &args)
                }
                Fn::User(UserFn {
                    name: _,
                    params,
                    body,
                    env,
                }) => {
                    match eval_block(
                        glob_env,
                        fuel,
                        &env,
                        out,
                        body,
                        iter::zip(params.iter().cloned(), args.into_iter()),
                    )? {
                        ControlFlow::Ret(val) => Ok(val),
                        ControlFlow::Cont => {
                            // https://craftinginterpreters.com/functions.html#return-statements
                            Ok(Val::Nil)
                        }
                        ControlFlow::Break => Err(InternalCompilerError {
                            msg: format!("break outside a loop, should have been a syntax error"),
                        })?,
                    }
                }
            }
        }
    }
}

fn eval_fun(
    _glob_env: &EnvRef,
    fuel: &mut Fuel,
    env: &EnvRef,
    _out: &mut dyn io::Write,
    fun: &ast::Fun,
    name: Option<VarName>,
) -> Result<Val, RuntimeError> {
    let ast::Fun { params, body } = fun;
    fuel.burn()?;
    Ok(Val::Fn(Rc::new(Fn::User(UserFn {
        name: name.map(VarName::into_inner),
        params: params.iter().cloned().map(Into::into).collect(),
        body: body.clone(),
        env: env.clone(),
    }))))
}

fn eval_expr(
    glob_env: &EnvRef,
    fuel: &mut Fuel,
    env: &EnvRef,
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
            Ok(env.borrow().get(&**x)?)
        }
        ast::Expr::Assign(x) => eval_assign(glob_env, fuel, env, out, x),
        ast::Expr::Call(x) => eval_call(glob_env, fuel, env, out, x),
        ast::Expr::Fun(x) => eval_fun(glob_env, fuel, env, out, x, None),
    }
}

fn eval_var_decl(
    glob_env: &EnvRef,
    fuel: &mut Fuel,
    env: &EnvRef,
    out: &mut dyn io::Write,
    var_decl: &ast::VarDecl,
) -> Result<ControlFlow, RuntimeError> {
    let ast::VarDecl { name, init } = var_decl;
    match init {
        None => {
            fuel.burn()?;
            // Challenge 2 from https://craftinginterpreters.com/statements-and-state.html#challenges
            env.borrow_mut().declare(name.clone().into())?
        }
        Some(init) => {
            let init = eval_expr(glob_env, fuel, env, out, init)?;
            env.borrow_mut().define(name.clone().into(), init)?
        }
    }
    Ok(ControlFlow::Cont)
}

fn eval_block(
    glob_env: &EnvRef,
    fuel: &mut Fuel,
    env: &EnvRef,
    out: &mut dyn io::Write,
    block: &ast::Block,
    extra_vars: impl Iterator<Item = (VarName, Val)>,
) -> Result<ControlFlow, RuntimeError> {
    let ast::Block(stmts) = block;
    let env = Env::extend(env);
    for (var_name, val) in extra_vars {
        env.borrow_mut().define(var_name, val)?
    }
    for stmt in stmts {
        match eval_stmt(glob_env, fuel, &env, out, stmt)? {
            x @ (ControlFlow::Break | ControlFlow::Ret(_)) => return Ok(x),
            ControlFlow::Cont => (),
        }
    }
    Ok(ControlFlow::Cont)
}

fn eval_if(
    glob_env: &EnvRef,
    fuel: &mut Fuel,
    env: &EnvRef,
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
    glob_env: &EnvRef,
    fuel: &mut Fuel,
    env: &EnvRef,
    out: &mut dyn io::Write,
    while_: &ast::WhileStmt,
) -> Result<ControlFlow, RuntimeError> {
    let ast::WhileStmt { cond, body } = while_;
    while eval_expr(glob_env, fuel, env, out, cond)?.truthy() {
        match eval_stmt(glob_env, fuel, env, out, body)? {
            x @ ControlFlow::Ret(_) => return Ok(x),
            ControlFlow::Break => break,
            ControlFlow::Cont => (),
        }
    }
    Ok(ControlFlow::Cont)
}

fn eval_fun_decl(
    glob_env: &EnvRef,
    fuel: &mut Fuel,
    env: &EnvRef,
    out: &mut dyn io::Write,
    fun_decl: &ast::FunDecl,
) -> Result<ControlFlow, RuntimeError> {
    let ast::FunDecl { name, fun } = fun_decl;
    let name: VarName = name.clone().into();
    let val = eval_fun(glob_env, fuel, env, out, fun, Some(name.clone()))?;
    env.borrow_mut().define(name, val);
    Ok(ControlFlow::Cont)
}

fn eval_return(
    glob_env: &EnvRef,
    fuel: &mut Fuel,
    env: &EnvRef,
    out: &mut dyn io::Write,
    return_: &ast::RetStmt,
) -> Result<ControlFlow, RuntimeError> {
    let ast::RetStmt(val) = return_;
    let val = match val {
        None => Val::Nil,
        Some(expr) => eval_expr(glob_env, fuel, env, out, expr)?,
    };
    fuel.burn()?;
    Ok(ControlFlow::Ret(val))
}

fn eval_stmt(
    glob_env: &EnvRef,
    fuel: &mut Fuel,
    env: &EnvRef,
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
        ast::Stmt::Block(x) => eval_block(glob_env, fuel, env, out, x, iter::empty()),
        ast::Stmt::If(x) => eval_if(glob_env, fuel, env, out, x),
        ast::Stmt::While(x) => eval_while(glob_env, fuel, env, out, x),
        ast::Stmt::Break => Ok(ControlFlow::Break),
        ast::Stmt::FunDecl(x) => eval_fun_decl(glob_env, fuel, env, out, x),
        ast::Stmt::Ret(x) => eval_return(glob_env, fuel, env, out, x),
    }
}

fn eval_prog(
    glob_env: &EnvRef,
    fuel: &mut Fuel,
    env: &EnvRef,
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
            ControlFlow::Ret(x) => Err(InternalCompilerError {
                msg: format!("return `{x:?}` outside a function, should have been a syntax error"),
            })?,
        }
    }
    Ok(())
}

pub struct Interpreter {
    fuel: Fuel,
    glob_env: EnvRef,
    out: Box<dyn io::Write>,
}

impl Interpreter {
    fn new_global_env(clock: Option<Rc<RefCell<f64>>>) -> Result<EnvRef, RuntimeError> {
        let env = Env::new();
        for x in native_fns(clock).into_iter().map(Fn::Native) {
            env.borrow_mut()
                .define(VarName::new(x.name()), Val::Fn(Rc::new(x)))?
        }
        Ok(env)
    }

    pub fn new(out: Box<dyn io::Write>) -> Result<Self, RuntimeError> {
        Ok(Self {
            fuel: Fuel::Infinite,
            out,
            glob_env: Self::new_global_env(None)?,
        })
    }

    #[cfg(test)]
    pub(super) fn new_for_test(
        out: Option<Rc<RefCell<Vec<u8>>>>,
        fuel: u64,
        clock: Option<Rc<RefCell<f64>>>,
    ) -> Result<Self, RuntimeError> {
        struct SharedWriter(Rc<RefCell<Vec<u8>>>);

        impl io::Write for SharedWriter {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.0.borrow_mut().write(buf)
            }

            fn flush(&mut self) -> io::Result<()> {
                self.0.borrow_mut().flush()
            }
        }
        Ok(Self {
            fuel: Fuel::Finite(fuel),
            out: match out {
                Some(x) => Box::new(SharedWriter(x)),
                None => Box::new(io::sink()),
            },
            glob_env: Self::new_global_env(clock)?,
        })
    }

    #[cfg(test)]
    pub(super) fn eval_expr(&mut self, expr: &ast::Expr) -> Result<Val, RuntimeError> {
        let mut env = Env::extend(&self.glob_env);
        eval_expr(
            &mut self.glob_env,
            &mut self.fuel,
            &mut env,
            &mut *self.out,
            expr,
        )
    }

    pub fn eval_prog(&mut self, prog: &ast::Prog) -> Result<(), RuntimeError> {
        let mut env = Env::extend(&self.glob_env);
        eval_prog(
            &mut self.glob_env,
            &mut &mut self.fuel,
            &mut env,
            &mut *self.out,
            prog,
        )
    }
}
