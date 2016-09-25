mod lexer;
mod error;

use self::error::*;
use self::lexer::{Token, TKind};
use std::iter::Peekable;
use ast;

pub fn parse(s: &str) -> Result<ast::SExpr, String> {
    unimplemented!()
}

// struct Parser<I: Iterator<Item = Token>> {
//     input: I,
// }
//
// impl<I: Iterator<Item = Token>> Parser<I> {
//     pub fn new<T: Into<I>>(x: T) -> Parser<I> {
//         Parser { input: x.into() }
//     }
// }


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

    fn eat_and_get(&mut self, expected: TKind) -> PResult<(Token)> {
        if self.token.kind != expected {
            // return Err(self.err(ErrorKind::unexpected_token(vec![expected], self.token.clone())));
            panic!("unexpected token")
        }
        Ok(self.bump_and_get())
    }
}
