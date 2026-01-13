mod pretty {
    use crate::ast;
    use crate::ast::Pretty;
    use crate::grammar;
    use expect_test::expect;

    fn parse_expr(expr: &str) -> anyhow::Result<ast::Node<ast::Expr>> {
        let mut ids = ast::SeqNodeIdGen::new();
        Ok(grammar::parse_expr(&mut ids, expr)?)
    }

    fn parse_prog(prog: &str) -> anyhow::Result<ast::Node<ast::Prog>> {
        let mut ids = ast::SeqNodeIdGen::new();
        Ok(grammar::parse_prog(&mut ids, prog)?)
    }

    #[test]
    fn test_expression() -> anyhow::Result<()> {
        let actual = parse_expr(r#"("hello" + 3)"#)?.display().to_string();
        expect![[r#"("hello" + 3)"#]].assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_program() -> anyhow::Result<()> {
        let actual = parse_expr(r#"print "hello";)"#)?.display().to_string();
        expect![[r#"
            print "hello";
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_var_decl() -> anyhow::Result<()> {
        let actual = parse_expr(
            r#"
            var foo = "bar";
            print foo;
        "#,
        )?
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
