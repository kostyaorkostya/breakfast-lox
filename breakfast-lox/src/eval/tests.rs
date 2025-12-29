use super::{Interpreter, Value};
use crate::grammar::ExprParser;

fn parse_and_eval(expr: &str) -> anyhow::Result<Value> {
    let expr = ExprParser::new()
        .parse(expr)
        .map_err(|e| e.map_token(|t| format!("{t:?}")))?;
    Ok(Interpreter.eval(&expr)?)
}

mod bool_literals {
    use crate::eval::tests::parse_and_eval;
    use expect_test::expect;

    #[test]
    fn test_false() -> anyhow::Result<()> {
        let actual = parse_and_eval("false")?;
        expect![[r#"
            Bool(
                false,
            )
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }
}

mod stringify {
    use super::super::{Stringify, Value};
    use expect_test::expect;

    #[test]
    fn test_nil() -> anyhow::Result<()> {
        let actual = Value::Nil.display().to_string();
        expect!["nil"]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_bool() -> anyhow::Result<()> {
        let actual = Value::Bool(true).display().to_string();
        expect!["true"]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_num() -> anyhow::Result<()> {
        let actual = Value::Num(33f64).display().to_string();
        expect!["33"]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_str() -> anyhow::Result<()> {
        let actual = Value::Str("hello".into()).display().to_string();
        expect!["hello"]
        .assert_eq(&actual);
        Ok(())
    }
}
