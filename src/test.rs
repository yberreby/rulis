use value::Expr;
use eval;
use std::fmt::Debug;

/// Generic helper method to simplify testing.
fn test_runs<'a,
             T: PartialEq + Debug,
             U,
             I: Iterator<Item = (&'a str, U)>,
             F: Fn(Result<Expr, String>) -> T,
             G: Fn(U) -> T>
    (runs: I,
     actual_f: F,
     expected_f: G) {
    for (operation, expected) in runs {
        print!("{}... ", operation);
        let actual: T = actual_f(eval(&operation));
        let expected: T = expected_f(expected);
        assert_eq!(actual,
                   expected,
                   "expected {:?}, got {:?} when evaluating {:?}",
                   expected,
                   actual,
                   operation);
        println!("OK");
    }
}


#[test]
fn invalid_code_is_rejected() {
    let src = "(+ 5 ((* 2 428))";
    assert!(eval(src).is_err());
}

#[test]
fn simple_arithmetic_evaluation_works() {
    let runs = vec![("(+ 42 268)", 310),
                    ("(- 96)", -96),
                    ("(- 96 20 1 1 1)", 73),
                    ("(* 5 7)", 35),
                    ("(* 2 2 2)", 8),
                    ("(* 5 7 11)", 385),
                    ("(* 1 1 5 1 1 1 7 1 1 1 11)", 385),
                    ("(* 1 1 5 1 1 1 7 0 1 1 11)", 0),
                    ("(/ 128674 48 3)", 893)];

    test_runs(runs.into_iter(), |s| s, |i| Ok(Expr::Integer(i)));
}

#[test]
fn qexpressions_builtins_work() {
    let runs = vec![("(list 1 2 3 4)", "{1 2 3 4}"),
                    ("{head (list 1 2 3 4)}", "{head (list 1 2 3 4)}"),
                    ("(head (list 1 2 3 4))", "{1}"),
                    ("(eval {head (list 1 2 3 4)})", "{1}"),
                    ("(tail {tail tail tail})", "{tail tail}"),
                    ("(eval (tail {tail tail {5 6 7}}))", "{6 7}"),
                    ("(eval {+ 1 2})", "3"),
                    ("(join {+ 1 2} {6548} {/ / /})", "{+ 1 2 6548 / / /}")];

    test_runs(runs.into_iter(),
              |r| r.map(|x| x.to_string()),
              |s| Ok(s.into()));
}
