use std::fmt;
use super::lexer::{Token, TKind};

pub type PResult<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub kind: ErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    UnexpectedToken { found: Token, expected: Vec<TKind> },
    Other { msg: String },
}

impl ErrorKind {
    pub fn unexpected_token(expected: Vec<TKind>, found: Token) -> ErrorKind {
        ErrorKind::UnexpectedToken {
            found: found,
            expected: expected,
        }
    }

    // XXX: potential code bloat due to monomorphisation, unless inlined. But we don't want to
    // inline cold functions... so it could be best to remove all generics here and
    // #[inline(never)].
    pub fn other<T: Into<String>>(msg: T) -> ErrorKind {
        ErrorKind::Other { msg: msg.into() }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::UnexpectedToken { ref found, ref expected } => {
                if found.kind == TKind::Eof {
                    try!(write!(f, "unexpected end of input"));
                    if !expected.is_empty() {
                        try!(write!(f, ", "));
                        try!(print_expected(f, &expected));
                    }
                    return Ok(());
                }

                if expected.is_empty() {
                    return write!(f, "unexpected token \"{:?}\"", found);
                }

                try!(print_expected(f, &expected));
                write!(f, "found \"{:?}\"", found)
            }
            ErrorKind::Other { ref msg } => write!(f, "{}", msg),
        }
    }
}

fn print_expected(f: &mut fmt::Formatter, expected: &[TKind]) -> fmt::Result {
    try!(write!(f, "expected "));

    if expected.len() > 2 {
        try!(write!(f, "one of "));

        let mut sep = " ";
        for tk in expected {
            try!(write!(f, "\"{:?}\"{}", tk, sep));
            sep = ", ";
        }
    } else if expected.len() == 1 {
        try!(write!(f, "\"{:?}\" ", expected[0]));
    }

    Ok(())
}
