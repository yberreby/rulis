mod sexpr;
mod qexpr;
pub use self::sexpr::*;
pub use self::qexpr::*;

use std::fmt;


#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Integer(i64),
    Symbol(String),
    SExpr(SExpr),
    QExpr(QExpr),
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
        }
    }
}
