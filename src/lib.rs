mod grammar;
mod ast;
mod interpreter;

pub fn eval(s: &str) -> Result<i64, String> {
    let expr = try!(grammar::parse_Expr(s).map_err(|e| format!("{:?}", e)));
    Ok(interpreter::eval_expr(&expr))
}

#[cfg(test)]
mod test;
