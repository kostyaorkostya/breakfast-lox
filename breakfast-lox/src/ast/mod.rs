use std::fmt;

// TODO(kostya): Implement a Wadler-Lindig approach. `pretty` crate might be useful.
pub trait Pretty {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

#[derive(Debug)]
pub struct NilLit;

#[derive(Debug)]
pub struct BoolLit(pub bool);

#[derive(Debug)]
pub struct NumLit(pub f64);

#[derive(Debug)]
pub struct StrLit(pub String);

#[derive(Debug)]
pub enum Lit {
    Nil(NilLit),
    Bool(BoolLit),
    Num(NumLit),
    Str(StrLit),
}

#[derive(Debug)]
pub enum UnOp {
    /// Negation (`-`)
    Neg,
    /// Logical not (`!`)
    Not,
}

#[derive(Debug)]
pub enum EqOp {
    /// Equality (`==`)
    Eq,
    /// Inequality (`!=`)
    Ne,
}

#[derive(Debug)]
pub enum CmpOp {
    /// Less than (`<`)
    Lt,
    /// Less than or equal to (`<=`)
    Le,
    /// Greater than (`>`)
    Gt,
    /// Greater than or equal to (`>=`)
    Ge,
}

#[derive(Debug)]
pub enum RelOp {
    Eq(EqOp),
    Cmp(CmpOp),
}

#[derive(Debug)]
pub enum AddOp {
    /// Addition (`+`)
    Add,
    /// Substraction (`-`)
    Sub,
}

#[derive(Debug)]
pub enum MulOp {
    /// Multiplication (`*`)
    Mul,
    /// Division (`-`)
    Div,
}

#[derive(Debug)]
pub enum BinOp {
    Rel(RelOp),
    Add(AddOp),
    Mul(MulOp),
}

#[derive(Debug)]
pub struct UnExpr {
    pub op: UnOp,
    pub e: Box<Expr>,
}

#[derive(Debug)]
pub struct BinExpr {
    pub op: BinOp,
    pub l: Box<Expr>,
    pub r: Box<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Lit(Lit),
    Un(UnExpr),
    Bin(BinExpr),
}

// impls for each struct

impl StrLit {
    /// Assumes ASCII
    pub fn from_raw(s: &str) -> Self {
        let s = &s[1..s.len() - 1]; // strip quotes
        let mut result = String::new();
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some(c) => panic!("unknown escape: \\{c}"),
                    None => panic!("trailing backslash"),
                }
            } else {
                result.push(c);
            }
        }
        Self(result)
    }
}

// impl for `Pretty`

impl Pretty for NilLit {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nil")
    }
}

impl Pretty for BoolLit {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Pretty for NumLit {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO(kostya): What precision and format should it use?
        write!(f, "{}", self.0)
    }
}

impl Pretty for StrLit {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // debug formatting should output escaped string
        write!(f, "{:?}", self.0)
    }
}

impl Pretty for Lit {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil(x) => x.pretty(f),
            Self::Bool(x) => x.pretty(f),
            Self::Num(x) => x.pretty(f),
            Self::Str(x) => x.pretty(f),
        }
    }
}

impl Pretty for UnOp {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Neg => write!(f, "-"),
            Self::Not => write!(f, "!"),
        }
    }
}

impl Pretty for EqOp {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
        }
    }
}

impl Pretty for CmpOp {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lt => write!(f, "<"),
            Self::Le => write!(f, "<="),
            Self::Gt => write!(f, ">"),
            Self::Ge => write!(f, ">="),
        }
    }
}

impl Pretty for RelOp {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eq(x) => x.pretty(f),
            Self::Cmp(x) => x.pretty(f),
        }
    }
}

impl Pretty for AddOp {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
        }
    }
}

impl Pretty for MulOp {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
}

impl Pretty for BinOp {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rel(x) => x.pretty(f),
            Self::Add(x) => x.pretty(f),
            Self::Mul(x) => x.pretty(f),
        }
    }
}
