mod lexer;
mod error;

use self::error::*;
use self::lexer::{Token, TKind};
use std::iter::Peekable;
use std::str::FromStr;
use ast;

pub fn parse(s: &str) -> Result<ast::SExpr, String> {
    let tokens = lexer::lex(s);
    let mut parser = Parser::new(tokens.into_iter().map(|tas| tas.token));
    parser.parse_sexpr().map_err(|e| e.kind.to_string())
}

pub struct Parser<R: Iterator<Item = Token>> {
    /// Our source of tokens.
    /// Users can choose to read all the tokens up-front, or to read them lazily.
    reader: Peekable<R>,
    /// The current token.
    token: Token,
    /// (XXX) Seems to indicate the current level of nesting when parsing expressions.
    expression_level: u32,
}

impl<R: Iterator<Item = Token>> Parser<R> {
    pub fn new(mut it: R) -> Parser<R> {
        // TODO: handle missing tok gracefully.
        let first_tok = it.next().expect("missing first token");
        Parser {
            token: first_tok,
            reader: it.peekable(),
            // In the Go code, the initial value of `exprLev` implicitly defaults to 0.
            expression_level: 0,
        }
    }

    /// Advance the parser by one token.
    fn bump(&mut self) {
        let next = self.reader.next();

        if let Some(t) = next {
            self.token = t;
        } else {
            // XXX what span to set?
            self.token = Token {
                kind: TKind::Eof,
                value: None,
            };
        }
    }

    /// Advance the parser by one token and return the bumped token.
    fn bump_and_get(&mut self) -> Token {
        // XXX(perf): clone; cloning is needed to let bump() see the previous token.
        let old_token = self.token.clone();
        self.bump();
        old_token
    }

    /// Consume the next token, asserting its kind is equal to `expected`.
    fn eat(&mut self, expected: TKind) -> PResult<()> {
        if self.token.kind != expected {
            // return Err(self.err(ErrorKind::unexpected_token(vec![expected], self.token.clone())));
            panic!("unexpected token")
        }
        self.bump();
        Ok(())
    }

    fn eat_and_get(&mut self, expected: TKind) -> PResult<Token> {
        if self.token.kind != expected {
            // return Err(self.err(ErrorKind::unexpected_token(vec![expected], self.token.clone())));
            panic!("unexpected token")
        }
        Ok(self.bump_and_get())
    }

    fn parse_sexpr(&mut self) -> PResult<ast::SExpr> {
        try!(self.eat(TKind::LParen));

        let mut exprs = Vec::new();
        while self.token.kind != TKind::RParen {
            let expr = try!(self.parse_expr());
            exprs.push(expr);
        }

        try!(self.eat(TKind::RParen));

        Ok(ast::SExpr::new(exprs))
    }

    fn parse_expr(&mut self) -> PResult<ast::Expr> {
        let res = match self.token.kind {
            k if k.can_start_atom() => try!(self.parse_atom()),
            TKind::LParen => ast::Expr::SExpr(try!(self.parse_sexpr())),
            _ => unimplemented!(),
        };

        Ok(res)
    }

    // Atoms = numbers, string literals, symbols...
    fn parse_atom(&mut self) -> PResult<ast::Expr> {
        match self.token.kind {
            TKind::DecimalLit => {
                let i = i64::from_str(&self.token.value.as_ref().unwrap()).unwrap();
                self.bump();
                Ok(ast::Expr::Integer(i))
            }
            TKind::Symbol => {
                let s = self.token.value.clone().unwrap();
                self.bump();
                Ok(ast::Expr::Symbol(s))
            }
            _ => unimplemented!(),
        }
    }
}
