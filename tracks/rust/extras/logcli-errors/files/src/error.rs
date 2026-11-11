//! One error type for the whole program, and the exit code for each kind.

use std::fmt;
use std::io;

use crate::entry::{parse_line, Entry, ParseError};

#[derive(Debug)]
pub enum Error {
    /// A line that isn't an entry, with its 1-based number.
    Parse { line: usize, source: ParseError },
    /// A file that couldn't be opened.
    Open { path: String, source: io::Error },
    /// Reading or writing failed.
    Io(io::Error),
    /// The command line is wrong; the message says how.
    Usage(String),
}

impl Error {
    /// The process exit code for this error, as in BSD's sysexits.h:
    /// usage 2, parse 65 (bad data), open 66 (no input), io 74.
    pub fn exit_code(&self) -> i32 {
        todo!()
    }
}

impl fmt::Display for Error {
    /// `line 3: <the parse error>`, `can't open <path>: <the io error>`,
    /// `i/o error: <the io error>`, or a usage error's message as it is.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {
    /// The error inside, if there is one.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        todo!()
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        todo!()
    }
}

/// `parse_line`, with the line's number in the error.
pub fn parse_numbered(line: usize, text: &str) -> Result<Entry, Error> {
    todo!("parse_line, then map_err")
}

/// Read everything from `r`. Use `?` on the read: `From<io::Error>` turns its
/// error into `Error::Io`.
pub fn read_all(mut r: impl io::Read) -> Result<String, Error> {
    todo!()
}
