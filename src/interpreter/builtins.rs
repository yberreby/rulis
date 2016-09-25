use ast::{Expr, QExpr, SExpr};

pub fn arithmetic_operation(operator: &str, arguments: &[Expr]) -> Result<Expr, String> {
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

pub fn list(arguments: &[Expr]) -> Result<Expr, String> {
    Ok(Expr::QExpr(QExpr::new(arguments.into())))
}

/// head returns a QExpr containing one element, the first element of the qexpr it was passed.
pub fn head(arguments: &[Expr]) -> Result<Expr, String> {
    if arguments.len() > 1 {
        return Err("too many arguments".into());
    }

    if let Expr::QExpr(qexpr) = arguments[0].clone() {
        let head = qexpr[0].clone();
        Ok(Expr::QExpr(QExpr::new(vec![head])))
    } else {
        return Err("type error, expected Q-Expression".into());
    }
}

pub fn tail(arguments: &[Expr]) -> Result<Expr, String> {
    if arguments.len() > 1 {
        return Err("too many arguments".into());
    }

    if let Expr::QExpr(qexpr) = arguments[0].clone() {
        let tail = qexpr[1..].to_vec();
        Ok(Expr::QExpr(QExpr::new(tail)))
    } else {
        return Err("type error, expected Q-Expression".into());
    }
}

pub fn join(arguments: &[Expr]) -> Result<Expr, String> {
    unimplemented!()
}

pub fn eval(arguments: &[Expr]) -> Result<Expr, String> {
    if arguments.len() > 1 {
        return Err("too many arguments".into());
    }

    if let Expr::QExpr(mut qexpr) = arguments[0].clone() {
        println!("eval qexpr: {}", qexpr);
        super::eval_sexpr(&mut qexpr)
    } else {
        return Err("type error, expected Q-Expression".into());
    }
}
