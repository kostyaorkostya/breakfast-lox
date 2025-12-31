use super::{ParseError, SyntaxError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("parse error: '{0}'")]
    Parse(#[from] ParseError),
    #[error("syntax error: '{0}'")]
    Syntax(#[from] SyntaxError),
    #[error("lalrpop error: '{0}'")]
    Lalrpop(#[from] anyhow::Error),
}
