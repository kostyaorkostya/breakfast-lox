use super::{Interpreter, OutOfFuelError, RuntimeError, Val};
use crate::grammar::{parse_expr, parse_prog};
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

fn parse_and_eval_expr(
    expr: &str,
    fuel: Option<u64>,
    clock: Option<Rc<RefCell<f64>>>,
) -> anyhow::Result<Val> {
    let expr = parse_expr(expr)?;
    Ok(Interpreter::new_for_test(None, resolve_fuel(fuel), clock).eval_expr(&expr)?)
}

fn parse_and_eval_prog(
    prog: &str,
    fuel: Option<u64>,
    clock: Option<Rc<RefCell<f64>>>,
) -> anyhow::Result<String> {
    let prog = parse_prog(prog)?;
    let buf = Rc::new(RefCell::new(Vec::new()));
    Interpreter::new_for_test(Some(Rc::clone(&buf)), resolve_fuel(fuel), clock).eval_prog(&prog)?;
    Ok(String::from_utf8(buf.borrow().clone())?)
}

fn parse_and_eval_divergent_prog(
    prog: &str,
    fuel: u64,
    clock: Option<Rc<RefCell<f64>>>,
) -> anyhow::Result<String> {
    let prog = parse_prog(prog)?;
    let buf = Rc::new(RefCell::new(Vec::new()));
    match Interpreter::new_for_test(Some(Rc::clone(&buf)), fuel, clock).eval_prog(&prog) {
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
        let actual = parse_and_eval_expr("false", None, None)?;
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
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_empty() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(r#""#, None, None)?;
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
    fn test_fibonacci_via_loop() -> anyhow::Result<()> {
        // last example from https://craftinginterpreters.com/control-flow.html#desugaring
        let actual = parse_and_eval_prog(
            r#"
            var a = 0;
            var temp;
            for (var b = 1; a <= 144; b = temp + b) {
              print a;
              temp = a;
              a = b;
            }
        "#,
            None,
            None,
        )?;
        expect![[r#"
            0
            1
            1
            2
            3
            5
            8
            13
            21
            34
            55
            89
            144
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_break() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            for (var i = 0;; i = i + 1) {
              if (i >= 5) break;
              print i;
            }
        "#,
            None,
            None,
        )?;
        expect![[r#"
            0
            1
            2
            3
            4
        "#]]
        .assert_eq(&actual);
        Ok(())
    }

    #[test]
    fn test_native_function_clock() -> anyhow::Result<()> {
        let actual = parse_and_eval_prog(
            r#"
            print clock();
        "#,
            None,
            Some(Rc::new(RefCell::new(20260103f64))),
        )?;
        expect![[r#"
            20260103
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
