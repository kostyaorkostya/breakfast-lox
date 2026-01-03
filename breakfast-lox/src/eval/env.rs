use super::{Val, VarName};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UndefinedVariableError {
    #[error("undefined variable '{0}'")]
    Undefined(String),
    #[error("cannot assign to undefined variable '{0}'")]
    AssignToUndefined(String),
    #[error("variable '{0}' used before initialization")]
    AccessUninitialized(String),
}

#[derive(Debug, Error)]
#[error("variable redeclaration '{var_name}'")]
pub struct VariableRedeclarationError {
    var_name: String,
}

pub type EnvRef = Rc<RefCell<Env>>;

#[derive(Debug, Default)]
pub struct Env {
    // TODO(kostya): Once `Val` supports closures, `Env`s might form a loop that will leak.
    enclosing: Option<EnvRef>,
    bindings: HashMap<String, Option<Val>>,
}

impl Env {
    pub fn new() -> EnvRef {
        Rc::new(RefCell::new(Self::default()))
    }

    pub fn extend(env: &EnvRef) -> EnvRef {
        Rc::new(RefCell::new(Self {
            enclosing: Some(Rc::clone(env)),
            ..Self::default()
        }))
    }

    fn declare_or_define(
        &mut self,
        name: VarName,
        val: Option<Val>,
    ) -> Result<(), VariableRedeclarationError> {
        let var_name = name.into_inner();
        if self.bindings.contains_key(&var_name) {
            return Err(VariableRedeclarationError { var_name });
        }
        self.bindings.insert(var_name, val);
        Ok(())
    }

    pub fn declare(&mut self, name: VarName) -> Result<(), VariableRedeclarationError> {
        self.declare_or_define(name, None)
    }

    pub fn define(&mut self, name: VarName, val: Val) -> Result<(), VariableRedeclarationError> {
        self.declare_or_define(name, Some(val))
    }

    pub fn assign(&mut self, name: &str, val: Val) -> Result<(), UndefinedVariableError> {
        match self.bindings.get_mut(name) {
            Some(slot) => {
                *slot = Some(val);
                Ok(())
            }
            None => match &self.enclosing {
                None => Err(UndefinedVariableError::AssignToUndefined(name.to_owned())),
                Some(env) => env.borrow_mut().assign(name, val),
            },
        }
    }

    pub fn get(&self, name: &str) -> Result<Val, UndefinedVariableError> {
        match self.bindings.get(name).cloned() {
            Some(Some(x)) => Ok(x),
            Some(None) => Err(UndefinedVariableError::AccessUninitialized(name.to_owned())),
            None => match &self.enclosing {
                None => Err(UndefinedVariableError::Undefined(name.to_owned())),
                Some(env) => env.borrow_mut().get(name),
            },
        }
    }
}
