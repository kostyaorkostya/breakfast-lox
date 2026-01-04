use std::ops;

use super::{EnvRef, RuntimeError, Stringify, Val, VarName};
use crate::ast;

pub struct NativeFn {
    name: String,
    arity: usize,
    pub fn_: Box<dyn ops::Fn(&EnvRef, &[Val]) -> Result<Val, RuntimeError>>,
}

impl std::fmt::Debug for NativeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeFn")
            .field("name", &self.name)
            .field("arity", &self.arity)
            .field("fn_", &"<native>")
            .finish()
    }
}

impl NativeFn {
    pub fn new<const ARITY: usize>(
        name: String,
        fn_: impl ops::Fn(&EnvRef, &[Val; ARITY]) -> Result<Val, RuntimeError> + 'static,
    ) -> Self {
        Self {
            name,
            arity: ARITY,
            fn_: Box::new(move |global, args| fn_(global, args.try_into().unwrap())),
        }
    }
}

#[derive(Debug)]
pub struct UserFn {
    pub name: Option<String>,
    pub params: Vec<VarName>,
    pub body: ast::Node<ast::Block>,
    pub env: EnvRef,
}

#[derive(Debug)]
pub enum Fn {
    Native(NativeFn),
    User(UserFn),
}

impl Stringify for Fn {
    fn stringify(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}/{}>", self.name(), self.arity())
    }
}

impl Fn {
    pub fn name(&self) -> &str {
        match self {
            Self::Native(NativeFn { name, .. }) => name,
            Self::User(UserFn { name, .. }) => name.as_deref().unwrap_or("(anon)"),
        }
    }
    pub fn arity(&self) -> usize {
        match self {
            Self::Native(NativeFn { arity, .. }) => *arity,
            Self::User(UserFn { params, .. }) => params.len(),
        }
    }
}
