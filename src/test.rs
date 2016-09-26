use value::Expr;
use eval;

#[test]
fn invalid_code_is_rejected() {
    let src = "(+ 5 ((* 2 428))";
    assert!(eval(src).is_err());
}

fn test_runs<'a, I: Iterator<Item = (&'a str, Result<Expr, String>)>>(runs: I) {
    for (operation, expected) in runs {
        print!("{}... ", operation);
        let actual = eval(&operation);
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
fn simple_arithmetic_evaluation_works() {
    let runs = [("(+ 42 268)", 310),
                ("(- 96)", -96),
                ("(- 96 20 1 1 1)", 73),
                ("(* 5 7)", 35),
                ("(* 2 2 2)", 8),
                ("(* 5 7 11)", 385),
                ("(* 1 1 5 1 1 1 7 1 1 1 11)", 385),
                ("(* 1 1 5 1 1 1 7 0 1 1 11)", 0),
                ("(/ 128674 48 3)", 893)];

    test_runs(runs.iter().map(|&(s, i)| (s, Ok(Expr::Integer(i)))));
}

#[test]
fn qexpressions_builtins_work() {
    let runs = [("(list 1 2 3 4)", "{1 2 3 4}"),
                ("{head (list 1 2 3 4)}", "{head (list 1 2 3 4)}"),
                ("(head (list 1 2 3 4))", "{1}"),
                ("(eval {head (list 1 2 3 4)})", "{1}"),
                ("(tail {tail tail tail})", "{tail tail}"),
                ("(eval (tail {tail tail {5 6 7}}))", "{6 7}"),
                ("(eval {+ 1 2})", "3"),
                ("(join {+ 1 2} {6548} {/ / /})", "{+ 1 2 6548 / / /}")];

    for &(ref operation, expected) in &runs {
        print!("{}... ", operation);
        let expected = Ok(expected);
        let actual = eval(&operation).map(|x| x.to_string());
        assert_eq!(actual.as_ref().map(|s| &s[..]),
                   expected,
                   "expected {:?}, got {:?} when evaluating {:?}",
                   expected,
                   actual,
                   operation);
        println!("OK");
    }
}
