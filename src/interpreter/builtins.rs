use ast::Expr;

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

pub fn list(operator: &str, arguments: &[Expr]) -> Result<Expr, String> {
    unimplemented!()
}
pub fn head(operator: &str, arguments: &[Expr]) -> Result<Expr, String> {
    unimplemented!()
}
pub fn tail(operator: &str, arguments: &[Expr]) -> Result<Expr, String> {
    unimplemented!()
}
pub fn join(operator: &str, arguments: &[Expr]) -> Result<Expr, String> {
    unimplemented!()
}
pub fn eval(operator: &str, arguments: &[Expr]) -> Result<Expr, String> {
    unimplemented!()
}
