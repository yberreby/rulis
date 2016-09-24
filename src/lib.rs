#[macro_use]
extern crate nom;
mod parser;
mod ast;
// mod interpreter;

pub fn eval(s: &str) -> Result<i64, String> {
    let expr = try!(parser::parse(s).map_err(|e| format!("{:?}", e)));
    unimplemented!()
    // let res = try!(interpreter::eval_expr(&expr));
    // Ok(res)
}

// #[cfg(test)]
// mod test;
