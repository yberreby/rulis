extern crate rgo_token as token;

mod pos;

use std::iter::Iterator;
pub use self::pos::Position;
pub use self::token::*;

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
                kind: TokenKind::Hex,
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
            } else if c == 'i' {
                self.bump();

                return Token {
                    value: Some(self.src[start..self.byte_offset].into()),
                    kind: TokenKind::Imaginary,
                };
            } else {
                break;
            }
        }

        let s = &self.src[start..self.byte_offset];

        let kind = if had_e || had_dot {
            TokenKind::Float
        } else if has_leading_zero {
            TokenKind::Octal
        } else {
            TokenKind::Decimal
        };

        Token {
            value: Some(s.into()),
            kind: kind,
        }
    }

    /// Skip whitespace and comments, returning whether at least one newline was encountered.
    fn skip_whitespace_and_comments(&mut self) -> bool {
        let mut contains_newline = false;

        while let Some(c) = self.current_char {
            if c == '\n' {
                contains_newline = true;
            }

            // Are we at the start of a general comment (`/* ... */`)?
            if c == '/' && self.next_char() == Some('*') {
                // Skip the '/*'.
                self.bump();
                self.bump();

                // Skip the comment body.
                while let Some(c) = self.current_char {
                    if c == '*' && self.next_char() == Some('/') {
                        break;
                    } else {
                        self.bump();
                    }
                }

                // Skip the '*/'.
                self.bump();
                self.bump();

                // Resume whitespace skipping.
                continue;

            } else if c == '/' && self.next_char() == Some('/') {
                while let Some(c) = self.current_char {
                    if c == '\n' {
                        break;
                    } else {
                        self.bump();
                    }
                }

                // Resume whitespace skipping.
                // Since we have not bumped past the newline character,
                // the next iteration of the loop will catch it.
                continue;
            }

            if c.is_whitespace() {
                self.bump();
            } else {
                break;
            }
        }


        contains_newline
    }

    fn scan_ident(&mut self) -> &str {
        let start = self.byte_offset;

        while let Some(c) = self.current_char {
            if can_continue_identifier(c) {
                self.bump();
            } else {
                break;
            }
        }

        &self.src[start..self.byte_offset]
    }



    /// Return the next token, if any.
    fn next_token_inner(&mut self) -> Option<Token> {
        // Whitespace and comment handling.
        let contains_newline = self.skip_whitespace_and_comments();

        // Automatic semicolon insertion in the simplest case (newline + token that may terminate a
        // statement).
        //
        // The Go Spec also says that a semicolon may be omitted before a closing ")" or "}".
        // This case is _not_ handled by the lexer, but by the parser, as it requires too much
        // context.
        if contains_newline && may_terminate_statement(self.last_token_kind) {
            return Some(Token {
                kind: TokenKind::Semicolon,
                value: None,
            });
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
                TokenKind::LParen
            }
            ')' => {
                self.bump();
                TokenKind::RParen
            }
            '{' => {
                self.bump();
                TokenKind::LBrace
            }
            '}' => {
                self.bump();
                TokenKind::RBrace
            }
            '[' => {
                self.bump();
                TokenKind::LBracket
            }
            ']' => {
                self.bump();
                TokenKind::RBracket
            }
            ',' => {
                self.bump();
                TokenKind::Comma
            }
            ';' => {
                self.bump();
                TokenKind::Semicolon
            }
            // More complex tokens.
            '.' => {
                if self.next_char().map(|x| x.is_digit(10)) == Some(true) {
                    return Some(self.scan_number());
                }

                self.bump();

                // Look for an ellipsis ('...').
                if self.current_char == Some('.') && self.next_char() == Some('.') {
                    self.bump();
                    self.bump();
                    TokenKind::Ellipsis
                } else {
                    TokenKind::Dot
                }
            }
            ':' => {
                self.bump();

                if self.current_char == Some('=') {
                    self.bump();
                    TokenKind::ColonAssign
                } else {
                    TokenKind::Colon
                }
            }
            '=' => {
                self.bump();

                if self.current_char == Some('=') {
                    self.bump();
                    TokenKind::Equals
                } else {
                    TokenKind::Assign
                }
            }
            '+' => {
                self.bump();

                match self.current_char {
                    Some('+') => {
                        self.bump();
                        TokenKind::Increment
                    }
                    Some('=') => {
                        self.bump();
                        TokenKind::PlusAssign
                    }
                    _ => TokenKind::Plus,
                }
            }
            '-' => {
                self.bump();

                match self.current_char {
                    Some('-') => {
                        self.bump();
                        TokenKind::Decrement
                    }
                    Some('=') => {
                        self.bump();
                        TokenKind::MinusAssign
                    }
                    _ => TokenKind::Minus,
                }
            }
            '*' => {
                self.bump();

                match self.current_char {
                    Some('=') => {
                        self.bump();
                        TokenKind::StarAssign
                    }
                    _ => TokenKind::Star,
                }
            }
            '/' => {
                self.bump();

                match self.current_char {
                    Some('=') => {
                        self.bump();
                        TokenKind::SlashAssign
                    }
                    _ => TokenKind::Slash,
                }
            }
            '<' => {
                self.bump();

                match self.current_char {
                    Some('<') => {
                        self.bump();
                        match self.current_char {
                            Some('=') => {
                                self.bump();
                                TokenKind::LshiftAssign
                            }
                            _ => TokenKind::Lshift,
                        }
                    }
                    Some('=') => {
                        self.bump();
                        TokenKind::LessThanOrEqual
                    }
                    Some('-') => {
                        self.bump();
                        TokenKind::Arrow
                    }
                    _ => TokenKind::LessThan,
                }
            }
            '>' => {
                self.bump();

                match self.current_char {
                    Some('>') => {
                        self.bump();
                        match self.current_char {
                            Some('=') => {
                                self.bump();
                                TokenKind::RshiftAssign
                            }
                            _ => TokenKind::Rshift,
                        }
                    }
                    Some('=') => {
                        self.bump();
                        TokenKind::GreaterThanOrEqual
                    }
                    _ => TokenKind::GreaterThan,
                }
            }
            '|' => {
                self.bump();

                match self.current_char {
                    Some('|') => {
                        self.bump();
                        TokenKind::OrOr
                    }
                    Some('=') => {
                        self.bump();
                        TokenKind::OrAssign
                    }
                    _ => TokenKind::Or,
                }
            }
            '&' => {
                self.bump();

                match self.current_char {
                    Some('&') => {
                        self.bump();
                        TokenKind::AndAnd
                    }
                    Some('=') => {
                        self.bump();
                        TokenKind::AndAssign
                    }
                    Some('^') => {
                        self.bump();
                        match self.current_char {
                            Some('=') => {
                                self.bump();
                                TokenKind::BitClearAssign
                            }
                            _ => TokenKind::BitClear,
                        }
                    }
                    _ => TokenKind::And,
                }
            }
            '!' => {
                self.bump();

                match self.current_char {
                    Some('=') => {
                        self.bump();
                        TokenKind::NotEqual
                    }
                    _ => TokenKind::Not,
                }
            }
            '^' => {
                self.bump();

                match self.current_char {
                    Some('=') => {
                        self.bump();
                        TokenKind::CaretAssign
                    }
                    _ => TokenKind::Caret,
                }
            }
            '%' => {
                self.bump();

                match self.current_char {
                    Some('=') => {
                        self.bump();
                        TokenKind::PercentAssign
                    }
                    _ => TokenKind::Percent,
                }
            }
            // Scan integer.
            c if c.is_digit(10) => return Some(self.scan_number()),
            c if can_start_identifier(c) => return Some(self.scan_ident_or_keyword()),
            // Start of _interpreted_ string literal.
            '"' => return Some(self.scan_interpreted_str_lit()),
            c => panic!("unexpected start of token: '{}'", c),
        };

        Some(Token {
            kind: kind,
            value: None,
        })
    }



    fn scan_interpreted_str_lit(&mut self) -> Token {
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
            kind: TokenKind::Str,
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = TokenAndSpan;

    fn next(&mut self) -> Option<TokenAndSpan> {
        let start = self.byte_offset as u32;
        let t = self.next_token_inner();
        self.last_token_kind = t.as_ref().map(|t| t.kind);

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

/// Convenience function to collect all the tokens from a string.
pub fn tokenize(s: &str) -> Vec<TokenAndSpan> {
    let lexer = Lexer::new(s);
    lexer.collect()
}


// =====
// Utility functions.
// =====

fn can_start_identifier(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn can_continue_identifier(c: char) -> bool {
    c.is_alphabetic() || c.is_numeric() || c == '_'
}

fn char_at(s: &str, byte: usize) -> char {
    s[byte..].chars().next().unwrap()
}


