use super::{Value, VarName};
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
}

#[derive(Debug, Default)]
struct Inner {
    // TODO(kostya): Once `Value` supports closures, `Env`s might form a loop that will leak.
    enclosing: Option<Rc<RefCell<Inner>>>,
    bindings: HashMap<String, Value>,
}

impl Inner {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::default()))
    }

    pub fn extend(env: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            enclosing: Some(Rc::clone(env)),
            ..Self::default()
        }))
    }

    pub fn define(&mut self, name: VarName, val: Value) {
        self.bindings.insert(name.into_inner(), val);
    }

    pub fn assign(&mut self, name: &str, val: Value) -> Result<(), UndefinedVariableError> {
        match self.bindings.get_mut(name) {
            Some(slot) => {
                *slot = val;
                Ok(())
            }
            None => match &self.enclosing {
                None => Err(UndefinedVariableError::AssignToUndefined(name.to_owned())),
                Some(env) => env.borrow_mut().assign(name, val),
            },
        }
    }

    pub fn get(&self, name: &str) -> Result<Value, UndefinedVariableError> {
        match self.bindings.get(name).cloned() {
            Some(x) => Ok(x),
            None => match &self.enclosing {
                None => Err(UndefinedVariableError::Undefined(name.to_owned())),
                Some(env) => env.borrow_mut().get(name),
            },
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Env(Rc<RefCell<Inner>>);

impl Env {
    pub fn new() -> Self {
        Self(Inner::new())
    }

    pub fn extend(&self) -> Self {
        Self(Inner::extend(&self.0))
    }

    pub fn define(&mut self, name: VarName, val: Value) {
        self.0.borrow_mut().define(name, val);
    }

    pub fn assign(&mut self, name: &str, val: Value) -> Result<(), UndefinedVariableError> {
        self.0.borrow_mut().assign(name, val)
    }

    pub fn get(&self, name: &str) -> Result<Value, UndefinedVariableError> {
        self.0.borrow().get(name)
    }
}
