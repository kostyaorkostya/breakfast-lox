use super::SyntaxError;
use crate::ast;

const KWRDS: &[&str] = &[
    "nil", "print", "var", "true", "false", "if", "else", "or", "and", "while", "for", "break",
    "fun", "return",
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
    _expr: &ast::Node<ast::Expr>,
) -> Result<(), SyntaxError> {
    Ok(())
}

pub fn validate_var_name(
    _in_func: bool,
    _loop_depth: usize,
    var_name: &ast::Node<ast::VarName>,
) -> Result<(), SyntaxError> {
    if KWRDS.contains(&var_name.kind.as_str()) {
        Err(SyntaxError::ReservedKeyword(
            var_name.kind.clone().into_inner(),
        ))
    } else {
        Ok(())
    }
}

pub fn validate_block(
    in_func: bool,
    loop_depth: usize,
    block: &ast::Node<ast::Block>,
) -> Result<(), SyntaxError> {
    let ast::Block(stmts) = &block.kind;
    for stmt in stmts {
        validate_stmt(in_func, loop_depth, stmt)?
    }
    Ok(())
}

pub fn validate_while_stmt(
    in_func: bool,
    loop_depth: usize,
    while_: &ast::Node<ast::WhileStmt>,
) -> Result<(), SyntaxError> {
    let ast::WhileStmt { cond, body } = &while_.kind;
    validate_expr(in_func, loop_depth, cond)?;
    validate_stmt(in_func, loop_depth, body)
}

pub fn validate_if_stmt(
    in_func: bool,
    loop_depth: usize,
    if_: &ast::Node<ast::IfStmt>,
) -> Result<(), SyntaxError> {
    let ast::IfStmt { cond, then, else_ } = &if_.kind;
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
    expr: &ast::Node<ast::ExprStmt>,
) -> Result<(), SyntaxError> {
    let ast::ExprStmt(expr) = &expr.kind;
    validate_expr(in_func, loop_depth, expr)
}

pub fn validate_print_stmt(
    in_func: bool,
    loop_depth: usize,
    print: &ast::Node<ast::PrintStmt>,
) -> Result<(), SyntaxError> {
    let ast::PrintStmt(expr) = &print.kind;
    validate_expr(in_func, loop_depth, expr)
}

pub fn validate_var_decl(
    in_func: bool,
    loop_depth: usize,
    var_decl: &ast::Node<ast::VarDecl>,
) -> Result<(), SyntaxError> {
    let ast::VarDecl { name, init } = &var_decl.kind;
    validate_var_name(in_func, loop_depth, name)?;
    if let Some(init) = init {
        validate_expr(in_func, loop_depth, init)
    } else {
        Ok(())
    }
}

pub fn validate_fun(
    in_func: bool,
    loop_depth: usize,
    fun_decl: &ast::Node<ast::Fun>,
) -> Result<(), SyntaxError> {
    let ast::Fun { params, body } = &fun_decl.kind;
    for param in params {
        validate_var_name(in_func, loop_depth, param)?;
    }
    validate_block(true, loop_depth, body)
}

pub fn validate_fun_decl(
    in_func: bool,
    loop_depth: usize,
    fun_decl: &ast::Node<ast::FunDecl>,
) -> Result<(), SyntaxError> {
    let ast::FunDecl { name, fun } = &fun_decl.kind;
    validate_var_name(in_func, loop_depth, name)?;
    validate_fun(in_func, loop_depth, fun)
}

pub fn validate_ret_stmt(
    in_func: bool,
    loop_depth: usize,
    ret_stmt: &ast::Node<ast::RetStmt>,
) -> Result<(), SyntaxError> {
    let ast::RetStmt(val) = &ret_stmt.kind;
    err_if_not_in_func(in_func)?;
    if let Some(expr) = val {
        validate_expr(in_func, loop_depth, expr)?;
    }
    Ok(())
}

pub fn validate_stmt(
    in_func: bool,
    loop_depth: usize,
    stmt: &ast::Node<ast::Stmt>,
) -> Result<(), SyntaxError> {
    match &stmt.kind {
        ast::Stmt::Break(_) => err_if_loop_depth_zero(loop_depth)?,
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

pub fn validate_prog(prog: &ast::Node<ast::Prog>) -> Result<(), SyntaxError> {
    let ast::Prog(stmts) = &prog.kind;
    for stmt in stmts {
        match stmt.kind {
            ast::Stmt::Break(_) => Err(SyntaxError::BreakOutsideLoop)?,
            _ => validate_stmt(false, 0, stmt)?,
        }
    }
    Ok(())
}
