pub type SExpr = Vec<Expr>;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Integer(i64),
    Symbol(String),
    SExpr(SExpr),
}
