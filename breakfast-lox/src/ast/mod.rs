pub enum BinOp {
    /// Equality (`==`)
    Eq,
    /// Inequality (`!=`)
    Ne,
    /// Less than (`<`)
    Lt,
    /// Less than or equal to (`<=`)
    Le,
    /// Greater than (`>`)
    Gt,
    /// Greater than or equal to (`>=`)
    Ge,
    /// Addition (`+`)
    Add,
    /// Substraction (`-`)
    Sub,
    /// Multiplication (`*`)
    Mul,
    /// Division (`-`)
    Div,
}

pub enum UnaryOp {
    /// Negation (`-`)
    Neg,
    /// Logical not (`!`)
    Not,
}

struct Number(f64);

struct Str(String);
