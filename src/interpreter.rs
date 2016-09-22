use ast::{Expr, Operation, Operator};

pub fn eval_expr(expr: &Expr) -> i64 {
    match *expr {
        Expr::Integer(i) => i,
        Expr::Operation(ref op) => eval_op(&op),
    }
}

fn eval_op(op: &Operation) -> i64 {
    match op.operator {
        Operator::Plus => {
            let mut acc = 0;
            for operand in &op.operands {
                acc += eval_expr(operand)
            }
            acc
        }
        Operator::Minus => {
            let mut iter = op.operands.iter();
            let mut acc = eval_expr(&iter.next().unwrap());

            if op.operands.len() == 1 {
                acc = -acc;
            } else {
                for operand in iter {
                    acc -= eval_expr(operand)
                }
            }

            acc
        }
        _ => unimplemented!(),
    }
}

// #[cfg(test)]
// mod test {
//     use super::eval_op;
//     use ast::{Expr, Operation, Operator};
//
//     #[test]
//     fn eval_op_works() {
//         let runs = [(Operation {
//                         operator: Operator::Plus,
//                         operands: vec![
//                         Expr::Integer(5),
//                         Expr::Integer(1),
//                         Expr::Integer(27),
//                     ],
//                     },
//                      33),
//                     (Operation {
//                         operator: Operator::Minus,
//                         operands: vec![
//                         Expr::Integer(247),
//                         Expr::Integer(18),
//                         Expr::Integer(297),
//                     ],
//                     },
//                      -68),
//                     (Operation {
//                         operator: Operator::Multiply,
//                         operands: vec![
//                         Expr::Integer(247),
//                         Expr::Integer(18),
//                         Expr::Integer(297),
//                     ],
//                     },
//                      1320462)];
//
//         for &(ref operation, expected) in &runs {
//             let actual = eval_op(&operation);
//             assert_eq!(actual,
//                        expected,
//                        "expected {}, got {} when evaluating {:?}",
//                        expected,
//                        actual,
//                        operation);
//         }
//     }
// }
