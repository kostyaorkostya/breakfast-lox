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
