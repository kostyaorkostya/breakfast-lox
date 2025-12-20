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
        "#]].assert_debug_eq(&actual);
        Ok(())
    }
}
