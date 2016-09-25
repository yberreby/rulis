mod builtins;
use ast::{Expr, SExpr};

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


pub fn eval_expr(expr: &mut Expr) -> Result<Expr, String> {
    match *expr {
        Expr::Integer(_) | Expr::QExpr(_) => Ok(expr.clone()),
        Expr::SExpr(ref mut sexpr) => eval_sexpr(&mut sexpr.exprs),
        // TODO: implement name resolution (identifiers).
        Expr::Symbol(_) => Ok(expr.clone()),
    }
}

fn call(operator: &str, arguments: &[Expr]) -> Result<Expr, String> {
    match operator {
        "+" | "-" | "*" | "/" => builtins::arithmetic_operation(operator, arguments),
        "list" => builtins::list(operator, arguments),
        "head" => builtins::head(operator, arguments),
        "tail" => builtins::tail(operator, arguments),
        "join" => builtins::join(operator, arguments),
        "eval" => builtins::eval(operator, arguments),
        _ => Err(format!("unknown builtin: {}", operator)),
    }
}

fn arithmetic_operation(operator: &str, arguments: &[Expr]) -> Result<Expr, String> {
    let mut numeric_arguments = arguments.iter().map(|e| e.as_i64().unwrap());
    let first_argument = numeric_arguments.next().unwrap();
    let mut result = first_argument;

    if operator == "-" && arguments.len() == 1 {
        result = -result;
    }

    for arg in numeric_arguments {
        match operator {
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
}
