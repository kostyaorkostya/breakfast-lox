use super::{Interpreter, Value};
use crate::grammar::{ExprParser, ProgParser};
use std::cell::RefCell;
use std::rc::Rc;

fn parse_and_eval_expr(expr: &str) -> anyhow::Result<Value> {
    let expr = ExprParser::new()
        .parse(expr)
        .map_err(|e| e.map_token(|t| format!("{t:?}")))?;
    Ok(Interpreter::new_for_test(None).eval_expr(&expr)?)
}

fn parse_and_eval_prog(prog: &str) -> anyhow::Result<String> {
    let prog = ProgParser::new()
        .parse(prog)
        .map_err(|e| e.map_token(|t| format!("{t:?}")))?;
    let buf = Rc::new(RefCell::new(Vec::new()));
    Interpreter::new_for_test(Some(Rc::clone(&buf))).eval_prog(&prog)?;
    Ok(String::from_utf8(buf.borrow().clone())?)
}

mod bool_literals {
    use super::parse_and_eval_expr;
    use expect_test::expect;

    #[test]
    fn test_false() -> anyhow::Result<()> {
        let actual = parse_and_eval_expr("false")?;
        expect![[r#"
            Bool(
                false,
            )
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }
}

mod prog {
    use super::parse_and_eval_prog;
    use expect_test::expect;

    #[test]
    fn test_hello_world() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            print "Hello, world!";
        "#,
        )?;
        expect![[r#"
            Hello, world!
        "#]]
        .assert_eq(&actual);
        Ok(())
    }
}

mod stringify {
    use super::super::{Stringify, Value};
    use expect_test::expect;

    #[test]
    fn test_nil() -> anyhow::Result<()> {
        let actual = Value::Nil.display().to_string();
        expect!["nil"].assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_bool() -> anyhow::Result<()> {
        let actual = Value::Bool(true).display().to_string();
        expect!["true"].assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_num() -> anyhow::Result<()> {
        let actual = Value::Num(33f64).display().to_string();
        expect!["33"].assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_str() -> anyhow::Result<()> {
        let actual = Value::Str("hello".into()).display().to_string();
        expect!["hello"].assert_eq(&actual);
        Ok(())
    }
}
