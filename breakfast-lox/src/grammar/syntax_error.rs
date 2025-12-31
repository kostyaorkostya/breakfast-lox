use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("break outside loop")]
    BreakOutsideLoop,
    #[error("reserved keyword: '{0}'")]
    ReservedKeyword(String),
}
