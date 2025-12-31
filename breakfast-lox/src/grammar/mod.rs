use crate::ast;
use lalrpop_util::lalrpop_mod;

mod parse_error;
pub use parse_error::ParseError;

mod syntax_error;
pub use syntax_error::SyntaxError;

mod compile_error;
pub use compile_error::CompileError;

mod grammar_support;

lalrpop_mod!(grammar, "/grammar/grammar.rs");
use grammar::{ExprParser, ProgParser};

pub(crate) fn parse_expr(expr: &str) -> Result<ast::Expr, CompileError> {
    let ast = match ExprParser::new().parse(expr) {
        Ok(x) => Ok(x),
        Err(lalrpop_util::ParseError::User { error }) => Err(CompileError::Parse(error)),
        Err(x) => Err(CompileError::Lalrpop(anyhow::anyhow!("{x:?}"))),
    }?;
    Ok(ast)
}

pub fn parse_prog(prog: &str) -> Result<ast::Prog, CompileError> {
    let ast = match ProgParser::new().parse(prog) {
        Ok(x) => Ok(x),
        Err(lalrpop_util::ParseError::User { error }) => Err(CompileError::Parse(error)),
        Err(x) => Err(CompileError::Lalrpop(anyhow::anyhow!("{x:?}"))),
    }?;
    Ok(ast)
}

#[cfg(test)]
mod tests;
