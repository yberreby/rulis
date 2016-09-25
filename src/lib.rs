extern crate num;

mod parser;
mod ast;
mod interpreter;

pub fn eval(s: &str) -> Result<ast::Expr, String> {
    let mut sexpr = try!(parser::parse(s).map_err(|e| format!("{:?}", e)));
    println!("AST: {:?}", sexpr);
    println!("S-Expression: {}", sexpr);
    let res = try!(interpreter::eval_sexpr(&mut sexpr));
    Ok(res)
}

#[cfg(test)]
mod test;
