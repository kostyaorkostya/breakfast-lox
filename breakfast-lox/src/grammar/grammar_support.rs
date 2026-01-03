use super::parse_error::{
    DecimalFloatingPointLiteralParseError, HexadecimalFloatingPointLiteralParseError,
    IntegerLiteralParseError, NumLitParseError, NumberIsNotFiniteParseError, ParseError,
    TooManyArguments,
};
use crate::ast;
use std::str::FromStr;

// https://craftinginterpreters.com/functions.html#maximum-argument-counts
const MAX_ARG_COUNT: usize = 255;

pub enum ForInit {
    VarDecl(ast::VarDecl),
    Expr(ast::ExprStmt),
}

pub fn desugar_for_stmt(
    init: Option<ForInit>,
    cond: Option<ast::Expr>,
    incr: Option<ast::Expr>,
    body: ast::Stmt,
) -> ast::Stmt {
    let body = if let Some(incr) = incr {
        ast::Stmt::Block(Box::new(ast::Block(vec![
            ast::Stmt::Block(Box::new(ast::Block(vec![body]))),
            ast::Stmt::Expr(ast::ExprStmt(incr)),
        ])))
    } else {
        body
    };
    let cond = if let Some(cond) = cond {
        cond
    } else {
        ast::Expr::Lit(ast::Lit::Bool(ast::BoolLit(true)))
    };
    match init {
        None => ast::Stmt::While(Box::new(ast::WhileStmt { cond, body })),
        Some(init) => {
            let init = match init {
                ForInit::VarDecl(x) => ast::Stmt::VarDecl(x),
                ForInit::Expr(x) => ast::Stmt::Expr(x),
            };
            ast::Stmt::Block(Box::new(ast::Block(vec![
                init,
                ast::Stmt::While(Box::new(ast::WhileStmt { cond, body })),
            ])))
        }
    }
}

// TODO(kostya): These [is_finite] checks don't actually work in practice, there should be a better
// way of doing it.
pub fn parse_decimal_float(token: &str) -> Result<ast::NumLit, ParseError> {
    let x = f64::from_str(token).map_err(|source| {
        NumLitParseError::DecimalFloatingPointLiteral(DecimalFloatingPointLiteralParseError {
            token: token.into(),
            source: source.into(),
        })
    })?;
    if x.is_finite() {
        Ok(ast::NumLit(x))
    } else {
        Err(NumLitParseError::NumberIsNotFinite(
            NumberIsNotFiniteParseError {
                token: token.into(),
            },
        ))?
    }
}

pub fn parse_hex_float(token: &str) -> Result<ast::NumLit, ParseError> {
    let x = hexf_parse::parse_hexf64(token, false).map_err(|source| {
        NumLitParseError::HexadecimalFloatingPointLiteral(
            HexadecimalFloatingPointLiteralParseError {
                token: token.into(),
                source: source.into(),
            },
        )
    })?;
    if x.is_finite() {
        Ok(ast::NumLit(x))
    } else {
        Err(NumLitParseError::NumberIsNotFinite(
            NumberIsNotFiniteParseError {
                token: token.into(),
            },
        ))?
    }
}

pub fn parse_decimal_int_as_float(token: &str) -> Result<ast::NumLit, ParseError> {
    let x = f64::from_str(token).map_err(|source| {
        NumLitParseError::IntegerLiteral(IntegerLiteralParseError {
            token: token.into(),
            source: source.into(),
        })
    })?;
    if x.is_finite() {
        Ok(ast::NumLit(x))
    } else {
        Err(NumLitParseError::NumberIsNotFinite(
            NumberIsNotFiniteParseError {
                token: token.into(),
            },
        ))?
    }
}

pub fn validate_arg_count(got: usize) -> Result<(), ParseError> {
    if got > MAX_ARG_COUNT {
        Err(TooManyArguments {
            got,
            max: MAX_ARG_COUNT,
        })?
    } else {
        Ok(())
    }
}
