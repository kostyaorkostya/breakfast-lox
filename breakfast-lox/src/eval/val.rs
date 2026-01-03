use super::{ArithmeticError, Fn, Stringify, Truthy, TypeError};
use crate::ast;
use std::fmt;
use std::rc::Rc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("{0}")]
    Arithmetic(#[from] ArithmeticError),
    #[error("{0}")]
    Type(#[from] TypeError),
}

impl From<RuntimeError> for super::RuntimeError {
    fn from(value: RuntimeError) -> Self {
        match value {
            RuntimeError::Type(x) => super::RuntimeError::Type(x),
            RuntimeError::Arithmetic(x) => super::RuntimeError::Arithmetic(x),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Val {
    Nil,
    Bool(bool),
    Num(f64),
    Str(String),
    Fn(Rc<Fn>),
}

impl Stringify for Val {
    fn stringify(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Bool(x) => write!(f, "{x}"),
            Self::Num(x) => write!(f, "{x}"),
            Self::Str(x) => write!(f, "{x}"),
            Self::Fn(x) => x.stringify(f),
        }
    }
}

// https://craftinginterpreters.com/evaluating-expressions.html#truthiness-and-falsiness
impl Truthy for Val {
    fn truthy(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Bool(x) => *x,
            Self::Num(_) | Self::Str(_) | Self::Fn(_) => true,
        }
    }
}

impl From<ast::Lit> for Val {
    fn from(value: ast::Lit) -> Self {
        match value {
            ast::Lit::Nil(ast::NilLit) => Self::Nil,
            ast::Lit::Bool(x) => Self::Bool(x.0),
            ast::Lit::Num(x) => Self::Num(x.0),
            ast::Lit::Str(x) => Self::Str(x.0),
        }
    }
}

impl Val {
    pub fn eval_un_op(self: &Self, op: &ast::UnOp) -> Result<Self, TypeError> {
        match (op, self) {
            (ast::UnOp::Neg, Self::Num(x)) => {
                // TODO(kostya): check if `-x` is representable
                Ok(Self::Num(-x))
            }
            (ast::UnOp::Not, e) => Ok(Self::Bool(!e.truthy())),
            (ast::UnOp::Neg, Self::Nil) => Err(TypeError {
                msg: format!("can't `-` nil"),
            }),
            (ast::UnOp::Neg, Self::Bool(_)) => Err(TypeError {
                msg: format!("can't `-` bool"),
            }),
            (ast::UnOp::Neg, Self::Str(_)) => Err(TypeError {
                msg: format!("can't `-` string"),
            }),
            (_, Self::Fn(this)) => Err(TypeError {
                msg: format!("can't {op:?} {this:?}"),
            }),
        }
    }

    pub fn eval_eq_op(self: &Self, op: &ast::EqOp, r: &Self) -> bool {
        let eq = match (self, r) {
            (Self::Nil, Self::Nil) => true,
            (Self::Bool(this), Self::Bool(r)) => this == r,
            (Self::Num(this), Self::Num(r)) => this == r,
            (Self::Str(this), Self::Str(r)) => this == r,
            (Self::Fn(this), Self::Fn(r)) => Rc::ptr_eq(this, r),
            (Self::Nil, _)
            | (Self::Bool(_), _)
            | (Self::Num(_), _)
            | (Self::Str(_), _)
            | (Self::Fn(_), _) => false,
        };
        match op {
            ast::EqOp::Eq => eq,
            ast::EqOp::Ne => !eq,
        }
    }

    pub fn eval_cmp_op(self: &Self, op: &ast::CmpOp, r: &Self) -> Result<bool, TypeError> {
        match (self, r) {
            (Self::Num(this), Self::Num(r)) => Ok(match op {
                ast::CmpOp::Lt => this < r,
                ast::CmpOp::Le => this <= r,
                ast::CmpOp::Gt => this > r,
                ast::CmpOp::Ge => this >= r,
            }),
            (Self::Nil, _)
            | (Self::Bool(_), _)
            | (Self::Num(_), _)
            | (Self::Str(_), _)
            | (Self::Fn(_), _) => Err(TypeError {
                msg: format!("can't compare `{self:?}` and `{r:?}`"),
            }),
        }
    }

    fn eval_add(self: &Self, r: &Self) -> Result<Self, TypeError> {
        match (self, r) {
            (Self::Num(this), Self::Num(r)) => Ok(Self::Num(this + r)),
            (Self::Str(this), Self::Str(r)) => Ok(Self::Str(format!("{this}{r}"))),
            (Self::Str(this), r @ Self::Num(_)) => {
                // Challenge 2 from https://craftinginterpreters.com/evaluating-expressions.html#running-the-interpreter
                // TODO(kostya): Apply some formatting rules to `r`?
                Ok(Self::Str(format!("{this}{}", r.display())))
            }
            (Self::Nil, _)
            | (Self::Bool(_), _)
            | (Self::Num(_), _)
            | (Self::Str(_), _)
            | (Self::Fn(_), _) => Err(TypeError {
                msg: format!("can't `+` `{self:?}` and `{r:?}`"),
            }),
        }
    }

    fn eval_sub(self: &Self, r: &Self) -> Result<f64, TypeError> {
        match (self, r) {
            (Val::Num(this), Val::Num(r)) => Ok(this - r),
            (Self::Nil, _)
            | (Self::Bool(_), _)
            | (Self::Num(_), _)
            | (Self::Str(_), _)
            | (Self::Fn(_), _) => Err(TypeError {
                msg: format!("can't `-` `{self:?}` and `{r:?}`"),
            }),
        }
    }

    pub fn eval_add_op(self: &Self, op: &ast::AddOp, r: &Self) -> Result<Val, TypeError> {
        match op {
            ast::AddOp::Add => self.eval_add(r),
            ast::AddOp::Sub => Ok(Self::Num(self.eval_sub(r)?)),
        }
    }

    pub fn eval_mul_op(self: &Self, op: &ast::MulOp, r: &Self) -> Result<f64, RuntimeError> {
        match (self, r) {
            (Val::Num(this), Val::Num(r)) => Ok(match op {
                ast::MulOp::Mul => Ok(this * r),
                ast::MulOp::Div => {
                    if *r == 0.0f64 {
                        Err(ArithmeticError::DivisionByZero)
                    } else {
                        Ok(this / r)
                    }
                }
            }?),
            (Self::Nil, _)
            | (Self::Bool(_), _)
            | (Self::Num(_), _)
            | (Self::Str(_), _)
            | (Self::Fn(_), _) => Err(TypeError {
                msg: format!("can't `{op:?}` `{self:?}` and `{r:?}`"),
            })?,
        }
    }
}
