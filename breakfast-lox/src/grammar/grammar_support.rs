use super::parse_error::{
    DecimalFloatingPointLiteralParseError, HexadecimalFloatingPointLiteralParseError,
    IntegerLiteralParseError, NumLitParseError, NumberIsNotFiniteParseError, ParseError,
    TooManyArguments,
};
use crate::ast;
use std::ops::Range;
use std::str::FromStr;

// https://craftinginterpreters.com/functions.html#maximum-argument-counts
const MAX_ARG_COUNT: usize = 255;

pub enum ForInit {
    VarDecl(ast::VarDecl),
    Expr(ast::ExprStmt),
}

// https://craftinginterpreters.com/control-flow.html#for-loops
pub fn desugar_for_stmt(
    ids: &mut dyn ast::NodeIdGen,
    loc: Range<usize>,
    init: Option<ast::Node<ForInit>>,
    cond: Option<ast::Node<ast::Expr>>,
    incr: Option<ast::Node<ast::Expr>>,
    body: ast::Node<ast::Stmt>,
) -> ast::Node<ast::Stmt> {
    let body = if let Some(incr) = incr {
        let body = ast::synth_node(ids, ast::Stmt::Block(ast::Block(vec![body])));
        let incr = ast::synth_node(ids, ast::Stmt::Expr(ast::ExprStmt(incr)));
        ast::synth_node(ids, ast::Stmt::Block(ast::Block(vec![body, incr])))
    } else {
        body
    };
    let cond = if let Some(cond) = cond {
        cond
    } else {
        ast::synth_node(ids, ast::Expr::Lit(ast::Lit::Bool(ast::BoolLit(true))))
    };
    match init {
        None => ast::synth_with_loc_node(
            ids,
            loc,
            ast::Stmt::While(ast::WhileStmt {
                cond,
                body: Box::new(body),
            }),
        ),
        Some(init) => {
            let init = init.map(|x| match x {
                ForInit::VarDecl(x) => ast::Stmt::VarDecl(x),
                ForInit::Expr(x) => ast::Stmt::Expr(x),
            });
            let while_ = ast::synth_node(
                ids,
                ast::Stmt::While(ast::WhileStmt {
                    cond,
                    body: Box::new(body),
                }),
            );
            ast::synth_with_loc_node(ids, loc, ast::Stmt::Block(ast::Block(vec![init, while_])))
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
