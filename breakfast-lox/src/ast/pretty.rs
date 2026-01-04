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

impl<T: Pretty> Pretty for super::Node<T> {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.pretty(f)
    }
}

impl Pretty for super::NilLit {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nil")
    }
}

impl Pretty for super::BoolLit {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(x) = self;
        write!(f, "{}", x)
    }
}

impl Pretty for super::NumLit {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(x) = self;
        // TODO(kostya): What precision and format should it use?
        write!(f, "{}", x)
    }
}

impl Pretty for super::StrLit {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(x) = self;
        // debug formatting should output escaped string
        write!(f, "{:?}", x)
    }
}

impl Pretty for super::Lit {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil(x) => x.pretty(f),
            Self::Bool(x) => x.pretty(f),
            Self::Num(x) => x.pretty(f),
            Self::Str(x) => x.pretty(f),
        }
    }
}

impl Pretty for super::VarName {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", **self)
    }
}

impl Pretty for super::UnOp {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Neg => write!(f, "-"),
            Self::Not => write!(f, "!"),
        }
    }
}

impl Pretty for super::EqOp {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
        }
    }
}

impl Pretty for super::CmpOp {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lt => write!(f, "<"),
            Self::Le => write!(f, "<="),
            Self::Gt => write!(f, ">"),
            Self::Ge => write!(f, ">="),
        }
    }
}

impl Pretty for super::RelOp {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eq(x) => x.pretty(f),
            Self::Cmp(x) => x.pretty(f),
        }
    }
}

impl Pretty for super::AddOp {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
        }
    }
}

impl Pretty for super::MulOp {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
}

impl Pretty for super::LogOp {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Or => write!(f, "or"),
            Self::And => write!(f, "and"),
        }
    }
}

impl Pretty for super::BinOp {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rel(x) => x.pretty(f),
            Self::Add(x) => x.pretty(f),
            Self::Mul(x) => x.pretty(f),
            Self::Log(x) => x.pretty(f),
        }
    }
}

impl Pretty for super::UnExpr {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { op, e } = self;
        write!(f, "{}({})", op.display(), e.display())
    }
}

impl Pretty for super::BinExpr {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { op, l, r } = self;
        write!(f, "({} {} {})", l.display(), op.display(), r.display())
    }
}

impl Pretty for super::Call {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { callee, args } = self;
        callee.pretty(f)?;
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?
            };
            arg.pretty(f)?;
        }
        Ok(())
    }
}

impl Pretty for super::Expr {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lit(x) => x.pretty(f),
            Self::Un(x) => x.pretty(f),
            Self::Bin(x) => x.pretty(f),
            Self::Var(x) => x.pretty(f),
            Self::Assign(x) => x.pretty(f),
            Self::Call(x) => x.pretty(f),
            Self::Fun(x) => write!(f, "fun {}", x.display()),
        }
    }
}

impl Pretty for super::VarDecl {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { name, init } = self;
        match init {
            Some(init) => writeln!(f, "var {} = {};", name.display(), init.display()),
            None => writeln!(f, "var {};", name.display()),
        }
    }
}

impl Pretty for super::Assign {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { name, val } = self;
        writeln!(f, "{} = {};", name.display(), val.display())
    }
}

impl Pretty for super::PrintStmt {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(x) = self;
        writeln!(f, "print {};", x.display())
    }
}

impl Pretty for super::ExprStmt {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(x) = self;
        writeln!(f, "{};", x.display())
    }
}

impl Pretty for super::Fun {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { params, body } = self;
        write!(f, "(")?;
        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?
            };
            param.pretty(f)?;
        }
        writeln!(f, ")")?;
        body.pretty(f)
    }
}

impl Pretty for super::FunDecl {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { name, fun } = self;
        write!(f, "fun {}{}", name.display(), fun.display())
    }
}

impl Pretty for super::RetStmt {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(val) = self;
        match val {
            Some(x) => writeln!(f, "return {};", x.display()),
            None => writeln!(f, "return;"),
        }
    }
}

impl Pretty for super::Stmt {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expr(x) => x.pretty(f),
            Self::Print(x) => x.pretty(f),
            Self::VarDecl(x) => x.pretty(f),
            Self::Block(x) => x.pretty(f),
            Self::If(x) => x.pretty(f),
            Self::While(x) => x.pretty(f),
            Self::Break(_) => writeln!(f, "break;"),
            Self::FunDecl(x) => x.pretty(f),
            Self::Ret(x) => x.pretty(f),
        }
    }
}

impl Pretty for super::Block {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(xs) = self;
        for x in xs {
            writeln!(f, "{{")?;
            x.pretty(f)?;
            writeln!(f, "}}")?
        }
        Ok(())
    }
}

impl Pretty for super::IfStmt {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { cond, then, else_ } = self;
        writeln!(f, "if {} {}", cond.display(), then.display())?;
        if let Some(else_) = else_ {
            writeln!(f, "else {}", else_.display())?;
        }
        Ok(())
    }
}

impl Pretty for super::WhileStmt {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { cond, body } = self;
        writeln!(f, "while ({}) {}", cond.display(), body.display())
    }
}

impl Pretty for super::Prog {
    fn pretty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(xs) = self;
        for x in xs {
            x.pretty(f)?
        }
        Ok(())
    }
}
