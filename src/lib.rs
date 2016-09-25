#[macro_use]
extern crate log;
extern crate num;

mod parser;
mod ast;
mod interpreter;

pub fn eval(s: &str) -> Result<ast::Expr, String> {
    let mut expr = try!(parser::parse(s).map_err(|e| format!("{}", e)));
    debug!("Expr: (debug): {:?}", expr);
    debug!("Expr: (display): {}", expr);
    let res = try!(interpreter::eval_expr(&mut expr));
    Ok(res)
}

#[cfg(test)]
mod test;
