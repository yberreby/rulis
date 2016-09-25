use std::fmt;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct SExpr {
    pub exprs: Vec<Expr>,
}

impl SExpr {
    pub fn new(v: Vec<Expr>) -> SExpr {
        SExpr { exprs: v }
    }

    pub fn empty() -> SExpr {
        SExpr::new(vec![])
    }
}

impl Deref for SExpr {
    type Target = [Expr];

    fn deref(&self) -> &[Expr] {
        &self.exprs
    }
}

impl DerefMut for SExpr {
    fn deref_mut(&mut self) -> &mut [Expr] {
        &mut self.exprs
    }
}

impl fmt::Display for SExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "("));

        // Print with spaces in-between.
        let mut separator_prefix = "";
        for expr in &self.exprs {
            try!(write!(f, "{}{}", separator_prefix, expr));
            separator_prefix = " ";
        }
        try!(write!(f, ")"));
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Integer(i64),
    Symbol(String),
    SExpr(SExpr),
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
        }
    }
}
