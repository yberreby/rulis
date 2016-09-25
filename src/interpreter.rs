use ast::{Expr, SExpr, Symbol};

pub fn eval_sexpr(sexpr: &mut [Expr]) -> Result<Expr, String> {
    if sexpr.len() == 0 {
        return Ok(Expr::SExpr(SExpr::empty()));
    }

    for operand in sexpr.iter_mut() {
        *operand = try!(eval_expr(operand));
    }

    let (operator, arguments) = sexpr.split_at_mut(1);

    if let Expr::Symbol(ref s) = operator[0] {
        call(s, arguments)
    } else {
        Err("first element must be a symbol".into())
    }
}


fn eval_expr(expr: &mut Expr) -> Result<Expr, String> {
    match *expr {
        Expr::Integer(i) => Ok(*expr),
        Expr::SExpr(ref mut sexpr) => eval_sexpr(sexpr),
        Expr::Symbol(_) => Ok(*expr), // TODO: implement
    }
}

fn call(operator: &Symbol, arguments: &[Expr]) -> Result<Expr, String> {
    let mut numeric_arguments = arguments.iter().map(|e| e.as_i64().unwrap());
    let mut result = 0;

    let first_argument = numeric_arguments.next().unwrap();
    if operator == "-" {
        result = -first_argument;
    }

    for arg in numeric_arguments {
        match &**operator {
            "+" => result += arg,
            "-" => result -= arg,
            "*" => result *= arg,
            "/" => {
                if arg == 0 {
                    return Err("tried to divide by zero".into());
                }
                result /= arg
            }
            _ => return Err("cannot call non-existent operator".into()),
        }
    }

    Ok(Expr::Integer(result))

    //   /* Ensure all arguments are numbers */
    //   for (int i = 0; i < a->count; i++) {
    //     if (a->cell[i]->type != LVAL_NUM) {
    //       lval_del(a);
    //       return lval_err("Cannot operate on non-number!");
    //     }
    //   }
    //
    //   /* Pop the first element */
    //   lval* x = lval_pop(a, 0);
    //
    //   /* If no arguments and sub then perform unary negation */
    //   if ((strcmp(op, "-") == 0) && a->count == 0) {
    //     x->num = -x->num;
    //   }
    //
    //   /* While there are still elements remaining */
    //   while (a->count > 0) {
    //
    //     /* Pop the next element */
    //     lval* y = lval_pop(a, 0);
    //
    //     if (strcmp(op, "+") == 0) { x->num += y->num; }
    //     if (strcmp(op, "-") == 0) { x->num -= y->num; }
    //     if (strcmp(op, "*") == 0) { x->num *= y->num; }
    //     if (strcmp(op, "/") == 0) {
    //       if (y->num == 0) {
    //         lval_del(x); lval_del(y);
    //         x = lval_err("Division By Zero!"); break;
    //       }
    //       x->num /= y->num;
    //     }
    //
    //     lval_del(y);
    //   }
    //
    //   lval_del(a); return x;

    // // We're using a function to avoid computing the first operand separately unless it is needed.
    // // The performance impact would probably be small, but computing it upfront could be
    // // semantically incorrect.
    // fn first_operand<'a, I: Iterator<Item = &'a Expr>>(iter: &mut I) -> Result<&'a Expr, String> {
    //     iter.next().ok_or_else(|| "missing first operand".into())
    // }

    // let mut arguments = arguments.iter();
    // let initial_accumulator: Expr;
    // let f: fn(Expr, Expr) -> Expr;

    // match &*operator {
    //     "+" => {
    //         initial_accumulator = 0;
    //         fn g(a: Expr, b: Expr) -> Expr {
    //             a + b
    //         }
    //         f = g;
    //     }
    //     "-" => {
    //         let val: Expr = try!(eval_expr(try!(first_operand(&mut arguments))));
    //         initial_accumulator = if arguments.len() == 1 { -val } else { val };

    //         fn g(a: Expr, b: Expr) -> Expr {
    //             a - b
    //         }
    //         f = g;
    //     }
    //     "*" => {
    //         initial_accumulator = 1;

    //         fn g(a: Expr, b: Expr) -> Expr {
    //             a * b
    //         }
    //         f = g;
    //     }
    //     "/" => {
    //         initial_accumulator = try!(eval_expr(try!(first_operand(&mut arguments))));

    //         fn g(a: Expr, b: Expr) -> Expr {
    //             a / b
    //         }
    //         f = g;
    //     }
    // }

    // Ok(arguments.fold(initial_accumulator,
    //                   |acc, operand| f(acc, eval_expr(operand).unwrap())))
}
