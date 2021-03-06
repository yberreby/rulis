use value::*;
use super::eval_sexpr;

// TODO: clean up error handling in this module. It's a mess.

/// Kind of variable declaration.
enum DeclKind {
    Global,
    Local,
}

/// Kind of comparison.
enum CmpKind {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

pub fn add_builtins(env: EnvPtr) {
    add_builtin_fn(env.clone(), "def", builtin_def);
    add_builtin_fn(env.clone(), "=", builtin_local_def);
    add_builtin_fn(env.clone(), "\\", builtin_lambda);

    add_builtin_fn(env.clone(), "list", builtin_list);
    add_builtin_fn(env.clone(), "head", builtin_head);
    add_builtin_fn(env.clone(), "tail", builtin_tail);
    add_builtin_fn(env.clone(), "eval", builtin_eval);
    add_builtin_fn(env.clone(), "join", builtin_join);

    add_builtin_fn(env.clone(), "+", builtin_add);
    add_builtin_fn(env.clone(), "-", builtin_sub);
    add_builtin_fn(env.clone(), "*", builtin_mul);
    add_builtin_fn(env.clone(), "/", builtin_div);

    add_builtin_fn(env.clone(), "if", builtin_if);
    add_builtin_fn(env.clone(), ">", builtin_greater_than);
    add_builtin_fn(env.clone(), "<", builtin_less_than);
    add_builtin_fn(env.clone(), ">=", builtin_greater_than_or_equal);
    add_builtin_fn(env.clone(), "<=", builtin_lesser_than_or_equal);
    add_builtin_fn(env.clone(), "==", builtin_is_eq);
    add_builtin_fn(env.clone(), "!=", builtin_is_not_eq);
}

fn add_builtin_fn<S: Into<String>>(env: EnvPtr, name: S, f: InnerFunc) {
    add_builtin(env, name.into(), Expr::Function(Function::from_builtin(f)));
}

fn add_builtin(env: EnvPtr, name: String, value: Expr) {
    env.define_local(name, value);
}


pub fn arithmetic_operation(operator: &str, arguments: Vec<Expr>) -> Result<Expr, String> {
    let numeric_arguments_res: Result<Vec<i64>, String> = arguments.iter()
        .map(|e| e.as_i64().ok_or_else(|| "arguments should all be numbers".into()))
        .collect();
    let mut numeric_arguments = try!(numeric_arguments_res).into_iter();

    let first_argument: i64 = try!(numeric_arguments.next()
        .ok_or_else(|| "missing first argument".to_string()));
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

fn builtin_add(_env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
    arithmetic_operation("+", arguments)
}

fn builtin_sub(_env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
    arithmetic_operation("-", arguments)
}

fn builtin_mul(_env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
    arithmetic_operation("*", arguments)
}

fn builtin_div(_env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
    arithmetic_operation("/", arguments)
}


fn builtin_list(_env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
    Ok(Expr::QExpr(QExpr::new(arguments.into())))
}

/// `head` returns a `QExpr` containing one element, the first element of the qexpr it was passed.
fn builtin_head(_env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
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

fn builtin_tail(_env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
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

fn builtin_join(_env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
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

fn builtin_eval(env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
    if arguments.len() > 1 {
        return Err("too many arguments".into());
    }

    if let Expr::QExpr(qexpr) = arguments[0].clone() {
        debug!("eval qexpr: {}", qexpr);
        eval_sexpr(env, qexpr.exprs)
    } else {
        return Err("type error, expected Q-Expression".into());
    }
}

fn var(env: EnvPtr, arguments: Vec<Expr>, decl_kind: DeclKind) -> Result<Expr, String> {
    if let Expr::QExpr(symbols) = arguments[0].clone() {
        let values = &arguments[1..];
        if symbols.len() != values.len() {
            return Err(format!("the amount of symbols being defined ({}) must be equal to the \
                                number of values ({}).\nSymbols: {:#?}\nValues: {:#?}",
                               symbols.len(),
                               values.len(),
                               symbols,
                               values));
        }

        for (i, maybe_symbol) in symbols.to_vec().into_iter().enumerate() {
            if let Expr::Symbol(s) = maybe_symbol {
                match decl_kind {
                    DeclKind::Global => env.define_global(s, values[i].clone()),
                    DeclKind::Local => env.define_local(s, values[i].clone()),
                }
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

fn builtin_def(env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
    var(env, arguments, DeclKind::Global)
}

fn builtin_local_def(env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
    var(env, arguments, DeclKind::Local)
}


fn builtin_lambda(env_ptr: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
    if arguments.len() != 2 {
        return Err(format!("expected 2 arguments, got {}", arguments.len()));
    }

    if let Expr::QExpr(params) = arguments[0].clone() {
        if let Expr::QExpr(body) = arguments[1].clone() {
            debug!("builtin_lambda env: {:#?}", env_ptr);
            // Note: we're _cloning_ the parent environment here, not keeping a reference to it.
            let lambda = try!(Lambda::new(params, body, env_ptr.clone()));
            Ok(Expr::Function(Function::Lambda(lambda)))
        } else {
            return Err(format!("expected Q-expression as second argument, found {:?}",
                               arguments[1]));
        }
    } else {
        return Err(format!("expected Q-expression as first argument, found {:?}",
                           arguments[0]));
    }
}

fn builtin_if(env: EnvPtr, mut arguments: Vec<Expr>) -> Result<Expr, String> {
    if arguments.len() > 3 {
        return Err(format!("expected at most 3 arguments, got {}", arguments.len()));
    }

    if let Expr::Integer(test) = arguments.remove(0) {
        let then_expr: QExpr = match arguments.remove(0) {
            Expr::QExpr(q) => q,
            found => {
                return Err(format!("expected Q-expression as second argument, found {:?}",
                                   found))
            }
        };

        let else_expr_opt: Option<QExpr> = match arguments.get(0).cloned() { // XXX clone
            Some(Expr::QExpr(q)) => Some(q),
            Some(found) => {
                return Err(format!("expected Q-expression as second argument, found {:?}",
                                   found))
            }
            _ => None,
        };

        if test != 0 {
            eval_sexpr(env, then_expr.exprs)
        } else if let Some(q) = else_expr_opt {
            eval_sexpr(env, q.exprs)
        } else {
            // If no else expression was supplied and the condition is not true, we do nothing.
            Ok(Expr::SExpr(SExpr::empty()))
        }
    } else {
        return Err(format!("expected integer as first argument, found {:?}",
                           arguments[0]));
    }
}


fn builtin_less_than(env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
    ord(env, arguments, CmpKind::LessThan)
}

fn builtin_lesser_than_or_equal(env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
    ord(env, arguments, CmpKind::LessThanOrEqual)
}

fn builtin_greater_than(env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
    ord(env, arguments, CmpKind::GreaterThan)
}

fn builtin_greater_than_or_equal(env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
    ord(env, arguments, CmpKind::GreaterThanOrEqual)
}

fn builtin_is_eq(_env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
    let b = arguments[0] == arguments[1];
    Ok(Expr::Integer(b as i64))
}

fn builtin_is_not_eq(_env: EnvPtr, arguments: Vec<Expr>) -> Result<Expr, String> {
    let b = arguments[0] != arguments[1];
    Ok(Expr::Integer(b as i64))
}

// TODO: use bool instead of i64.
fn ord(_env: EnvPtr, mut arguments: Vec<Expr>, cmp_kind: CmpKind) -> Result<Expr, String> {
    let a = match arguments.remove(0) {
        Expr::Integer(x) => x,
        other => return Err(format!("expected integer as first argument, found {:?}", other)),
    };

    let b = match arguments.remove(0) {
        Expr::Integer(x) => x,
        other => return Err(format!("expected integer as first argument, found {:?}", other)),
    };

    let res = match cmp_kind {
        CmpKind::LessThan => a < b,
        CmpKind::LessThanOrEqual => a <= b,
        CmpKind::GreaterThan => a > b,
        CmpKind::GreaterThanOrEqual => a >= b,
    };

    Ok(Expr::Integer(res as i64))
}
