use super::{Value, VarName};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UndefinedVariableError {
    #[error("undefined variable '{0}'")]
    Undefined(String),
    #[error("cannot assign to undefined variable '{0}'")]
    AssignToUndefined(String),
}

#[derive(Debug, Default)]
pub struct Env {
    bindings: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: VarName, val: Value) {
        self.bindings.insert(name.into_inner(), val);
    }

    pub fn assign(&mut self, name: &str, val: Value) -> Result<(), UndefinedVariableError> {
        match self.bindings.get_mut(name) {
            None => Err(UndefinedVariableError::AssignToUndefined(name.to_owned())),
            Some(slot) => {
                *slot = val;
                Ok(())
            }
        }
    }

    pub fn get(&self, name: &str) -> Result<Value, UndefinedVariableError> {
        self.bindings
            .get(name)
            .cloned()
            .ok_or_else(|| UndefinedVariableError::Undefined(name.to_owned()))
    }
}
