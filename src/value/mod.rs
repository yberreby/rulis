mod sexpr;
mod qexpr;
pub use self::sexpr::*;
pub use self::qexpr::*;
use std::fmt;
use std::collections::HashMap;

pub type Env = HashMap<String, Expr>;

pub type InnerFunc = fn(&mut Env, &[Expr]) -> Result<Expr, String>;

// FIXME: clearly separate AST and interpreter data structures clearly.
// For now, they live together.
// XXX: shall we keep this Result<T, E>?
#[derive(Copy)]
pub struct Function {
    f: InnerFunc,
}

impl Function {
    pub fn new(f: InnerFunc) -> Function {
        Function { f: f }
    }

    pub fn call(&self, env: &mut Env, arguments: &[Expr]) -> Result<Expr, String> {
        (self.f)(env, arguments)
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Function { f: self.f }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Function) -> bool {
        // XXX: make sure this is correct.
        self.f as *const () == other.f as *const ()
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<a Rulis function @ {}>", self.f as usize)
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
