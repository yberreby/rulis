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
        Err(format!("first element should be a symbol, but was {:?}",
                    operator[0]))
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
        "list" => builtins::list(arguments),
        "head" => builtins::head(arguments),
        "tail" => builtins::tail(arguments),
        "join" => builtins::join(arguments),
        "eval" => builtins::eval(arguments),
        _ => Err(format!("unknown builtin: {}", operator)),
    }
}
