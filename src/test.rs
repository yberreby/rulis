use value::*;
use eval;
use std::fmt::Debug;
use Interpreter;

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
    for (src, expected) in runs {
        print!("{}... ", src);

        let mut interpreter = Interpreter::new();
        let mut last_val = None;
        for line in src.lines() {
            last_val = Some(interpreter.evaluate(line));
        }

        let actual: T = actual_f(last_val.unwrap());
        let expected: T = expected_f(expected);
        assert_eq!(actual,
                   expected,
                   "expected {:?}, got {:?} when evaluating {:?}",
                   expected,
                   actual,
                   src);
        println!("OK");
    }
}

#[test]
fn incorrect_code_is_rejected_and_doesnt_cause_panics() {
    let sources = vec!["(def {a b c} {+ (* a c) b})", "(+ 5 ((* 2 428))", "(+ 5 {})",
    //":"
    ];

    for src in &sources {
        assert!(eval(src).is_err());
    }
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

    test_runs(runs.into_iter(), |x| x, |i| Ok(Expr::Integer(i)));
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

#[test]
fn lambdas_work() {
    let runs = vec![
(r"(def {myFunc} (\ {a} {+ a 5}))
(myFunc 10)",
                     15),
(r"(def {add} (\ {a b} {+ a b}))
(def {add5} (add 5))
(add5 11)
", 16),

    ];

    test_runs(runs.into_iter(), |x| x, |i| Ok(Expr::Integer(i)));
}


#[test]
fn fun_works() {
    let runs = vec![
(r"
(def {fun} (\ {args body} {def (head args) (\ (tail args) body)}))
(fun {add a b} {+ a b})
(add 5 7)",
                     Expr::Integer(12)),

    ];

    test_runs(runs.into_iter(), |x| x, |x| Ok(x));
}

#[test]
fn conditionals_work() {
    let runs = vec![
        ("(if 1 {+ 1 2} {+ 5 6})", Expr::Integer(3)),
        ("(if 0 {+ 1 2} {+ 5 6})", Expr::Integer(11)),
        ("(if 1 {17})", Expr::Integer(17)),
        ("(if 0 {17} {45})", Expr::Integer(45)),
    ];

    test_runs(runs.into_iter(), |x| x, |x| Ok(x));
}

#[test]
fn comparison_operators_work() {
    let runs = vec![
        ("(<= 5 7)", 1),
        ("(>= 5 7)", 0),
        ("(<= 8 8)", 1),
        ("(>= 8 8)", 1),
        ("(< 5 7)", 1),
        ("(< 7 5)", 0),
        ("(> 5 7)", 0),
        ("(> 7 5)", 1),
        ("(> 10 5)", 1),
        ("(<= 88 5)", 0),
        ("(== 5 6)", 0),
        ("(== 5 {})", 0),
        ("(== 1 1)", 1),
        ("(!= {} 56)", 1),
        ("(== {1 2 3 {5 6}} {1   2  3   {5 6}})", 1),
    ];

    test_runs(runs.into_iter(), |x| x, |x| Ok(Expr::Integer(x)));
}

#[test]
fn negative_num_literals_work() {
    let runs = vec![
        ("-65", -65),
        ("(- -65)", 65),
        ("(< -8 8)", 1),
    ];

    test_runs(runs.into_iter(), |x| x, |x| Ok(Expr::Integer(x)));
}

#[test]
fn recursion_works() {
    let runs = vec![
        (r"(def {fun} (\ {args body} {def (head args) (\ (tail args) body)}))
(fun {len l} {if (== l {}) {0} {+ 1 (len (tail l))} })
(len {a 5 6 {}})", 4),
    ];

    test_runs(runs.into_iter(), |x| x, |x| Ok(Expr::Integer(x)));
}



// Recursion limit test & micro-benchmark:
//
// (def {f} (\ {x acc} { if (== x 0) {acc} {f (- x 1) (+ acc 1)} }))
// (f 10000 0)
