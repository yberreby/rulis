use ast::{Expr, SExpr};

pub fn eval_sexpr(sexpr: &[Expr]) -> Result<Value, String> {
    let (operator, operands) = sexpr.split_at_mut(1);
    for operand in operands.iter_mut() {
        operand = try!(eval_expr(&operand));
    }

    unimplemented!()
}

fn eval_expr(expr: &Expr) -> Result<Value, String> {
    match *expr {
        Expr::Integer(i) => Ok(i),
        Expr::SExpr(ref sexpr) => eval_sexpr(&sexpr),
        Expr::Symbol(_) => panic!("symbols not implemented"),
    }
}

// // This *might* be over-engineered... ahem.
// fn eval_op(op: &Operation) -> Result<Value, String> {
//     // We're using a function to avoid computing the first operand separately unless it is needed.
//     // The performance impact would probably be small, but computing it upfront could be
//     // semantically incorrect.
//     fn first_operand<'a, I: Iterator<Item = &'a Expr>>(iter: &mut I) -> Result<&'a Expr, String> {
//         iter.next().ok_or_else(|| "missing first operand".into())
//     }
//
//     let mut operands = op.operands.iter();
//     let initial_accumulator: Value;
//     let f: fn(Value, Value) -> Value;
//
//     match op.operator {
//         Operator::Plus => {
//             initial_accumulator = 0;
//             fn g(a: Value, b: Value) -> Value {
//                 a + b
//             }
//             f = g;
//         }
//         Operator::Minus => {
//             let val: Value = try!(eval_expr(try!(first_operand(&mut operands))));
//             initial_accumulator = if op.operands.len() == 1 { -val } else { val };
//
//             fn g(a: Value, b: Value) -> Value {
//                 a - b
//             }
//             f = g;
//         }
//         Operator::Multiply => {
//             initial_accumulator = 1;
//
//             fn g(a: Value, b: Value) -> Value {
//                 a * b
//             }
//             f = g;
//         }
//         Operator::Divide => {
//             initial_accumulator = try!(eval_expr(try!(first_operand(&mut operands))));
//
//             fn g(a: Value, b: Value) -> Value {
//                 a / b
//             }
//             f = g;
//         }
//     }
//
//     Ok(operands.fold(initial_accumulator,
//                      |acc, operand| f(acc, eval_expr(operand).unwrap())))
// }
