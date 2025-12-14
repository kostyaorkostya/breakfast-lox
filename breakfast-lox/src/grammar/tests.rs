mod bool_literals {
    use super::super::grammar::ExprParser;
    use expect_test::expect;

    #[test]
    fn test_false() -> anyhow::Result<()> {
        let actual = ExprParser::new().parse("false")?;
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
        let actual = ExprParser::new().parse("false")?;
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
    use super::super::grammar::ExprParser;
    use expect_test::expect;
    #[test]
    fn test_nil() -> anyhow::Result<()> {
        let actual = ExprParser::new().parse("nil")?;
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
    use super::super::grammar::ExprParser;
    use super::super::parse_error;
    use super::super::parse_error::NumLitParseError;
    use expect_test::expect;
    use lalrpop_util::ParseError;

    #[test]
    fn test_0_one() -> anyhow::Result<()> {
        let actual = ExprParser::new().parse("0")?;
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
        let actual = ExprParser::new().parse("0.")?;
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
        let actual = ExprParser::new().parse(".0")?;
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
    fn test_integer_greater_than_representable_by_f64() {
        let err = ExprParser::new().parse("9007199254740993").unwrap_err();
        assert!(matches!(
            err,
            ParseError::User {
                error: parse_error::ParseError::NumLit(NumLitParseError::NumberIsNotFinite { .. })
            }
        ));
    }
}
