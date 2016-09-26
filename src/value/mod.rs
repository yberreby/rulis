mod sexpr;
mod qexpr;
pub use self::sexpr::*;
pub use self::qexpr::*;
use std::fmt;
use std::collections::HashMap;

pub type Env = HashMap<String, Expr>;

pub type InnerFunc = fn(&mut Env, &[Expr]) -> Result<Expr, String>;

pub enum Function {
    Builtin(InnerFunc),
    Lambda(Lambda),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lambda {
    local_env: Env,
    parameters: QExpr,
    body: QExpr,
}

impl Lambda {
    pub fn new(parameters: QExpr, body: QExpr) -> Lambda {
        Lambda {
            local_env: Env::new(),
            parameters: parameters,
            body: body,
        }
    }

    pub fn call(&mut self, env: &mut Env, arguments: &[Expr]) -> Result<Expr, String> {
        unimplemented!()
    }
}

impl Function {
    pub fn from_builtin(f: InnerFunc) -> Function {
        Function::Builtin(f)
    }

    pub fn call(&mut self, env: &mut Env, arguments: &[Expr]) -> Result<Expr, String> {
        match *self {
            Function::Builtin(f) => f(env, arguments),
            Function::Lambda(ref mut f) => f.call(env, arguments),
        }
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        match *self {
            Function::Builtin(f) => Function::Builtin(f),
            Function::Lambda(ref lambda) => Function::Lambda(lambda.clone()),
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Function) -> bool {
        // XXX: make sure this is correct.
        match *self {
            Function::Builtin(ref builtin) => {
                if let Function::Builtin(ref other_builtin) = *other {
                    *builtin as *const () == *other_builtin as *const ()
                } else {
                    false
                }
            }
            Function::Lambda(ref lambda) => {
                if let Function::Lambda(ref other_lambda) = *other {
                    lambda == other_lambda
                } else {
                    false
                }
            }
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Function::Builtin(f_ptr) => write!(f, "<a Rulis function @ {:p}>", f_ptr as *const ()),
            Function::Lambda(ref lambda) => write!(f, "lambda: {:?}", lambda),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Integer(i64),
    Symbol(String),
    SExpr(SExpr),
    QExpr(QExpr),
    Function(Function),
}

impl Expr {
    pub fn as_i64(&self) -> Option<i64> {
        if let Expr::Integer(i) = *self {
            Some(i)
        } else {
            None
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;

        match *self {
            Integer(i) => write!(f, "{}", i),
            Symbol(ref s) => write!(f, "{}", s),
            SExpr(ref sexpr) => write!(f, "{}", sexpr),
            QExpr(ref qexpr) => write!(f, "{}", qexpr),
            Function(ref func) => write!(f, "{:?}", func),
        }
    }
}
