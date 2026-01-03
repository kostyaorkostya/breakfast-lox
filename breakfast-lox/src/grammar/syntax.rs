use super::SyntaxError;
use crate::ast;

const KWRDS: &[&str] = &[
    "nil", "print", "var", "true", "false", "if", "else", "or", "and", "while", "for", "break",
];

fn err_if_loop_depth_zero(loop_depth: usize) -> Result<(), SyntaxError> {
    if loop_depth == 0 {
        Err(SyntaxError::BreakOutsideLoop)
    } else {
        Ok(())
    }
}

fn err_if_not_in_func(in_func: bool) -> Result<(), SyntaxError> {
    if in_func {
        Ok(())
    } else {
        Err(SyntaxError::ReturnOutsideFunction)
    }
}

pub fn validate_expr(
    _in_func: bool,
    _loop_depth: usize,
    _expr: &ast::Expr,
) -> Result<(), SyntaxError> {
    Ok(())
}

pub fn validate_var_name(
    _in_func: bool,
    _loop_depth: usize,
    var_name: &ast::VarName,
) -> Result<(), SyntaxError> {
    if KWRDS.contains(&var_name.as_str()) {
        Err(SyntaxError::ReservedKeyword(var_name.clone().into_inner()))
    } else {
        Ok(())
    }
}

pub fn validate_block(
    in_func: bool,
    loop_depth: usize,
    block: &ast::Block,
) -> Result<(), SyntaxError> {
    let ast::Block(stmts) = block;
    for stmt in stmts {
        validate_stmt(in_func, loop_depth, stmt)?
    }
    Ok(())
}

pub fn validate_while_stmt(
    in_func: bool,
    loop_depth: usize,
    while_: &ast::WhileStmt,
) -> Result<(), SyntaxError> {
    let ast::WhileStmt { cond, body } = while_;
    validate_expr(in_func, loop_depth, cond)?;
    validate_stmt(in_func, loop_depth, body)
}

pub fn validate_if_stmt(
    in_func: bool,
    loop_depth: usize,
    if_: &ast::IfStmt,
) -> Result<(), SyntaxError> {
    let ast::IfStmt { cond, then, else_ } = if_;
    validate_expr(in_func, loop_depth, cond)?;
    validate_stmt(in_func, loop_depth, then)?;
    if let Some(else_) = else_ {
        validate_stmt(in_func, loop_depth, else_)
    } else {
        Ok(())
    }
}

pub fn validate_expr_stmt(
    in_func: bool,
    loop_depth: usize,
    expr: &ast::ExprStmt,
) -> Result<(), SyntaxError> {
    let ast::ExprStmt(expr) = expr;
    validate_expr(in_func, loop_depth, expr)
}

pub fn validate_print_stmt(
    in_func: bool,
    loop_depth: usize,
    print: &ast::PrintStmt,
) -> Result<(), SyntaxError> {
    let ast::PrintStmt(expr) = print;
    validate_expr(in_func, loop_depth, expr)
}

pub fn validate_var_decl(
    in_func: bool,
    loop_depth: usize,
    var_decl: &ast::VarDecl,
) -> Result<(), SyntaxError> {
    let ast::VarDecl { name, init } = var_decl;
    validate_var_name(in_func, loop_depth, name)?;
    if let Some(init) = init {
        validate_expr(in_func, loop_depth, init)
    } else {
        Ok(())
    }
}

pub fn validate_fun_decl(
    in_func: bool,
    loop_depth: usize,
    fun_decl: &ast::FunDecl,
) -> Result<(), SyntaxError> {
    let ast::FunDecl { name, params, body } = fun_decl;
    validate_var_name(in_func, loop_depth, name)?;
    for param in params {
        validate_var_name(in_func, loop_depth, param)?;
    }
    validate_block(true, loop_depth, body)
}

pub fn validate_ret_stmt(
    in_func: bool,
    loop_depth: usize,
    ret_stmt: &ast::RetStmt,
) -> Result<(), SyntaxError> {
    let ast::RetStmt(val) = ret_stmt;
    err_if_not_in_func(in_func)?;
    if let Some(expr) = val {
        validate_expr(in_func, loop_depth, expr)?;
    }
    Ok(())
}

pub fn validate_stmt(
    in_func: bool,
    loop_depth: usize,
    stmt: &ast::Stmt,
) -> Result<(), SyntaxError> {
    match stmt {
        ast::Stmt::Break => err_if_loop_depth_zero(loop_depth)?,
        ast::Stmt::Block(x) => validate_block(in_func, loop_depth, x)?,
        ast::Stmt::While(x) => validate_while_stmt(in_func, loop_depth + 1, x)?,
        ast::Stmt::If(x) => validate_if_stmt(in_func, loop_depth, x)?,
        ast::Stmt::Expr(x) => validate_expr_stmt(in_func, loop_depth, x)?,
        ast::Stmt::Print(x) => validate_print_stmt(in_func, loop_depth, x)?,
        ast::Stmt::VarDecl(x) => validate_var_decl(in_func, loop_depth, x)?,
        ast::Stmt::FunDecl(x) => validate_fun_decl(in_func, 0, x)?,
        ast::Stmt::Ret(x) => validate_ret_stmt(in_func, loop_depth, x)?,
    };
    Ok(())
}

pub fn validate_prog(prog: &ast::Prog) -> Result<(), SyntaxError> {
    let ast::Prog(stmts) = prog;
    for stmt in stmts {
        match stmt {
            ast::Stmt::Break => Err(SyntaxError::BreakOutsideLoop)?,
            _ => validate_stmt(false, 0, stmt)?,
        }
    }
    Ok(())
}
