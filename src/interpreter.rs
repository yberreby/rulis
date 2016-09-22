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
    // Yes this is horribly inefficient.
    let f: Box<Fn(i64, i64) -> i64>;


    match op.operator {
        Operator::Plus => {
            initial_accumulator = 0;
            f = Box::new(|a, b| a + b)
        }
        Operator::Minus => {
            let val = eval_expr(iter.next().unwrap());
            initial_accumulator = if op.operands.len() == 1 { -val } else { val };
            f = Box::new(|a, b| a - b)
        }
        Operator::Multiply => {
            initial_accumulator = 1;
            f = Box::new(|a, b| a * b)
        }
        Operator::Divide => {
            initial_accumulator = eval_expr(iter.next().unwrap());
            f = Box::new(|a, b| a / b)
        }
    }

    iter.fold(initial_accumulator,
              |acc, operand| (*f)(acc, eval_expr(operand)))
}
