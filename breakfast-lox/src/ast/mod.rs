pub struct NilLit;

pub struct BoolLit(bool);

pub struct NumLit(f64);

pub struct StrLit(String);

pub enum Lit {
    Nil(NilLit),
    Bool(BoolLit),
    Num(NumLit),
    Str(StrLit),
}

pub enum UnOp {
    /// Negation (`-`)
    Neg,
    /// Logical not (`!`)
    Not,
}

pub enum EqOp {
    /// Equality (`==`)
    Eq,
    /// Inequality (`!=`)
    Ne,
}

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

pub enum RelOp {
    Eq(EqOp),
    Cmp(CmpOp),
}

pub enum AddOp {
    /// Addition (`+`)
    Add,
    /// Substraction (`-`)
    Sub,
}

pub enum MulOp {
    /// Multiplication (`*`)
    Mul,
    /// Division (`-`)
    Div,
}

pub enum BinOp {
    Rel(RelOp),
    Add(AddOp),
    Mul(MulOp),
}

pub struct UnExpr {
    pub op: UnOp,
    pub expr: Box<Expr>,
}

pub struct BinExpr {
    pub op: BinOp,
    pub left: Box<Expr>,
    pub righ: Box<Expr>,
}

pub enum Expr {
    Lit(Lit),
    Un(UnExpr),
    Bin(BinExpr),
}

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
