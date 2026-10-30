//! A single user-facing error type. Every message should say what went wrong
//! and, where possible, what to do next.

use std::fmt;

#[derive(Debug)]
pub struct Error(pub String);

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

/// Build an error from a format string: `bail!("no spec for {slug}")`.
#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => { return Err($crate::error::Error(format!($($arg)*))) };
}

/// Attach a human-readable context to any error.
pub trait Context<T> {
    fn context(self, msg: impl FnOnce() -> String) -> Result<T>;
}

impl<T, E: fmt::Display> Context<T> for std::result::Result<T, E> {
    fn context(self, msg: impl FnOnce() -> String) -> Result<T> {
        self.map_err(|e| Error(format!("{}: {e}", msg())))
    }
}
