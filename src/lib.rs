mod grammar;
mod ast;
mod interpreter;

pub fn eval(s: &str) -> i64 {
    let expr = grammar::parse_Expr(s).expect("failed to parse");
    interpreter::eval_expr(&expr)
}

#[cfg(test)]
mod test;
