#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Integer(i64),
    Operation(Operation),
}


#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub operator: Operator,
    pub operands: Vec<Expr>,
}
