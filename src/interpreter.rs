use ast::{Expr, Operation, Operator};

pub fn eval_expr(expr: &Expr) -> i64 {
    match *expr {
        Expr::Integer(i) => i,
        Expr::Operation(ref op) => eval_op(&op),
    }
}

// This code could use some .fold()-ing.
fn eval_op(op: &Operation) -> i64 {
    let mut iter = op.operands.iter();
    let initial_accumulator: i64;
    let f: fn(i64, i64) -> i64;

    match op.operator {
        Operator::Plus => {
            initial_accumulator = 0;
            fn g(a: i64, b: i64) -> i64 {
                a + b
            }
            f = g;
        }
        Operator::Minus => {
            let val = eval_expr(iter.next().unwrap());
            initial_accumulator = if op.operands.len() == 1 { -val } else { val };

            fn g(a: i64, b: i64) -> i64 {
                a - b
            }
            f = g;
        }
        Operator::Multiply => {
            initial_accumulator = 1;

            fn g(a: i64, b: i64) -> i64 {
                a * b
            }
            f = g;
        }
        Operator::Divide => {
            initial_accumulator = eval_expr(iter.next().unwrap());

            fn g(a: i64, b: i64) -> i64 {
                a / b
            }
            f = g;
        }
    }

    iter.fold(initial_accumulator,
              |acc, operand| f(acc, eval_expr(operand)))
}
