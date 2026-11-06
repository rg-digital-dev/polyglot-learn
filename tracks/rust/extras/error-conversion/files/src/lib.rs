use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

/// Everything that can go wrong while reading a colour.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppError {
    /// A part isn't a whole number. Wraps the standard library's error.
    Parse(ParseIntError),
    /// A part is a number, but not in 0-255.
    Range { field: String, value: i64 },
    /// One of "red", "green" or "blue" has no part.
    Missing(String),
    /// More than three parts; holds how many there were.
    TooMany(usize),
}

/// Lets `?` turn a ParseIntError into an AppError::Parse.
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> AppError {
        todo!("implement From for {e:?}")
    }
}

impl fmt::Display for AppError {
    /// Messages:
    /// - Parse: "not a number: " followed by the ParseIntError's own message
    /// - Range: "`green` out of range: 300"
    /// - Missing: "missing `blue`"
    /// - TooMany: "expected 3 parts, got 4"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f;
        todo!("implement Display for {self:?}")
    }
}

impl Error for AppError {
    /// The error this one wraps: the ParseIntError for Parse, None otherwise.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!("implement source for {self:?}")
    }
}

/// Parse one whole number, ignoring surrounding whitespace. Use `?`, not
/// `map_err`: your From implementation does the conversion.
pub fn parse_field(text: &str) -> Result<i64, AppError> {
    todo!("implement parse_field({text:?})")
}

/// Read "red, green, blue": three comma-separated numbers in 0-255, checked in
/// that order. Errors: Missing for the first part that isn't there, TooMany for
/// more than three parts, and Parse or Range for a part that's wrong.
pub fn parse_rgb(text: &str) -> Result<(u8, u8, u8), AppError> {
    todo!("implement parse_rgb({text:?})")
}

/// Read a colour and return it as "#rrggbb" in lowercase hex. Any error comes
/// back as a `Box<dyn Error>`.
pub fn run(text: &str) -> Result<String, Box<dyn Error>> {
    todo!("implement run({text:?})")
}
