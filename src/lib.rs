mod parser;
mod ast;
// mod interpreter;

pub fn eval(s: &str) -> Result<ast::Expr, String> {
    let sexpr = try!(parser::parse(s).map_err(|e| format!("{:?}", e)));
    println!("AST\n: {:?}", sexpr);
    println!("S-Expression: {}", sexpr);
    unimplemented!()
    // let res = try!(interpreter::eval_sexpr(&sexpr));
    // Ok(res)
}

// #[cfg(test)]
// mod test;
