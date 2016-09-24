mod grammar;
mod ast;
mod interpreter;

pub fn eval(s: &str) -> Result<i64, String> {
    let expr = try!(grammar::parse_Expr(s).map_err(|e| format!("{:?}", e)));
    let res = try!(interpreter::eval_expr(&expr));
    Ok(res)
}

#[cfg(test)]
mod test;
