mod pretty {
    use super::super::{AddOp, BinExpr, BinOp, Expr, Lit, NumLit, Pretty, StrLit};
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
        expect![[r#"("hello" + 3)"#]]
        .assert_eq(&actual);
        Ok(())
    }
}
