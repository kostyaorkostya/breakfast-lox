use std::fmt;

// TODO(kostya): Implement a Wadler-Lindig approach. `pretty` crate might be useful.
pub trait Pretty {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    fn display(&self) -> impl fmt::Display + '_ {
        struct Adapter<'a, T: Pretty + ?Sized>(&'a T);
        impl<T: Pretty + ?Sized> fmt::Display for Adapter<'_, T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.pretty(f)
            }
        }
        Adapter(self)
    }
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
pub struct Var(pub String);

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
    Var(Var),
}

#[derive(Debug)]
pub struct VarDecl {
    pub var: Var,
    pub init: Option<Expr>,
}

#[derive(Debug)]
pub struct PrintStmt(pub Expr);

#[derive(Debug)]
pub struct ExprStmt(pub Expr);

#[derive(Debug)]
pub enum Stmt {
    Expr(ExprStmt),
    Print(PrintStmt),
    VarDecl(VarDecl),
}

#[derive(Debug)]
pub struct Prog(pub Vec<Stmt>);

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
        let Self(x) = self;
        write!(f, "{}", x)
    }
}

impl Pretty for NumLit {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(x) = self;
        // TODO(kostya): What precision and format should it use?
        write!(f, "{}", x)
    }
}

impl Pretty for StrLit {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(x) = self;
        // debug formatting should output escaped string
        write!(f, "{:?}", x)
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

impl Pretty for Var {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(x) = self;
        write!(f, "{}", x)
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

impl Pretty for UnExpr {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { op, e } = self;
        write!(f, "{}({})", op.display(), e.display())
    }
}

impl Pretty for BinExpr {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { op, l, r } = self;
        write!(f, "({} {} {})", l.display(), op.display(), r.display())
    }
}

impl Pretty for Expr {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lit(x) => x.pretty(f),
            Self::Un(x) => x.pretty(f),
            Self::Bin(x) => x.pretty(f),
            Self::Var(x) => x.pretty(f),
        }
    }
}

impl Pretty for VarDecl {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { var, init } = self;
        match init {
            Some(init) => writeln!(f, "var {} = {};", var.display(), init.display()),
            None => writeln!(f, "var {};", var.display()),
        }
    }
}

impl Pretty for PrintStmt {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(x) = self;
        writeln!(f, "print {};", x.display())
    }
}

impl Pretty for ExprStmt {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(x) = self;
        writeln!(f, "{};", x.display())
    }
}

impl Pretty for Stmt {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expr(x) => x.pretty(f),
            Self::Print(x) => x.pretty(f),
            Self::VarDecl(x) => x.pretty(f),
        }
    }
}

impl Pretty for Prog {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(xs) = self;
        for x in xs {
            x.pretty(f)?
        }
        Ok(())
    }
}

// tests

#[cfg(test)]
mod tests;
