use nutype::nutype;

mod pretty;
#[cfg(test)]
use pretty::Pretty;

#[derive(Debug, Clone, Copy)]
pub struct NilLit;

#[derive(Debug, Clone, Copy)]
pub struct BoolLit(pub bool);

#[derive(Debug, Clone, Copy)]
pub struct NumLit(pub f64);

#[derive(Debug, Clone)]
pub struct StrLit(pub String);

#[derive(Debug, Clone)]
pub enum Lit {
    Nil(NilLit),
    Bool(BoolLit),
    Num(NumLit),
    Str(StrLit),
}

#[nutype(derive(Debug, Deref, Borrow, FromStr, Clone))]
pub struct VarName(String);

#[derive(Debug, Clone, Copy)]
pub enum UnOp {
    /// Negation (`-`)
    Neg,
    /// Logical not (`!`)
    Not,
}

#[derive(Debug, Clone, Copy)]
pub enum EqOp {
    /// Equality (`==`)
    Eq,
    /// Inequality (`!=`)
    Ne,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum RelOp {
    Eq(EqOp),
    Cmp(CmpOp),
}

#[derive(Debug, Clone, Copy)]
pub enum AddOp {
    /// Addition (`+`)
    Add,
    /// Substraction (`-`)
    Sub,
}

#[derive(Debug, Clone, Copy)]
pub enum MulOp {
    /// Multiplication (`*`)
    Mul,
    /// Division (`-`)
    Div,
}

#[derive(Debug, Clone, Copy)]
pub enum LogOp {
    /// Logical "or" aka disjunction
    Or,
    /// Logical "and" aka conjunction
    And,
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Rel(RelOp),
    Add(AddOp),
    Mul(MulOp),
    Log(LogOp),
}

#[derive(Debug, Clone)]
pub struct UnExpr {
    pub op: UnOp,
    pub e: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct BinExpr {
    pub op: BinOp,
    pub l: Box<Expr>,
    pub r: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: VarName,
    pub val: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Lit(Lit),
    Un(UnExpr),
    Bin(BinExpr),
    Var(VarName),
    Assign(Assign),
    Call(Call),
    Fun(Fun),
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: VarName,
    pub init: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct Fun {
    pub params: Vec<VarName>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct FunDecl {
    pub name: VarName,
    pub fun: Fun,
}

#[derive(Debug, Clone)]
pub struct PrintStmt(pub Expr);

#[derive(Debug, Clone)]
pub struct ExprStmt(pub Expr);

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(ExprStmt),
    Print(PrintStmt),
    VarDecl(VarDecl),
    Block(Box<Block>),
    If(Box<IfStmt>),
    While(Box<WhileStmt>),
    Break,
    FunDecl(FunDecl),
    Ret(RetStmt),
}

#[derive(Debug, Clone)]
pub struct RetStmt(pub Option<Expr>);

#[derive(Debug, Clone)]
pub struct Block(pub Vec<Stmt>);

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub cond: Expr,
    pub then: Stmt,
    pub else_: Option<Stmt>,
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub cond: Expr,
    pub body: Stmt,
}

#[derive(Debug, Clone)]
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

#[cfg(test)]
mod tests;
