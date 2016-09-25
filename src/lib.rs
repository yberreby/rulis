#[macro_use]
extern crate log;
extern crate num;

mod parser;
mod value;
mod interpreter;
pub use self::interpreter::Interpreter;

/// Evaluate a source string in a fresh context.
pub fn eval(s: &str) -> Result<value::Expr, String> {
    let mut expr = try!(parser::parse(s).map_err(|e| format!("{}", e)));
    debug!("Expr: (debug): {:?}", expr);
    debug!("Expr: (display): {}", expr);
    let mut interpreter = Interpreter::new();
    let res = try!(interpreter.evaluate_expression(&mut expr));
    Ok(res)
}

#[cfg(test)]
mod test;
