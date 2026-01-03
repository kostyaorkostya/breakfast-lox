use super::Val;

#[derive(Debug)]
pub enum ControlFlow {
    Cont,
    Break,
    Ret(Val),
}
