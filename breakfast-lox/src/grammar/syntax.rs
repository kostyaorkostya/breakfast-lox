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

pub fn validate_expr(_loop_depth: usize, _expr: &ast::Expr) -> Result<(), SyntaxError> {
    Ok(())
}

pub fn validate_var_name(_loop_depth: usize, var_name: &ast::VarName) -> Result<(), SyntaxError> {
    if KWRDS.contains(&var_name.as_str()) {
        Err(SyntaxError::ReservedKeyword(var_name.clone().into_inner()))
    } else {
        Ok(())
    }
}

pub fn validate_block(loop_depth: usize, block: &ast::Block) -> Result<(), SyntaxError> {
    let ast::Block(stmts) = block;
    for stmt in stmts {
        validate_stmt(loop_depth, stmt)?
    }
    Ok(())
}

pub fn validate_while_stmt(loop_depth: usize, while_: &ast::WhileStmt) -> Result<(), SyntaxError> {
    let ast::WhileStmt { cond, body } = while_;
    validate_expr(loop_depth, cond)?;
    validate_stmt(loop_depth, body)
}

pub fn validate_if_stmt(loop_depth: usize, if_: &ast::IfStmt) -> Result<(), SyntaxError> {
    let ast::IfStmt { cond, then, else_ } = if_;
    validate_expr(loop_depth, cond)?;
    validate_stmt(loop_depth, then)?;
    if let Some(else_) = else_ {
        validate_stmt(loop_depth, else_)
    } else {
        Ok(())
    }
}

pub fn validate_expr_stmt(loop_depth: usize, expr: &ast::ExprStmt) -> Result<(), SyntaxError> {
    let ast::ExprStmt(expr) = expr;
    validate_expr(loop_depth, expr)
}

pub fn validate_print_stmt(loop_depth: usize, print: &ast::PrintStmt) -> Result<(), SyntaxError> {
    let ast::PrintStmt(expr) = print;
    validate_expr(loop_depth, expr)
}

pub fn validate_var_decl(loop_depth: usize, var_decl: &ast::VarDecl) -> Result<(), SyntaxError> {
    let ast::VarDecl { name, init } = var_decl;
    validate_var_name(loop_depth, name)?;
    if let Some(init) = init {
        validate_expr(loop_depth, init)
    } else {
        Ok(())
    }
}

pub fn validate_stmt(loop_depth: usize, stmt: &ast::Stmt) -> Result<(), SyntaxError> {
    match stmt {
        ast::Stmt::Break => err_if_loop_depth_zero(loop_depth)?,
        ast::Stmt::Block(x) => validate_block(loop_depth, x)?,
        ast::Stmt::While(x) => validate_while_stmt(loop_depth + 1, x)?,
        ast::Stmt::If(x) => validate_if_stmt(loop_depth, x)?,
        ast::Stmt::Expr(x) => validate_expr_stmt(loop_depth, x)?,
        ast::Stmt::Print(x) => validate_print_stmt(loop_depth, x)?,
        ast::Stmt::VarDecl(x) => validate_var_decl(loop_depth, x)?,
    };
    Ok(())
}

pub fn validate_prog(prog: &ast::Prog) -> Result<(), SyntaxError> {
    let ast::Prog(stmts) = prog;
    for stmt in stmts {
        match stmt {
            ast::Stmt::Break => Err(SyntaxError::BreakOutsideLoop)?,
            _ => validate_stmt(0, stmt)?,
        }
    }
    Ok(())
}
