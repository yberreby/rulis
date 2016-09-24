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

struct Lexer<'input> {
    input: &'input str,
}

impl<'input> Lexer<'input> {
    pub fn new(src: &'input str) -> Lexer<'input> {
        Lexer { input: src }
    }
}

struct Parser {
    input: Vec<Token>,
}
