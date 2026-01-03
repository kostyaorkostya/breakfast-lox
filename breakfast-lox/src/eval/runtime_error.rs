use super::{OutOfFuelError, UndefinedVariableError, VariableRedeclarationError};
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("type error: '{msg}'")]
pub struct TypeError {
    pub msg: String,
}

#[derive(Error, Debug)]
#[error("internal compiler error `{msg}`")]
pub struct InternalCompilerError {
    pub msg: String,
}

#[derive(Error, Debug)]
pub enum ArithmeticError {
    #[error("attempt to divide by zero")]
    DivisionByZero,
}

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("arithmetic error; {0}")]
    Arithmetic(#[from] ArithmeticError),
    #[error("IO error; {0}")]
    InputOutput(#[from] io::Error),
    #[error("undefined variable; {0}")]
    UndefinedVariable(#[from] UndefinedVariableError),
    #[error("unimplemented")]
    Unimplemented,
    #[error("{0}")]
    Fuel(#[from] OutOfFuelError),
    #[error("{0}")]
    Type(#[from] TypeError),
    #[error("internal error: `{0}`")]
    Internal(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("{0}")]
    Compiler(#[from] InternalCompilerError),
    #[error("{0}")]
    VariableRedeclaration(#[from] VariableRedeclarationError),
}
