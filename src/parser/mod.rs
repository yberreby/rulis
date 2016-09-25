mod lexer;
mod error;

use std::iter::Peekable;
use std::str::FromStr;

use ast;
use self::error::*;
use self::lexer::{Token, TKind};
use num::bigint::BigInt;
use num::BigRational;
use num::ToPrimitive;

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
            return Err(self.err(ErrorKind::unexpected_token(vec![expected], self.token.clone())));
        }
        self.bump();
        Ok(())
    }

    fn eat_and_get(&mut self, expected: TKind) -> PResult<Token> {
        if self.token.kind != expected {
            return Err(self.err(ErrorKind::unexpected_token(vec![expected], self.token.clone())));
        }
        Ok(self.bump_and_get())
    }

    /// Build a parse error.
    #[cold]
    #[inline(never)]
    fn err(&self, kind: ErrorKind) -> Error {
        Error { kind: kind }
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
        if self.token.kind.is_integer_literal() {
            let i = try!(self.parse_int_lit()).to_i64().unwrap();
            return Ok(ast::Expr::Integer(i));
        }

        match self.token.kind {
            TKind::FloatLit => {
                unimplemented!()
                // interpret float lit
            }
            TKind::Symbol => {
                let s = self.token.value.clone().unwrap();
                self.bump();
                Ok(ast::Expr::Symbol(s))
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
                return Err(self.err(ErrorKind::unexpected_token(vec![TKind::DecimalLit,
                                                                     TKind::OctalLit,
                                                                     TKind::HexLit],
                                                                self.token.clone())));
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

        for c in lit.chars() {
            if let Some(d) = c.to_digit(base) {
                res = res * BigInt::from(base);
                res = res + BigInt::from(d);
            } else {
                let msg = format!("invalid character in {}: {}", token_name, c);
                return Err(self.err(ErrorKind::other(msg)));
            }
        }

        Ok(res)
    }

    /// Interpret the value of a float/imaginary literal and return the result as a BigRational.
    /// If this method is being used to parse an imaginary lit, don't include the trailing `i`.
    fn interpret_float_lit(&mut self, value: &str, token_name: &str) -> PResult<BigRational> {
        // float_lit = decimals "." [ decimals ] [ exponent ] |
        //             decimals exponent |
        //             "." decimals [ exponent ] .
        // decimals  = decimal_digit { decimal_digit } .
        // exponent  = ( "e" | "E" ) [ "+" | "-" ] decimals .

        let mut res = BigRational::from_integer(BigInt::from(0u8));
        let mut chars = value.chars().peekable();
        let mut parse_exponent = false;
        let mut digits_after_dot = 0u32; // the number of digits after the dot we are

        while let Some(c) = chars.next() {
            if c == '.' {
                digits_after_dot = 1;
            } else if c == 'e' || c == 'E' {
                parse_exponent = true;
                break;
            } else {
                let digit = c.to_digit(10).expect("BUG: invalid char in float/imag lit");
                let digit_value = BigRational::from_integer(BigInt::from(digit));

                if digits_after_dot == 0 {
                    res = res * BigRational::from_integer(BigInt::from(10u8));
                    res = res + BigRational::from_integer(BigInt::from(digit));
                } else {
                    res = res +
                          digit_value /
                          BigRational::from_integer(BigInt::from(10u32.pow(digits_after_dot)));

                    digits_after_dot += 1;
                }
            }
        }

        if parse_exponent {
            let mut negative = false;
            if let Some(&c) = chars.peek() {
                if c == '+' {
                    chars.next();
                } else if c == '-' {
                    negative = true;
                    chars.next();
                }
            } else {
                // Empty exponent
                return Err(self.err(ErrorKind::other(format!("malformed {} exponent",
                                                             token_name))));
            }

            let mut exponent = 0;
            while let Some(c) = chars.next() {
                exponent *= 10;
                exponent += c.to_digit(10).expect("BUG: invalid char in float/imag lit exponent");
            }

            for _ in 0..exponent {
                if negative {
                    res = res / BigRational::from_integer(BigInt::from(10u8));
                } else {
                    res = res * BigRational::from_integer(BigInt::from(10u8));
                }
            }
        }

        Ok(res)
    }
}
