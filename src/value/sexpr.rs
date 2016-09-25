use std::fmt;
use std::ops::{Deref, DerefMut};
use super::Expr;

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
