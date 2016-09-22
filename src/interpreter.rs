use ast::{Expr, Operation, Operator};

pub fn eval(expr: Expr) -> i64 {
    match expr {
        Expr::Integer(i) => i,
        Expr::Operation(op) => eval_op(&op),
    }
}

fn eval_op(op: &Operation) -> i64 {
    let mut res = 0;
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::eval_op;
    use ast::{Expr, Operation, Operator};

    #[test]
    fn eval_op_works() {
        let runs = [(Operation {
                        operator: Operator::Plus,
                        operands: vec![
                        Expr::Integer(5),
                        Expr::Integer(1),
                        Expr::Integer(27),
                    ],
                    },
                     33)];

        for &(ref operation, expected) in &runs {
            let actual = eval_op(&operation);
            assert_eq!(actual,
                       expected,
                       "expected {}, got {} when evaluating {:?}",
                       expected,
                       actual,
                       operation);
        }
    }
}
