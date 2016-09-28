use std::rc::Rc;
use std::cell::RefCell;
use value::{Expr, SExpr, QExpr, Env, Function, InnerFunc, Lambda};

// TODO: clean up error handling in this module. It's a mess.

pub fn add_builtins(env: &mut Env) {
    add_builtin_fn(env, "def", builtin_def);
    add_builtin_fn(env, "\\", builtin_lambda);

    add_builtin_fn(env, "list", builtin_list);
    add_builtin_fn(env, "head", builtin_head);
    add_builtin_fn(env, "tail", builtin_tail);
    add_builtin_fn(env, "eval", builtin_eval);
    add_builtin_fn(env, "join", builtin_join);

    add_builtin_fn(env, "+", builtin_add);
    add_builtin_fn(env, "-", builtin_sub);
    add_builtin_fn(env, "*", builtin_mul);
    add_builtin_fn(env, "/", builtin_div);
}

fn add_builtin_fn<S: Into<String>>(env: &mut Env, name: S, f: InnerFunc) {
    add_builtin(env, name.into(), Expr::Function(Function::from_builtin(f)));
}

fn add_builtin(env: &mut Env, name: String, value: Expr) {
    env.define_local(name, value);
}


pub fn arithmetic_operation(operator: &str, arguments: &[Expr]) -> Result<Expr, String> {
    let mut numeric_arguments = arguments.iter().map(|e| e.as_i64().unwrap());
    let first_argument = numeric_arguments.next().unwrap();
    let mut result = first_argument;

    if operator == "-" && arguments.len() == 1 {
        result = -result;
    }

    for arg in numeric_arguments {
        match operator {
            "+" => result += arg,
            "-" => result -= arg,
            "*" => result *= arg,
            "/" => {
                if arg == 0 {
                    return Err("tried to divide by zero".into());
                }
                result /= arg
            }
            _ => return Err("cannot call non-existent operator".into()),
        }
    }

    Ok(Expr::Integer(result))
}

fn builtin_add(_env: &mut Env, arguments: &[Expr]) -> Result<Expr, String> {
    arithmetic_operation("+", arguments)
}

fn builtin_sub(_env: &mut Env, arguments: &[Expr]) -> Result<Expr, String> {
    arithmetic_operation("-", arguments)
}

fn builtin_mul(_env: &mut Env, arguments: &[Expr]) -> Result<Expr, String> {
    arithmetic_operation("*", arguments)
}

fn builtin_div(_env: &mut Env, arguments: &[Expr]) -> Result<Expr, String> {
    arithmetic_operation("/", arguments)
}


fn builtin_list(_env: &mut Env, arguments: &[Expr]) -> Result<Expr, String> {
    Ok(Expr::QExpr(QExpr::new(arguments.into())))
}

/// head returns a QExpr containing one element, the first element of the qexpr it was passed.
fn builtin_head(_env: &mut Env, arguments: &[Expr]) -> Result<Expr, String> {
    if arguments.len() > 1 {
        return Err("too many arguments".into());
    }

    if let Expr::QExpr(qexpr) = arguments[0].clone() {
        let head = qexpr[0].clone();
        Ok(Expr::QExpr(QExpr::new(vec![head])))
    } else {
        return Err("type error, expected Q-Expression".into());
    }
}

fn builtin_tail(_env: &mut Env, arguments: &[Expr]) -> Result<Expr, String> {
    if arguments.len() > 1 {
        return Err("too many arguments".into());
    }

    if let Expr::QExpr(qexpr) = arguments[0].clone() {
        let tail = qexpr[1..].to_vec();
        Ok(Expr::QExpr(QExpr::new(tail)))
    } else {
        return Err("type error, expected Q-Expression".into());
    }
}

fn builtin_join(_env: &mut Env, arguments: &[Expr]) -> Result<Expr, String> {
    let mut v = Vec::new();

    for arg in arguments.to_vec() {
        if let Expr::QExpr(qexpr) = arg {
            for expr in qexpr.exprs {
                v.push(expr);
            }
        } else {
            return Err(format!("expected Q-Expression, found {:?}", arg));
        }
    }

    Ok(Expr::QExpr(QExpr::new(v)))
}

fn builtin_eval(env: &mut Env, arguments: &[Expr]) -> Result<Expr, String> {
    if arguments.len() > 1 {
        return Err("too many arguments".into());
    }

    if let Expr::QExpr(mut qexpr) = arguments[0].clone() {
        println!("eval qexpr: {}", qexpr);
        super::eval_sexpr(env, &mut qexpr)
    } else {
        return Err("type error, expected Q-Expression".into());
    }
}

fn builtin_def(env: &mut Env, arguments: &[Expr]) -> Result<Expr, String> {
    if let Expr::QExpr(symbols) = arguments[0].clone() {
        for (i, maybe_symbol) in symbols.to_vec().into_iter().enumerate() {
            if let Expr::Symbol(s) = maybe_symbol {
                // We need +1 to skip past the first argument, which is the QExpr of symbols.
                env.define_global(s, arguments[i + 1].clone());
            } else {
                return Err(format!("expected symbol, found {:?}", maybe_symbol));
            }
        }


        Ok(Expr::SExpr(SExpr::empty()))
    } else {
        return Err(format!("expected Q-expression as first argument, found {:?}",
                           arguments[0]));
    }
}


fn builtin_lambda(env: &mut Env, arguments: &[Expr]) -> Result<Expr, String> {
    if arguments.len() != 2 {
        return Err(format!("expected 2 arguments, got {}", arguments.len()));
    }

    if let Expr::QExpr(params) = arguments[0].clone() {
        if let Expr::QExpr(body) = arguments[1].clone() {
            // Note: we're _cloning_ the parent environment here, not keeping a reference to it.
            Ok(Expr::Function(Function::Lambda(try!(Lambda::new(params, body, Rc::new(RefCell::new(env.clone())))))))
        } else {
            return Err(format!("expected Q-expression as second argument, found {:?}",
                               arguments[1]));
        }
    } else {
        return Err(format!("expected Q-expression as first argument, found {:?}",
                           arguments[0]));
    }
}
