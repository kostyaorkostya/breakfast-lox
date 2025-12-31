use super::{Interpreter, OutOfFuelError, RuntimeError, Val};
use crate::grammar::{ExprParser, ProgParser};
use std::cell::RefCell;
use std::rc::Rc;

const DEFAULT_FUEL: u64 = 1000u64;
fn resolve_fuel(fuel: Option<u64>) -> u64 {
    if let Some(fuel) = fuel {
        fuel
    } else {
        DEFAULT_FUEL
    }
}

fn parse_and_eval_expr(expr: &str, fuel: Option<u64>) -> anyhow::Result<Val> {
    let expr = ExprParser::new()
        .parse(expr)
        .map_err(|e| e.map_token(|t| format!("{t:?}")))?;
    Ok(Interpreter::new_for_test(None, resolve_fuel(fuel)).eval_expr(&expr)?)
}

fn parse_and_eval_prog(prog: &str, fuel: Option<u64>) -> anyhow::Result<String> {
    let prog = ProgParser::new()
        .parse(prog)
        .map_err(|e| e.map_token(|t| format!("{t:?}")))?;
    let buf = Rc::new(RefCell::new(Vec::new()));
    Interpreter::new_for_test(Some(Rc::clone(&buf)), resolve_fuel(fuel)).eval_prog(&prog)?;
    Ok(String::from_utf8(buf.borrow().clone())?)
}

fn parse_and_eval_divergent_prog(prog: &str, fuel: u64) -> anyhow::Result<String> {
    let prog = ProgParser::new()
        .parse(prog)
        .map_err(|e| e.map_token(|t| format!("{t:?}")))?;
    let buf = Rc::new(RefCell::new(Vec::new()));
    match Interpreter::new_for_test(Some(Rc::clone(&buf)), fuel).eval_prog(&prog) {
        Err(RuntimeError::Fuel(OutOfFuelError)) => Ok(()),
        x @ (Err(_) | Ok(_)) => Err(anyhow::anyhow!("expected running out of fuel, got {x:?}")),
    }?;
    Ok(String::from_utf8(buf.borrow().clone())?)
}

mod bool_literals {
    use super::parse_and_eval_expr;
    use expect_test::expect;

    #[test]
    fn test_false() -> anyhow::Result<()> {
        let actual = parse_and_eval_expr("false", None)?;
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
    use super::{parse_and_eval_divergent_prog, parse_and_eval_prog};
    use expect_test::expect;

    #[test]
    fn test_empty() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(r#""#, None)?;
        expect![""].assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_hello_world() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            print "Hello, world!";
        "#,
            None,
        )?;
        expect![[r#"
            Hello, world!
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_print_with_vars() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            var prefix = "Hello, ";
            var suffix = "world!";
            print (prefix + suffix);
        "#,
            None,
        )?;
        expect![[r#"
            Hello, world!
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_referencing_unknown_variable() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            unknown_variable;
        "#,
            None,
        )
        .unwrap_err();
        expect![[r#"
            UndefinedVariable(
                Undefined(
                    "unknown_variable",
                ),
            )
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_assigning_unknown_variable() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            unknown_variable = "bad";
        "#,
            None,
        )
        .unwrap_err();
        expect![[r#"
            UndefinedVariable(
                AssignToUndefined(
                    "unknown_variable",
                ),
            )
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_global_variable_assignment() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            var msg = "Initial value";
            msg = "Hello world!";
            print msg;
        "#,
            None,
        )?;
        expect![[r#"
            Hello world!
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_global_variable_reassignment() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            var msg = "Initial value";
            msg = "reassigning";
            msg = "Hello world!";
            print msg;
        "#,
            None,
        )?;
        expect![[r#"
            Hello world!
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_assignment_returns_rvalue() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            var msg = "Initial value";
            print (msg = "Hello, world!");
            print msg;
        "#,
            None,
        )?;
        expect![[r#"
            Hello, world!
            Hello, world!
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_uninitialized_variable_access() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            var x;
            x;
        "#,
            None,
        )
        .unwrap_err();
        expect![[r#"
            UndefinedVariable(
                AccessUninitialized(
                    "x",
                ),
            )
        "#]]
        .assert_debug_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_block_eval() -> anyhow::Result<()> {
        // example from https://craftinginterpreters.com/statements-and-state.html#block-syntax-and-semantics
        let actual = parse_and_eval_prog(
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
            None,
        )?;
        expect![[r#"
            inner a
            outer b
            global c
            outer a
            outer b
            global c
            global a
            global b
            global c
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_referencing_var_with_the_same_name_in_init() -> anyhow::Result<()> {
        // Challenge 3 https://craftinginterpreters.com/statements-and-state.html#challenges
        let actual = parse_and_eval_prog(
            r#"
            var a = 1;
            {
              var a = a + 2;
              print a;
            }
        "#,
            None,
        )?;
        expect![[r#"
            3
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_if_without_else() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            if (true) print "then";
        "#,
            None,
        )?;
        expect![[r#"
            then
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_if_with_else() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            if (false) print "then"; else print "else";
        "#,
            None,
        )?;
        expect![[r#"
            else
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_if_dangling_else() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            if (false) print "then outer"; if (true) print "then inner"; else print "else";
        "#,
            None,
        )?;
        expect![[r#"
            then inner
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_logical_or() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            print "hi" or 2;
            print nil or "yes";
        "#,
            None,
        )?;
        expect![[r#"
            hi
            yes
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_logical_and() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            print "hi" and 2;
            print nil and "yes";
        "#,
            None,
        )?;
        expect![[r#"
            2
            nil
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_while_loop() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            var i = 2;
            while (i > 0) {
              print i;
              i = i - 1;
            }
        "#,
            None,
        )?;
        expect![[r#"
            2
            1
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_while_in_if_then() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            var i = 2;
            if (true) while (i > 0) {
              print i;
              i = i - 1;
            }
        "#,
            None,
        )?;
        expect![[r#"
            2
            1
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_for_loop() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            for (var i = 0; i < 3; i = i + 1) {
              print i;
            }
        "#,
            None,
        )?;
        expect![[r#"
            0
            1
            2
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_infinite_for_loop() -> anyhow::Result<()> {
        let actual = parse_and_eval_divergent_prog(
            r#"
            for (var i = 0; ; i = i + 1) {
              print i;
            }
        "#,
            18,
        )?;
        expect![[r#"
            0
            1
            2
        "#]]
        .assert_eq(&actual);
        Ok(())
    }
}

mod stringify {
    use super::super::{Stringify, Val};
    use expect_test::expect;

    #[test]
    fn test_nil() -> anyhow::Result<()> {
        let actual = Val::Nil.display().to_string();
        expect!["nil"].assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_bool() -> anyhow::Result<()> {
        let actual = Val::Bool(true).display().to_string();
        expect!["true"].assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_num() -> anyhow::Result<()> {
        let actual = Val::Num(33f64).display().to_string();
        expect!["33"].assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_str() -> anyhow::Result<()> {
        let actual = Val::Str("hello".into()).display().to_string();
        expect!["hello"].assert_eq(&actual);
        Ok(())
    }
}
