use crate::ast;
use crate::grammar;

fn parse_expr(expr: &str) -> Result<ast::Node<ast::Expr>, grammar::CompileError> {
    let mut ids = ast::SeqNodeIdGen::new();
    Ok(grammar::parse_expr(&mut ids, expr)?)
}

fn parse_prog(prog: &str) -> Result<ast::Node<ast::Prog>, grammar::CompileError> {
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
            Node {
                id: NodeId(
                    0,
                ),
                synthetic: false,
                loc: Some(
                    0..5,
                ),
                kind: Lit(
                    Bool(
                        BoolLit(
                            false,
                        ),
                    ),
                ),
            }
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_true() -> anyhow::Result<()> {
        let actual = parse_expr("false")?;
        expect![[r#"
            Node {
                id: NodeId(
                    0,
                ),
                synthetic: false,
                loc: Some(
                    0..5,
                ),
                kind: Lit(
                    Bool(
                        BoolLit(
                            false,
                        ),
                    ),
                ),
            }
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
            Node {
                id: NodeId(
                    0,
                ),
                synthetic: false,
                loc: Some(
                    0..3,
                ),
                kind: Lit(
                    Nil(
                        NilLit,
                    ),
                ),
            }
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }
}

mod num_literal {
    use super::super::parse_error::NumLitParseError;
    use super::super::{CompileError, ParseError};
    use super::parse_expr;
    use expect_test::expect;

    #[test]
    fn test_0_one() -> anyhow::Result<()> {
        let actual = parse_expr("0")?;
        expect![[r#"
            Node {
                id: NodeId(
                    0,
                ),
                synthetic: false,
                loc: Some(
                    0..1,
                ),
                kind: Lit(
                    Num(
                        NumLit(
                            0.0,
                        ),
                    ),
                ),
            }
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_0_two() -> anyhow::Result<()> {
        let actual = parse_expr("0.")?;
        expect![[r#"
            Node {
                id: NodeId(
                    0,
                ),
                synthetic: false,
                loc: Some(
                    0..2,
                ),
                kind: Lit(
                    Num(
                        NumLit(
                            0.0,
                        ),
                    ),
                ),
            }
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_0_three() -> anyhow::Result<()> {
        let actual = parse_expr(".0")?;
        expect![[r#"
            Node {
                id: NodeId(
                    0,
                ),
                synthetic: false,
                loc: Some(
                    0..2,
                ),
                kind: Lit(
                    Num(
                        NumLit(
                            0.0,
                        ),
                    ),
                ),
            }
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
    use super::parse_prog;
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
            Node {
                id: NodeId(
                    11,
                ),
                synthetic: false,
                loc: Some(
                    13..89,
                ),
                kind: Prog(
                    [
                        Node {
                            id: NodeId(
                                2,
                            ),
                            synthetic: false,
                            loc: Some(
                                13..29,
                            ),
                            kind: VarDecl(
                                VarDecl {
                                    name: Node {
                                        id: NodeId(
                                            0,
                                        ),
                                        synthetic: false,
                                        loc: Some(
                                            17..20,
                                        ),
                                        kind: VarName(
                                            "foo",
                                        ),
                                    },
                                    init: Some(
                                        Node {
                                            id: NodeId(
                                                1,
                                            ),
                                            synthetic: false,
                                            loc: Some(
                                                23..28,
                                            ),
                                            kind: Lit(
                                                Str(
                                                    StrLit(
                                                        "foo",
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        Node {
                            id: NodeId(
                                5,
                            ),
                            synthetic: false,
                            loc: Some(
                                42..58,
                            ),
                            kind: VarDecl(
                                VarDecl {
                                    name: Node {
                                        id: NodeId(
                                            3,
                                        ),
                                        synthetic: false,
                                        loc: Some(
                                            46..49,
                                        ),
                                        kind: VarName(
                                            "bar",
                                        ),
                                    },
                                    init: Some(
                                        Node {
                                            id: NodeId(
                                                4,
                                            ),
                                            synthetic: false,
                                            loc: Some(
                                                52..57,
                                            ),
                                            kind: Lit(
                                                Str(
                                                    StrLit(
                                                        "bar",
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        Node {
                            id: NodeId(
                                10,
                            ),
                            synthetic: false,
                            loc: Some(
                                71..89,
                            ),
                            kind: Print(
                                PrintStmt(
                                    Node {
                                        id: NodeId(
                                            9,
                                        ),
                                        synthetic: false,
                                        loc: Some(
                                            78..87,
                                        ),
                                        kind: Bin(
                                            BinExpr {
                                                op: Node {
                                                    id: NodeId(
                                                        7,
                                                    ),
                                                    synthetic: false,
                                                    loc: Some(
                                                        82..83,
                                                    ),
                                                    kind: Add(
                                                        Add,
                                                    ),
                                                },
                                                l: Node {
                                                    id: NodeId(
                                                        6,
                                                    ),
                                                    synthetic: false,
                                                    loc: Some(
                                                        78..81,
                                                    ),
                                                    kind: Var(
                                                        VarName(
                                                            "foo",
                                                        ),
                                                    ),
                                                },
                                                r: Node {
                                                    id: NodeId(
                                                        8,
                                                    ),
                                                    synthetic: false,
                                                    loc: Some(
                                                        84..87,
                                                    ),
                                                    kind: Var(
                                                        VarName(
                                                            "bar",
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                ),
            }
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
            Node {
                id: NodeId(
                    38,
                ),
                synthetic: false,
                loc: Some(
                    13..464,
                ),
                kind: Prog(
                    [
                        Node {
                            id: NodeId(
                                2,
                            ),
                            synthetic: false,
                            loc: Some(
                                13..32,
                            ),
                            kind: VarDecl(
                                VarDecl {
                                    name: Node {
                                        id: NodeId(
                                            0,
                                        ),
                                        synthetic: false,
                                        loc: Some(
                                            17..18,
                                        ),
                                        kind: VarName(
                                            "a",
                                        ),
                                    },
                                    init: Some(
                                        Node {
                                            id: NodeId(
                                                1,
                                            ),
                                            synthetic: false,
                                            loc: Some(
                                                21..31,
                                            ),
                                            kind: Lit(
                                                Str(
                                                    StrLit(
                                                        "global a",
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        Node {
                            id: NodeId(
                                5,
                            ),
                            synthetic: false,
                            loc: Some(
                                45..64,
                            ),
                            kind: VarDecl(
                                VarDecl {
                                    name: Node {
                                        id: NodeId(
                                            3,
                                        ),
                                        synthetic: false,
                                        loc: Some(
                                            49..50,
                                        ),
                                        kind: VarName(
                                            "b",
                                        ),
                                    },
                                    init: Some(
                                        Node {
                                            id: NodeId(
                                                4,
                                            ),
                                            synthetic: false,
                                            loc: Some(
                                                53..63,
                                            ),
                                            kind: Lit(
                                                Str(
                                                    StrLit(
                                                        "global b",
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        Node {
                            id: NodeId(
                                8,
                            ),
                            synthetic: false,
                            loc: Some(
                                77..96,
                            ),
                            kind: VarDecl(
                                VarDecl {
                                    name: Node {
                                        id: NodeId(
                                            6,
                                        ),
                                        synthetic: false,
                                        loc: Some(
                                            81..82,
                                        ),
                                        kind: VarName(
                                            "c",
                                        ),
                                    },
                                    init: Some(
                                        Node {
                                            id: NodeId(
                                                7,
                                            ),
                                            synthetic: false,
                                            loc: Some(
                                                85..95,
                                            ),
                                            kind: Lit(
                                                Str(
                                                    StrLit(
                                                        "global c",
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        Node {
                            id: NodeId(
                                31,
                            ),
                            synthetic: false,
                            loc: Some(
                                109..401,
                            ),
                            kind: Block(
                                Block(
                                    [
                                        Node {
                                            id: NodeId(
                                                11,
                                            ),
                                            synthetic: false,
                                            loc: Some(
                                                125..143,
                                            ),
                                            kind: VarDecl(
                                                VarDecl {
                                                    name: Node {
                                                        id: NodeId(
                                                            9,
                                                        ),
                                                        synthetic: false,
                                                        loc: Some(
                                                            129..130,
                                                        ),
                                                        kind: VarName(
                                                            "a",
                                                        ),
                                                    },
                                                    init: Some(
                                                        Node {
                                                            id: NodeId(
                                                                10,
                                                            ),
                                                            synthetic: false,
                                                            loc: Some(
                                                                133..142,
                                                            ),
                                                            kind: Lit(
                                                                Str(
                                                                    StrLit(
                                                                        "outer a",
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        },
                                        Node {
                                            id: NodeId(
                                                14,
                                            ),
                                            synthetic: false,
                                            loc: Some(
                                                158..176,
                                            ),
                                            kind: VarDecl(
                                                VarDecl {
                                                    name: Node {
                                                        id: NodeId(
                                                            12,
                                                        ),
                                                        synthetic: false,
                                                        loc: Some(
                                                            162..163,
                                                        ),
                                                        kind: VarName(
                                                            "b",
                                                        ),
                                                    },
                                                    init: Some(
                                                        Node {
                                                            id: NodeId(
                                                                13,
                                                            ),
                                                            synthetic: false,
                                                            loc: Some(
                                                                166..175,
                                                            ),
                                                            kind: Lit(
                                                                Str(
                                                                    StrLit(
                                                                        "outer b",
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        },
                                        Node {
                                            id: NodeId(
                                                24,
                                            ),
                                            synthetic: false,
                                            loc: Some(
                                                191..318,
                                            ),
                                            kind: Block(
                                                Block(
                                                    [
                                                        Node {
                                                            id: NodeId(
                                                                17,
                                                            ),
                                                            synthetic: false,
                                                            loc: Some(
                                                                209..227,
                                                            ),
                                                            kind: VarDecl(
                                                                VarDecl {
                                                                    name: Node {
                                                                        id: NodeId(
                                                                            15,
                                                                        ),
                                                                        synthetic: false,
                                                                        loc: Some(
                                                                            213..214,
                                                                        ),
                                                                        kind: VarName(
                                                                            "a",
                                                                        ),
                                                                    },
                                                                    init: Some(
                                                                        Node {
                                                                            id: NodeId(
                                                                                16,
                                                                            ),
                                                                            synthetic: false,
                                                                            loc: Some(
                                                                                217..226,
                                                                            ),
                                                                            kind: Lit(
                                                                                Str(
                                                                                    StrLit(
                                                                                        "inner a",
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        Node {
                                                            id: NodeId(
                                                                19,
                                                            ),
                                                            synthetic: false,
                                                            loc: Some(
                                                                244..252,
                                                            ),
                                                            kind: Print(
                                                                PrintStmt(
                                                                    Node {
                                                                        id: NodeId(
                                                                            18,
                                                                        ),
                                                                        synthetic: false,
                                                                        loc: Some(
                                                                            250..251,
                                                                        ),
                                                                        kind: Var(
                                                                            VarName(
                                                                                "a",
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        Node {
                                                            id: NodeId(
                                                                21,
                                                            ),
                                                            synthetic: false,
                                                            loc: Some(
                                                                269..277,
                                                            ),
                                                            kind: Print(
                                                                PrintStmt(
                                                                    Node {
                                                                        id: NodeId(
                                                                            20,
                                                                        ),
                                                                        synthetic: false,
                                                                        loc: Some(
                                                                            275..276,
                                                                        ),
                                                                        kind: Var(
                                                                            VarName(
                                                                                "b",
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        Node {
                                                            id: NodeId(
                                                                23,
                                                            ),
                                                            synthetic: false,
                                                            loc: Some(
                                                                294..302,
                                                            ),
                                                            kind: Print(
                                                                PrintStmt(
                                                                    Node {
                                                                        id: NodeId(
                                                                            22,
                                                                        ),
                                                                        synthetic: false,
                                                                        loc: Some(
                                                                            300..301,
                                                                        ),
                                                                        kind: Var(
                                                                            VarName(
                                                                                "c",
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                ),
                                            ),
                                        },
                                        Node {
                                            id: NodeId(
                                                26,
                                            ),
                                            synthetic: false,
                                            loc: Some(
                                                333..341,
                                            ),
                                            kind: Print(
                                                PrintStmt(
                                                    Node {
                                                        id: NodeId(
                                                            25,
                                                        ),
                                                        synthetic: false,
                                                        loc: Some(
                                                            339..340,
                                                        ),
                                                        kind: Var(
                                                            VarName(
                                                                "a",
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        },
                                        Node {
                                            id: NodeId(
                                                28,
                                            ),
                                            synthetic: false,
                                            loc: Some(
                                                356..364,
                                            ),
                                            kind: Print(
                                                PrintStmt(
                                                    Node {
                                                        id: NodeId(
                                                            27,
                                                        ),
                                                        synthetic: false,
                                                        loc: Some(
                                                            362..363,
                                                        ),
                                                        kind: Var(
                                                            VarName(
                                                                "b",
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        },
                                        Node {
                                            id: NodeId(
                                                30,
                                            ),
                                            synthetic: false,
                                            loc: Some(
                                                379..387,
                                            ),
                                            kind: Print(
                                                PrintStmt(
                                                    Node {
                                                        id: NodeId(
                                                            29,
                                                        ),
                                                        synthetic: false,
                                                        loc: Some(
                                                            385..386,
                                                        ),
                                                        kind: Var(
                                                            VarName(
                                                                "c",
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                ),
                            ),
                        },
                        Node {
                            id: NodeId(
                                33,
                            ),
                            synthetic: false,
                            loc: Some(
                                414..422,
                            ),
                            kind: Print(
                                PrintStmt(
                                    Node {
                                        id: NodeId(
                                            32,
                                        ),
                                        synthetic: false,
                                        loc: Some(
                                            420..421,
                                        ),
                                        kind: Var(
                                            VarName(
                                                "a",
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        Node {
                            id: NodeId(
                                35,
                            ),
                            synthetic: false,
                            loc: Some(
                                435..443,
                            ),
                            kind: Print(
                                PrintStmt(
                                    Node {
                                        id: NodeId(
                                            34,
                                        ),
                                        synthetic: false,
                                        loc: Some(
                                            441..442,
                                        ),
                                        kind: Var(
                                            VarName(
                                                "b",
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                        Node {
                            id: NodeId(
                                37,
                            ),
                            synthetic: false,
                            loc: Some(
                                456..464,
                            ),
                            kind: Print(
                                PrintStmt(
                                    Node {
                                        id: NodeId(
                                            36,
                                        ),
                                        synthetic: false,
                                        loc: Some(
                                            462..463,
                                        ),
                                        kind: Var(
                                            VarName(
                                                "c",
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ],
                ),
            }
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }
}

mod syntax {
    use super::super::{CompileError, SyntaxError};
    use super::parse_prog;
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
            Node {
                id: NodeId(
                    4,
                ),
                synthetic: false,
                loc: Some(
                    13..62,
                ),
                kind: Prog(
                    [
                        Node {
                            id: NodeId(
                                3,
                            ),
                            synthetic: false,
                            loc: Some(
                                13..62,
                            ),
                            kind: While(
                                WhileStmt {
                                    cond: Node {
                                        id: NodeId(
                                            0,
                                        ),
                                        synthetic: false,
                                        loc: Some(
                                            20..24,
                                        ),
                                        kind: Lit(
                                            Bool(
                                                BoolLit(
                                                    true,
                                                ),
                                            ),
                                        ),
                                    },
                                    body: Node {
                                        id: NodeId(
                                            2,
                                        ),
                                        synthetic: false,
                                        loc: Some(
                                            26..62,
                                        ),
                                        kind: Block(
                                            Block(
                                                [
                                                    Node {
                                                        id: NodeId(
                                                            1,
                                                        ),
                                                        synthetic: false,
                                                        loc: Some(
                                                            42..48,
                                                        ),
                                                        kind: Break,
                                                    },
                                                ],
                                            ),
                                        ),
                                    },
                                },
                            ),
                        },
                    ],
                ),
            }
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
            Node {
                id: NodeId(
                    5,
                ),
                synthetic: false,
                loc: Some(
                    13..96,
                ),
                kind: Prog(
                    [
                        Node {
                            id: NodeId(
                                4,
                            ),
                            synthetic: false,
                            loc: Some(
                                13..96,
                            ),
                            kind: While(
                                WhileStmt {
                                    cond: Node {
                                        id: NodeId(
                                            0,
                                        ),
                                        synthetic: false,
                                        loc: Some(
                                            20..24,
                                        ),
                                        kind: Lit(
                                            Bool(
                                                BoolLit(
                                                    true,
                                                ),
                                            ),
                                        ),
                                    },
                                    body: Node {
                                        id: NodeId(
                                            3,
                                        ),
                                        synthetic: false,
                                        loc: Some(
                                            26..96,
                                        ),
                                        kind: Block(
                                            Block(
                                                [
                                                    Node {
                                                        id: NodeId(
                                                            2,
                                                        ),
                                                        synthetic: false,
                                                        loc: Some(
                                                            42..82,
                                                        ),
                                                        kind: Block(
                                                            Block(
                                                                [
                                                                    Node {
                                                                        id: NodeId(
                                                                            1,
                                                                        ),
                                                                        synthetic: false,
                                                                        loc: Some(
                                                                            60..66,
                                                                        ),
                                                                        kind: Break,
                                                                    },
                                                                ],
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            ),
                                        ),
                                    },
                                },
                            ),
                        },
                    ],
                ),
            }
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
            Node {
                id: NodeId(
                    5,
                ),
                synthetic: false,
                loc: Some(
                    0..24,
                ),
                kind: Prog(
                    [
                        Node {
                            id: NodeId(
                                4,
                            ),
                            synthetic: false,
                            loc: Some(
                                0..24,
                            ),
                            kind: FunDecl(
                                FunDecl {
                                    name: Node {
                                        id: NodeId(
                                            0,
                                        ),
                                        synthetic: false,
                                        loc: Some(
                                            4..7,
                                        ),
                                        kind: VarName(
                                            "foo",
                                        ),
                                    },
                                    fun: Fun {
                                        params: [],
                                        body: Node {
                                            id: NodeId(
                                                3,
                                            ),
                                            synthetic: false,
                                            loc: Some(
                                                10..24,
                                            ),
                                            kind: Block(
                                                [
                                                    Node {
                                                        id: NodeId(
                                                            2,
                                                        ),
                                                        synthetic: false,
                                                        loc: Some(
                                                            12..22,
                                                        ),
                                                        kind: Ret(
                                                            RetStmt(
                                                                Some(
                                                                    Node {
                                                                        id: NodeId(
                                                                            1,
                                                                        ),
                                                                        synthetic: false,
                                                                        loc: Some(
                                                                            19..21,
                                                                        ),
                                                                        kind: Lit(
                                                                            Num(
                                                                                NumLit(
                                                                                    33.0,
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            ),
                                        },
                                    },
                                },
                            ),
                        },
                    ],
                ),
            }
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }
}
