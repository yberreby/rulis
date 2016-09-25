use std::fmt;
use std::ops::{Deref, DerefMut};
use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct QExpr {
    pub exprs: Vec<Expr>,
}

impl QExpr {
    pub fn new(v: Vec<Expr>) -> QExpr {
        QExpr { exprs: v }
    }

    pub fn empty() -> QExpr {
        QExpr::new(vec![])
    }
}

impl Deref for QExpr {
    type Target = [Expr];

    fn deref(&self) -> &[Expr] {
        &self.exprs
    }
}

impl DerefMut for QExpr {
    fn deref_mut(&mut self) -> &mut [Expr] {
        &mut self.exprs
    }
}

impl fmt::Display for QExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // We are writing '{{' because the '{' character has a special meaning for Rust's
        // formatting machinery, and has to be escaped.
        try!(write!(f, r"{{"));

        // Print with spaces in-between.
        let mut separator_prefix = "";
        for expr in &self.exprs {
            try!(write!(f, "{}{}", separator_prefix, expr));
            separator_prefix = " ";
        }
        try!(write!(f, r"}}"));
        Ok(())
    }
}
