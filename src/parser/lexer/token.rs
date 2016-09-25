#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenAndSpan {
    pub token: Token,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub value: Option<String>,
    pub kind: TKind,
}

/// Token Kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TKind {
    Symbol,
    LParen,
    RParen,
    LBrace,
    RBrace,
    HexLit,
    OctalLit,
    FloatLit,
    DecimalLit,
    StringLit,
    /// End Of File.
    Eof,
}
