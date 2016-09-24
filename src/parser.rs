use ast;

pub fn parse(s: &str) -> Result<ast::SExpr, String> {
    unimplemented!()
}

struct Token {
    value: String,
    kind: TKind,
}

/// Token Kind.
enum TKind {
    Symbol,
    LParen,
    RParen,
}

struct Parser {
    input: Vec<Token>,
}
