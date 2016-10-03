mod builtins;
use value::{Expr, SExpr, Env, EnvPtr};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Interpreter {
    /// The environment maps defined symbols to values.
    global_env_ptr: EnvPtr,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let env_ptr = Rc::new(RefCell::new(Env::empty()));
        builtins::add_builtins(env_ptr.clone());

        Interpreter { global_env_ptr: env_ptr }
    }

    pub fn evaluate(&mut self, s: &str) -> Result<Expr, String> {
        let expr = try!(::parser::parse(s));
        self.evaluate_expression(expr)
    }


    pub fn evaluate_expression(&mut self, expr: Expr) -> Result<Expr, String> {
        eval_expr(self.global_env_ptr.clone(), expr)
    }
}

pub fn eval_sexpr(env_ptr: EnvPtr, mut sexpr: Vec<Expr>) -> Result<Expr, String> {
    debug!("eval sexpr: {:?}", sexpr);

    match sexpr.len() {
        0 => return Ok(Expr::SExpr(SExpr::empty())),
        1 => {
            // XXX: clone
            return Ok(try!(eval_expr(env_ptr, sexpr[0].clone())));
        }
        _ => {}
    };

    for operand in &mut sexpr {
        *operand = try!(eval_expr(env_ptr.clone(), operand.clone())); // XXX: avoid cloning operand
    }

    let operator = sexpr.remove(0);
    let arguments = sexpr;

    if let Expr::Function(mut f) = operator {
        f.call(env_ptr.clone(), arguments)
    } else {
        Err(format!("first element should be function, but was {:?}", operator))
    }
}


fn eval_expr(env_ptr: EnvPtr, expr: Expr) -> Result<Expr, String> {
    match expr {
        Expr::Integer(_) |
        Expr::QExpr(_) |
        Expr::Function(_) => Ok(expr.clone()),

        Expr::Symbol(ref symbol) => {
            let val = try!(env_ptr.borrow()
                .get(&*symbol)
                .ok_or_else(|| format!("undefined symbol: {}", symbol)));
            Ok(val.clone())
        }
        Expr::SExpr(sexpr) => eval_sexpr(env_ptr.clone(), sexpr.exprs),
    }
}
