use ast;
use grammar;

#[test]
fn it_works() {
    let src = "+ 5 (* 2 428)";
    let expected = ast::Expr::Operation(ast::Operation {
        operator: ast::Operator::Plus,
        operands: vec![ast::Expr::Integer(5),
                       ast::Expr::Operation(ast::Operation {
                           operator: ast::Operator::Multiply,
                           operands: vec![
                ast::Expr::Integer(2),
                ast::Expr::Integer(428),
            ],
                       })],
    });

    assert_eq!(grammar::parse_Program(src), Ok(expected));
}

#[test]
fn invalid_code_is_rejected() {
    let src = "+ 5 ((* 2 428)";
    assert!(grammar::parse_Program(src).is_err());
}
