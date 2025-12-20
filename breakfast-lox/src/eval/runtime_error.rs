use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvalidOperandTypeError {
    #[error("cannot apply unary operator `-` on type `nil`")]
    UnOpNegOnNil,
    #[error("cannot apply unary operator `-` on type `bool`")]
    UnOpNegOnBool,
    #[error("cannot apply unary operator `-` on type `string`")]
    UnOpNegOnStr,
    #[error("cannot apply unary operator `!` on type `number`")]
    UnOpNotOnNum,
    #[error("cannot apply unary operator `!` on type `string`")]
    UnOpNotOnStr,
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
pub enum RuntimeError {
    #[error("invalid operand type; {0}")]
    InvalidOperandType(#[from] InvalidOperandTypeError),
}
