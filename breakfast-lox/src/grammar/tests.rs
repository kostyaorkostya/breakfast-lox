use crate::ast;
use crate::grammar;

fn parse_expr(expr: &str) -> anyhow::Result<ast::Node<ast::Expr>> {
    let mut ids = ast::SeqNodeIdGen::new();
    Ok(grammar::parse_expr(&mut ids, expr)?)
}

fn parse_prog(prog: &str) -> anyhow::Result<ast::Node<ast::Prog>> {
    let mut ids = ast::SeqNodeIdGen::new();
    Ok(grammar::parse_prog(&mut ids, prog)?)
}

mod bool_literals {
    use super::parse_expr;
    use expect_test::expect;

    #[test]
    fn test_false() -> anyhow::Result<()> {
        let actual = parse_expr("false")?;
        expect![[r#"
        Lit(
            Bool(
                BoolLit(
                    false,
                ),
            ),
        )
    "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_true() -> anyhow::Result<()> {
        let actual = parse_expr("false")?;
        expect![[r#"
        Lit(
            Bool(
                BoolLit(
                    false,
                ),
            ),
        )
    "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }
}

mod nil_literal {
    use super::parse_expr;
    use expect_test::expect;
    #[test]
    fn test_nil() -> anyhow::Result<()> {
        let actual = parse_expr("nil")?;
        expect![[r#"
            Lit(
                Nil(
                    NilLit,
                ),
            )
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }
}

mod num_literal {
    use super::super::parse_error::NumLitParseError;
    use super::super::parse_expr;
    use super::super::{CompileError, ParseError};
    use expect_test::expect;

    #[test]
    fn test_0_one() -> anyhow::Result<()> {
        let actual = parse_expr("0")?;
        expect![[r#"
            Lit(
                Num(
                    NumLit(
                        0.0,
                    ),
                ),
            )
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_0_two() -> anyhow::Result<()> {
        let actual = parse_expr("0.")?;
        expect![[r#"
            Lit(
                Num(
                    NumLit(
                        0.0,
                    ),
                ),
            )
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_0_three() -> anyhow::Result<()> {
        let actual = parse_expr(".0")?;
        expect![[r#"
            Lit(
                Num(
                    NumLit(
                        0.0,
                    ),
                ),
            )
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }

    #[test]
    #[ignore = "TODO(kostya): fix it"]
    fn test_integer_greater_than_representable_by_f64() {
        let err = parse_expr("9007199254740993").unwrap_err();
        assert!(matches!(
            err,
            CompileError::Parse(ParseError::NumLit(
                NumLitParseError::NumberIsNotFinite { .. }
            ))
        ));
    }
}

mod prog {
    use super::super::parse_prog;
    use expect_test::expect;

    #[test]
    fn test_basic() -> anyhow::Result<()> {
        let actual = parse_prog(
            r#"
            var foo = "foo";
            var bar = "bar";
            print (foo + bar);
    "#,
        )?;
        expect![[r#"
            Prog(
                [
                    VarDecl(
                        VarDecl {
                            name: VarName(
                                "foo",
                            ),
                            init: Some(
                                Lit(
                                    Str(
                                        StrLit(
                                            "foo",
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                    VarDecl(
                        VarDecl {
                            name: VarName(
                                "bar",
                            ),
                            init: Some(
                                Lit(
                                    Str(
                                        StrLit(
                                            "bar",
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                    Print(
                        PrintStmt(
                            Bin(
                                BinExpr {
                                    op: Add(
                                        Add,
                                    ),
                                    l: Var(
                                        VarName(
                                            "foo",
                                        ),
                                    ),
                                    r: Var(
                                        VarName(
                                            "bar",
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                ],
            )
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_block() -> anyhow::Result<()> {
        let actual = parse_prog(
            r#"
            var a = "global a";
            var b = "global b";
            var c = "global c";
            {
              var a = "outer a";
              var b = "outer b";
              {
                var a = "inner a";
                print a;
                print b;
                print c;
              }
              print a;
              print b;
              print c;
            }
            print a;
            print b;
            print c;
        "#,
        )?;
        expect![[r#"
            Prog(
                [
                    VarDecl(
                        VarDecl {
                            name: VarName(
                                "a",
                            ),
                            init: Some(
                                Lit(
                                    Str(
                                        StrLit(
                                            "global a",
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                    VarDecl(
                        VarDecl {
                            name: VarName(
                                "b",
                            ),
                            init: Some(
                                Lit(
                                    Str(
                                        StrLit(
                                            "global b",
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                    VarDecl(
                        VarDecl {
                            name: VarName(
                                "c",
                            ),
                            init: Some(
                                Lit(
                                    Str(
                                        StrLit(
                                            "global c",
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                    Block(
                        Block(
                            [
                                VarDecl(
                                    VarDecl {
                                        name: VarName(
                                            "a",
                                        ),
                                        init: Some(
                                            Lit(
                                                Str(
                                                    StrLit(
                                                        "outer a",
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                VarDecl(
                                    VarDecl {
                                        name: VarName(
                                            "b",
                                        ),
                                        init: Some(
                                            Lit(
                                                Str(
                                                    StrLit(
                                                        "outer b",
                                                    ),
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                Block(
                                    Block(
                                        [
                                            VarDecl(
                                                VarDecl {
                                                    name: VarName(
                                                        "a",
                                                    ),
                                                    init: Some(
                                                        Lit(
                                                            Str(
                                                                StrLit(
                                                                    "inner a",
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                            Print(
                                                PrintStmt(
                                                    Var(
                                                        VarName(
                                                            "a",
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            Print(
                                                PrintStmt(
                                                    Var(
                                                        VarName(
                                                            "b",
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            Print(
                                                PrintStmt(
                                                    Var(
                                                        VarName(
                                                            "c",
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                ),
                                Print(
                                    PrintStmt(
                                        Var(
                                            VarName(
                                                "a",
                                            ),
                                        ),
                                    ),
                                ),
                                Print(
                                    PrintStmt(
                                        Var(
                                            VarName(
                                                "b",
                                            ),
                                        ),
                                    ),
                                ),
                                Print(
                                    PrintStmt(
                                        Var(
                                            VarName(
                                                "c",
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                        ),
                    ),
                    Print(
                        PrintStmt(
                            Var(
                                VarName(
                                    "a",
                                ),
                            ),
                        ),
                    ),
                    Print(
                        PrintStmt(
                            Var(
                                VarName(
                                    "b",
                                ),
                            ),
                        ),
                    ),
                    Print(
                        PrintStmt(
                            Var(
                                VarName(
                                    "c",
                                ),
                            ),
                        ),
                    ),
                ],
            )
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }
}

mod syntax {
    use super::super::{CompileError, SyntaxError, parse_prog};
    use expect_test::expect;

    #[test]
    fn test_reserved_keywords() -> anyhow::Result<()> {
        // TODO(kostya): parser fails before syntax checking, should I even fix it?
        let err = parse_prog("and;").unwrap_err();
        println!("{err:?}");
        assert!(matches!(err, CompileError::Lalrpop(_)));
        Ok(())
    }

    #[test]
    fn test_global_break() -> anyhow::Result<()> {
        let err = parse_prog("break;").unwrap_err();
        assert!(matches!(
            err,
            CompileError::Syntax(SyntaxError::BreakOutsideLoop)
        ));
        Ok(())
    }

    #[test]
    fn test_break_not_in_loop() -> anyhow::Result<()> {
        let err = parse_prog("if (true) break;").unwrap_err();
        assert!(matches!(
            err,
            CompileError::Syntax(SyntaxError::BreakOutsideLoop)
        ));
        Ok(())
    }

    #[test]
    fn test_break_in_loop() -> anyhow::Result<()> {
        let actual = parse_prog(
            r#"
            while (true) {
              break;
            }
    "#,
        )?;
        expect![[r#"
            Prog(
                [
                    While(
                        WhileStmt {
                            cond: Lit(
                                Bool(
                                    BoolLit(
                                        true,
                                    ),
                                ),
                            ),
                            body: Block(
                                Block(
                                    [
                                        Break,
                                    ],
                                ),
                            ),
                        },
                    ),
                ],
            )
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_break_in_loop_in_block() -> anyhow::Result<()> {
        let actual = parse_prog(
            r#"
            while (true) {
              {
                break;
              }
            }
    "#,
        )?;
        expect![[r#"
            Prog(
                [
                    While(
                        WhileStmt {
                            cond: Lit(
                                Bool(
                                    BoolLit(
                                        true,
                                    ),
                                ),
                            ),
                            body: Block(
                                Block(
                                    [
                                        Block(
                                            Block(
                                                [
                                                    Break,
                                                ],
                                            ),
                                        ),
                                    ],
                                ),
                            ),
                        },
                    ),
                ],
            )
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_return_not_in_a_function() -> anyhow::Result<()> {
        let err = parse_prog("return;").unwrap_err();
        assert!(matches!(
            err,
            CompileError::Syntax(SyntaxError::ReturnOutsideFunction)
        ));
        Ok(())
    }

    #[test]
    fn test_return_in_a_function() -> anyhow::Result<()> {
        let actual = parse_prog("fun foo() { return 33; }")?;
        expect![[r#"
            Prog(
                [
                    FunDecl(
                        FunDecl {
                            name: VarName(
                                "foo",
                            ),
                            fun: Fun {
                                params: [],
                                body: Block(
                                    [
                                        Ret(
                                            RetStmt(
                                                Some(
                                                    Lit(
                                                        Num(
                                                            NumLit(
                                                                33.0,
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ],
                                ),
                            },
                        },
                    ),
                ],
            )
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }
}
