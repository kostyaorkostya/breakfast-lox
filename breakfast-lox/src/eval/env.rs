use super::{Value, VarName};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("undefined variable '{name}'")]
pub struct UndefinedVariableError {
    pub name: VarName,
}

#[derive(Debug, Default)]
pub struct Env {
    values: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: VarName, value: Value) {
        self.values.insert(name.into_inner(), value);
    }

    pub fn get(&self, name: &str) -> Result<Value, UndefinedVariableError> {
        self.values
            .get(name)
            .cloned()
            .ok_or_else(|| UndefinedVariableError {
                name: VarName::new(name.to_string()),
            })
    }
}
