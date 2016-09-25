mod builtins;
use value::{Expr, SExpr, Env};

pub struct Interpreter {
    /// The environment maps defined symbols to values.
    env: Env,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut env = Env::new();
        builtins::add_builtins(&mut env);

        Interpreter { env: env }
    }

    pub fn evaluate(&mut self, s: &str) -> Result<Expr, String> {
        let mut expr = try!(::parser::parse(s));
        self.evaluate_expression(&mut expr)
    }


    pub fn evaluate_expression(&mut self, expr: &mut Expr) -> Result<Expr, String> {
        eval_expr(&mut self.env, expr)
    }
}

fn eval_sexpr(env: &mut Env, sexpr: &mut [Expr]) -> Result<Expr, String> {
    debug!("eval sexpr: {:?}", sexpr);
    if sexpr.len() == 0 {
        return Ok(Expr::SExpr(SExpr::empty()));
    }

    for operand in sexpr.iter_mut() {
        *operand = try!(eval_expr(env, operand));
    }

    let (operator, arguments) = sexpr.split_at_mut(1);

    if let Expr::Function(ref f) = operator[0] {
        f.call(env, arguments)
    } else {
        Err(format!("first element should be function, but was {:?}",
                    operator[0]))
    }
}


fn eval_expr(env: &mut Env, expr: &mut Expr) -> Result<Expr, String> {
    match *expr {
        Expr::Integer(_) |
        Expr::QExpr(_) |
        Expr::Function(_) => Ok(expr.clone()),

        // TODO: implement name resolution (identifiers).
        Expr::Symbol(ref symbol) => {
            let val = try!(env.get(&*symbol)
                .ok_or_else(|| format!("undefined symbol: {}", symbol)));
            Ok(val.clone())
        }
        Expr::SExpr(ref mut sexpr) => eval_sexpr(env, &mut sexpr.exprs),
    }
}
