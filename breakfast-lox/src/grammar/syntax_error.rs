use thiserror::Error;

#[derive(Error, Debug)]
#[error("syntax error")]
pub struct SyntaxError;
