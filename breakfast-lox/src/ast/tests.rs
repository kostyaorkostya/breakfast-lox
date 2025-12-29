mod pretty {
    use super::super::{
        AddOp, BinExpr, BinOp, Expr, Lit, NumLit, Pretty, PrintStmt, Prog, Stmt, StrLit,
    };
    use expect_test::expect;

    #[test]
    fn test_expression() -> anyhow::Result<()> {
        let actual = Expr::Bin(BinExpr {
            op: BinOp::Add(AddOp::Add),
            l: Box::new(Expr::Lit(Lit::Str(StrLit("hello".into())))),
            r: Box::new(Expr::Lit(Lit::Num(NumLit(3f64)))),
        })
        .display()
        .to_string();
        expect![[r#"("hello" + 3)"#]].assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_program() -> anyhow::Result<()> {
        let actual = Prog(vec![Stmt::Print(PrintStmt(Expr::Lit(Lit::Str(StrLit(
            "hello".into(),
        )))))])
        .display()
        .to_string();
        expect![[r#"
            print "hello";
        "#]]
        .assert_eq(&actual);
        Ok(())
    }
}
