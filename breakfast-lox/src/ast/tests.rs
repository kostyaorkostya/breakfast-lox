mod pretty {
    use super::super::{
        AddOp, BinExpr, BinOp, Expr, Lit, NodeIdGen, NumLit, Pretty, PrintStmt, Prog, SeqNodeIdGen,
        Stmt, StrLit, VarDecl, VarName,
    };
    use expect_test::expect;

    #[test]
    fn test_expression() -> anyhow::Result<()> {
        let mut ids = SeqNodeIdGen::new();
        let actual = Expr::Bin(ids.new_synth_node(BinExpr {
            op: ids.new_synth_node(BinOp::Add(AddOp::Add)),
            l: Box::new(ids.new_synth_node(Expr::Lit(
                ids.new_synth_node(Lit::Str(StrLit("hello".into()))),
            ))),
            r: Box::new(ids.new_synth_node(Expr::Lit(ids.new_synth_node(Lit::Num(NumLit(3f64)))))),
        }))
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

    #[test]
    fn test_var_decl() -> anyhow::Result<()> {
        let actual = Prog(vec![
            Stmt::VarDecl(VarDecl {
                name: VarName::new("foo"),
                init: Some(Expr::Lit(Lit::Str(StrLit("bar".into())))),
            }),
            Stmt::Print(PrintStmt(Expr::Var(VarName::new("foo")))),
        ])
        .display()
        .to_string();
        expect![[r#"
            var foo = "bar";
            print foo;
        "#]]
        .assert_eq(&actual);
        Ok(())
    }
}
