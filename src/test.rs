use ast;
use eval;

#[test]
fn invalid_code_is_rejected() {
    let src = "(+ 5 ((* 2 428))";
    assert!(eval(src).is_err());
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

    for &(ref operation, expected) in &runs {
        print!("{}... ", operation);
        let expected = Ok(ast::Expr::Integer(expected));
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
