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
        let mut expr = try!(::parser::parse(s));
        self.evaluate_expression(&mut expr)
    }


    pub fn evaluate_expression(&mut self, expr: &mut Expr) -> Result<Expr, String> {
        eval_expr(self.global_env_ptr.clone(), expr)
    }
}

pub fn eval_sexpr(env_ptr: EnvPtr, sexpr: &mut [Expr]) -> Result<Expr, String> {
    debug!("eval sexpr: {:?}", sexpr);

    match sexpr.len() {
        0 => return Ok(Expr::SExpr(SExpr::empty())),
        1 => {
            // XXX: clone
            return Ok(try!(eval_expr(env_ptr, &mut sexpr[0].clone())));
        }
        _ => {}
    };

    for operand in sexpr.iter_mut() {
        *operand = try!(eval_expr(env_ptr.clone(), operand));
    }

    let (operator, arguments) = sexpr.split_at_mut(1);

    if let Expr::Function(ref mut f) = operator[0] {
        f.call(env_ptr.clone(), arguments)
    } else {
        Err(format!("first element should be function, but was {:?}",
                    operator[0]))
    }
}


fn eval_expr(env_ptr: EnvPtr, expr: &mut Expr) -> Result<Expr, String> {
    match *expr {
        Expr::Integer(_) |
        Expr::QExpr(_) |
        Expr::Function(_) => Ok(expr.clone()),

        Expr::Symbol(ref symbol) => {
            let val = try!(env_ptr.borrow()
                .get(&*symbol)
                .ok_or_else(|| format!("undefined symbol: {}", symbol)));
            Ok(val.clone())
        }
        Expr::SExpr(ref mut sexpr) => eval_sexpr(env_ptr.clone(), &mut sexpr.exprs),
    }
}
