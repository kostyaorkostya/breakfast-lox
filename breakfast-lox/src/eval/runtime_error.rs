use super::{OutOfFuelError, UndefinedVariableError};
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvalidOperandTypeError {
    #[error("cannot apply unary operator `-` on type `nil`")]
    UnOpNegOnNil,
    #[error("cannot apply unary operator `-` on type `bool`")]
    UnOpNegOnBool,
    #[error("cannot apply unary operator `-` on type `string`")]
    UnOpNegOnStr,
    #[error("cannot apply `==` or `!=`")]
    EqOp,
    #[error("cannot apply `<` or `<=` or `>` or `>=`")]
    CmpOp,
    #[error("cannot apply `+`")]
    AddOpAdd,
    #[error("cannot apply `-`")]
    AddOpSub,
    #[error("cannot apply `*` or `/`")]
    MulOp,
}

#[derive(Error, Debug)]
pub enum ArithmeticError {
    #[error("attempt to divide by zero")]
    DivisionByZero,
}

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("invalid operand type; {0}")]
    InvalidOperandType(#[from] InvalidOperandTypeError),
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
}
