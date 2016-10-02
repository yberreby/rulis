mod lexer;
mod error;

use std::iter::Peekable;

use value::*;
use self::error::*;
use self::lexer::{Token, TKind};
use num::bigint::BigInt;
use num::ToPrimitive;

pub fn parse(s: &str) -> Result<Expr, String> {
    let tokens = lexer::lex(s);
    let mut parser = try!(Parser::new(tokens.into_iter().map(|tas| tas.token))
        .map_err(|e| e.kind.to_string()));
    parser.parse_expr().map_err(|e| e.kind.to_string())
}

pub struct Parser<R: Iterator<Item = Token>> {
    /// Our source of tokens.
    /// Users can choose to read all the tokens up-front, or to read them lazily.
    reader: Peekable<R>,
    /// The current token.
    token: Token,
}

impl<R: Iterator<Item = Token>> Parser<R> {
    pub fn new(mut it: R) -> PResult<Parser<R>> {
        let first_tok = try!(it.next().ok_or_else(|| {
            Error {
                kind: ErrorKind::UnexpectedToken {
                    found: Token {
                        value: None,
                        kind: TKind::Eof,
                    },
                    expected: vec![],
                },
            }
        }));

        Ok(Parser {
            token: first_tok,
            reader: it.peekable(),
        })
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
            return Err(self.err(ErrorKind::unexpected_token(vec![expected], self.token.clone())));
        }
        self.bump();
        Ok(())
    }

    /// Build a parse error.
    #[cold]
    #[inline(never)]
    fn err(&self, kind: ErrorKind) -> Error {
        Error { kind: kind }
    }

    fn parse_sexpr(&mut self) -> PResult<SExpr> {
        try!(self.eat(TKind::LParen));

        let mut exprs = Vec::new();
        while self.token.kind != TKind::RParen {
            let expr = try!(self.parse_expr());
            exprs.push(expr);
        }

        try!(self.eat(TKind::RParen));

        Ok(SExpr::new(exprs))
    }

    fn parse_qexpr(&mut self) -> PResult<QExpr> {
        try!(self.eat(TKind::LBrace));

        let mut exprs = Vec::new();
        while self.token.kind != TKind::RBrace {
            let expr = try!(self.parse_expr());
            exprs.push(expr);
        }

        try!(self.eat(TKind::RBrace));

        Ok(QExpr::new(exprs))
    }

    fn parse_expr(&mut self) -> PResult<Expr> {
        let res = match self.token.kind {
            k if k.can_start_atom() => try!(self.parse_atom()),
            TKind::LParen => Expr::SExpr(try!(self.parse_sexpr())),
            TKind::LBrace => Expr::QExpr(try!(self.parse_qexpr())),
            _ => {
                return Err(self.err(ErrorKind::unexpected_token(vec![], self.token.clone())));
            }
        };

        Ok(res)
    }

    // Atoms = numbers, string literals, symbols...
    fn parse_atom(&mut self) -> PResult<Expr> {
        if self.token.kind.is_integer_literal() {
            let i = try!(self.parse_int_lit()).to_i64().unwrap();
            return Ok(Expr::Integer(i));
        }

        match self.token.kind {
            TKind::FloatLit => {
                unimplemented!()
                // interpret float lit
            }
            TKind::Symbol => {
                let s = self.token.value.clone().unwrap();
                self.bump();
                Ok(Expr::Symbol(s))
            }
            _ => panic!("unknown token: {:?}", self.token.kind),
        }
    }

    fn parse_int_lit(&mut self) -> PResult<BigInt> {
        // int_lit     = decimal_lit | octal_lit | hex_lit .
        // decimal_lit = ( "1" … "9" ) { decimal_digit } .
        // octal_lit   = "0" { octal_digit } .
        // hex_lit     = "0" ( "x" | "X" ) hex_digit { hex_digit } .

        match self.token.kind {
            TKind::DecimalLit => {
                let value = self.bump_and_get().value.expect("BUG: missing value in decimal lit");
                Ok(try!(self.interpret_int(&value[..], 10, "decimal literal")))
            }
            TKind::OctalLit => {
                let value = self.bump_and_get().value.expect("BUG: missing value in octal lit");
                assert_eq!(value.chars().next(), Some('0'));
                Ok(try!(self.interpret_int(&value[1..], 8, "octal literal")))
            }
            TKind::HexLit => {
                let value = self.bump_and_get().value.expect("BUG: missing value in hex lit");
                assert!(value.starts_with("0x") || value.starts_with("0X"));
                Ok(try!(self.interpret_int(&value[2..], 16, "hex literal")))
            }
            _ => {
                Err(self.err(ErrorKind::unexpected_token(vec![TKind::DecimalLit,
                                                              TKind::OctalLit,
                                                              TKind::HexLit],
                                                         self.token.clone())))
            }
        }
    }


    /// Interpret the value of an int literal and return the result as a BigInt, using the provided
    /// base.
    ///
    /// Use `token_name` to specify what type of literal this is, for error messages. To
    /// parse an octal or hex literal, do not pass the `0` or `0x` prefixes.
    fn interpret_int(&mut self, lit: &str, base: u32, token_name: &str) -> PResult<BigInt> {
        let mut res = BigInt::from(0u8);
        let mut chars = lit.chars();
        let mut negative = false;

        if lit.chars().next() == Some('-') {
            negative = true;
            chars.next().unwrap();
        }

        for c in chars {
            if let Some(d) = c.to_digit(base) {
                res = res * BigInt::from(base);
                res = res + BigInt::from(d);
            } else {
                let msg = format!("invalid character in {}: {}", token_name, c);
                return Err(self.err(ErrorKind::other(msg)));
            }
        }

        if negative {
            res = -res;
        }

        Ok(res)
    }
}
