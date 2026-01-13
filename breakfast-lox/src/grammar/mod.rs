use crate::ast;
use lalrpop_util::lalrpop_mod;

mod parse_error;
pub use parse_error::ParseError;

mod syntax_error;
pub use syntax_error::SyntaxError;

mod compile_error;
pub use compile_error::CompileError;

mod syntax;

mod grammar_support;

lalrpop_mod!(grammar, "/grammar/grammar.rs");

#[cfg(test)]
pub(crate) fn parse_expr(
    ids: &mut dyn ast::NodeIdGen,
    expr: &str,
) -> Result<ast::Node<ast::Expr>, CompileError> {
    let expr = match grammar::ExprParser::new().parse(ids, expr) {
        Ok(x) => Ok(x),
        Err(lalrpop_util::ParseError::User { error }) => Err(CompileError::Parse(error)),
        Err(x) => Err(CompileError::Lalrpop(anyhow::anyhow!("{x:?}"))),
    }?;
    Ok(expr)
}

pub fn parse_prog(
    ids: &mut dyn ast::NodeIdGen,
    prog: &str,
) -> Result<ast::Node<ast::Prog>, CompileError> {
    let prog = match grammar::ProgParser::new().parse(ids, prog) {
        Ok(x) => Ok(x),
        Err(lalrpop_util::ParseError::User { error }) => Err(CompileError::Parse(error)),
        Err(x) => Err(CompileError::Lalrpop(anyhow::anyhow!("{x:?}"))),
    }?;
    syntax::validate_prog(&prog)?;
    Ok(prog)
}

#[cfg(test)]
mod tests;
