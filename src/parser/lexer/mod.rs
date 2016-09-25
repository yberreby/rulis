mod token;
pub use self::token::*;

use std::iter::Iterator;

/// Convenience function to collect all the tokens from a string.
pub fn lex(s: &str) -> Vec<TokenAndSpan> {
    let lexer = Lexer::new(s);
    lexer.collect()
}

/// A `Lexer` parses a source string into a list of tokens, which may later be used to construct an
/// Abstract Syntax Tree.
pub struct Lexer<'src> {
    /// Byte offset from the start of the source string.
    byte_offset: usize,
    /// The source string.
    src: &'src str,
    /// The last character to be read. Is `None` if we are at the end of the file.
    current_char: Option<char>,
}

impl<'src> Lexer<'src> {
    /// Create a new Lexer from the given source string.
    pub fn new(s: &str) -> Lexer {
        // Initialize the lexer with the first character of the source string.
        let first_char = s.chars().next();

        Lexer {
            src: s,
            byte_offset: 0,
            current_char: first_char,
        }
    }

    /// 'Eat' one character, setting `self.current_char` to `None` in case of EOF.
    ///
    /// Will panick if we are already at EOF.
    ///
    /// This is a _very_ hot function.
    fn bump(&mut self) {
        self.byte_offset += self.current_char.unwrap().len_utf8();

        if self.byte_offset < self.src.len() {
            let ch = char_at(&self.src, self.byte_offset);
            self.current_char = Some(ch);
        } else {
            self.current_char = None;
        }
    }

    /// Return the next character **without** bumping.
    /// Useful for lookahead.
    fn next_char(&self) -> Option<char> {
        let next_offset = self.byte_offset + 1;
        if next_offset < self.src.len() {
            let ch = char_at(&self.src, next_offset);
            Some(ch)
        } else {
            None
        }
    }

    /// Scan a number literal (integer or float).
    ///
    /// The literal "0" is considered octal, [as in
    /// C++](http://stackoverflow.com/a/6895543/2754323).
    fn scan_number(&mut self) -> Token {
        // Integer literal grammar:
        //
        // int_lit     = decimal_lit | octal_lit | hex_lit .
        // decimal_lit = ( "1" … "9" ) { decimal_digit } .
        // octal_lit   = "0" { octal_digit } .
        // hex_lit     = "0" ( "x" | "X" ) hex_digit { hex_digit } .

        let start = self.byte_offset;

        // If we have a hexadecimal, treat it specially.
        if self.current_char == Some('0') &&
           (self.next_char() == Some('x') || self.next_char() == Some('x')) {
            self.bump();
            self.bump();

            while let Some(c) = self.current_char {
                if c.is_digit(16) {
                    self.bump();
                } else {
                    break;
                }
            }

            return Token {
                value: Some(self.src[start..self.byte_offset].into()),
                kind: TKind::HexLit,
            };
        }

        let has_leading_zero = self.current_char == Some('0');
        let mut had_e = false;
        let mut had_dot = false;

        'outer: while let Some(c) = self.current_char {
            if c.is_digit(10) {
                self.bump();
            } else if !had_e && (c == 'e' || c == 'E') {
                self.bump();
                had_e = true;

                if self.current_char == Some('+') || self.current_char == Some('-') {
                    self.bump();
                }
            } else if !had_e && !had_dot && c == '.' {
                self.bump();
                had_dot = true;
            } else {
                break;
            }
        }

        let s = &self.src[start..self.byte_offset];

        let kind = if had_e || had_dot {
            TKind::FloatLit
        } else if has_leading_zero {
            TKind::OctalLit
        } else {
            TKind::DecimalLit
        };

        Token {
            value: Some(s.into()),
            kind: kind,
        }
    }

    /// Skip whitespace and comments, returning whether at least one newline was encountered.
    fn skip_whitespace_and_comments(&mut self) {
        if self.current_char == Some(';') {
            while let Some(c) = self.current_char {
                self.bump();
                if c == '\n' {
                    break;
                }
            }
        }
    }

    fn scan_ident(&mut self) -> &str {
        let start = self.byte_offset;

        if let Some(c) = self.current_char {
            if can_start_identifier(c) {
                self.bump();
            }
        }

        while let Some(c) = self.current_char {
            if can_continue_identifier(c) {
                self.bump();
            } else {
                break;
            }
        }

        &self.src[start..self.byte_offset]
    }

    fn scan_symbol(&mut self) -> Token {
        Token {
            value: Some(self.scan_ident().to_string()),
            kind: TKind::Symbol,
        }
    }

    /// Return the next token, if any.
    fn next_token_inner(&mut self) -> Option<Token> {
        // Whitespace and comment handling.
        self.skip_whitespace_and_comments();

        if self.current_char == Some(';') {
            while let Some(c) = self.current_char {
                self.bump();
                if c == '\n' {
                    break;
                }
            }
        }

        // Check for EOF after whitespace handling.
        let c = match self.current_char {
            Some(c) => c,
            None => return None,
        };

        let kind = match c {
            // Single-character tokens.
            '(' => {
                self.bump();
                TKind::LParen
            }
            ')' => {
                self.bump();
                TKind::RParen
            }
            '{' => {
                self.bump();
                TKind::LBrace
            }
            '}' => {
                self.bump();
                TKind::RBrace
            }
            // Scan integer.
            c if c.is_digit(10) => return Some(self.scan_number()),
            c if can_start_identifier(c) => return Some(self.scan_symbol()),
            // Start of _interpreted_ string literal.
            '"' => return Some(self.scan_string_lit()),
            c => panic!("unexpected start of token: '{}'", c),
        };

        Some(Token {
            kind: kind,
            value: None,
        })
    }

    fn scan_string_lit(&mut self) -> Token {
        self.bump();
        let start = self.byte_offset;

        while let Some(c) = self.current_char {
            // If we encounter a backslash escape, we just skip past the '\' and the
            // following character.
            if c == '\\' {
                self.bump();
                self.bump();
            } else if c == '"' {
                break;
            } else {
                self.bump();
            }
        }

        let s = &self.src[start..self.byte_offset];

        // Skip the quote _after_ slicing so that it isn't included
        // in the slice.
        self.bump();

        Token {
            // XXX(perf): alloc.
            value: Some(s.into()),
            kind: TKind::StringLit,
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = TokenAndSpan;

    fn next(&mut self) -> Option<TokenAndSpan> {
        let start = self.byte_offset as u32;
        let t = self.next_token_inner();

        t.map(|t| {
            TokenAndSpan {
                token: t,
                span: Span {
                    start: start,
                    end: self.byte_offset as u32,
                },
            }
        })
    }
}


fn is_allowed_symbol(c: char) -> bool {
    match c {
        '_' | '+' | '-' | '*' | '/' | '\\' | '=' | '<' | '>' | '!' | '&' => true,
        _ => false,
    }
}

fn can_start_identifier(c: char) -> bool {
    is_allowed_symbol(c) || c.is_alphabetic()
}

fn can_continue_identifier(c: char) -> bool {
    is_allowed_symbol(c) || c.is_alphabetic() || c.is_numeric()
}

fn char_at(s: &str, byte: usize) -> char {
    s[byte..].chars().next().unwrap()
}
