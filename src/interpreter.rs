use ast::{Expr, Operation, Operator};

pub fn eval_expr(expr: &Expr) -> i64 {
    match *expr {
        Expr::Integer(i) => i,
        Expr::Operation(ref op) => eval_op(&op),
    }
}

// This code could use some .fold()-ing.
fn eval_op(op: &Operation) -> i64 {
    match op.operator {
        Operator::Plus => op.operands.iter().fold(0, |acc, operand| acc + eval_expr(operand)),
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
        Operator::Multiply => {
            let mut acc = 1;

            for operand in &op.operands {
                acc *= eval_expr(operand)
            }

            acc
        }
        Operator::Divide => {
            let mut iter = op.operands.iter();
            let mut acc = eval_expr(&iter.next().unwrap());

            for operand in iter {
                acc /= eval_expr(operand)
            }

            acc
        }
    }
}
