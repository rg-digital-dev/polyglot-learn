//! One log line: `2026-11-11T08:00:00Z WARN  http: slow request (812 ms)`.

use std::fmt;
use std::str::FromStr;

/// How serious an entry is. Declared from least to most, so the derived
/// `PartialOrd` and `Ord` put them in that order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    /// Every level, least serious first.
    pub const ALL: [Level; 5] = [Level::Trace, Level::Debug, Level::Info, Level::Warn, Level::Error];

    /// The level as written in a log: `"WARN"`.
    pub fn as_str(&self) -> &'static str {
        todo!("match on self")
    }
}

impl FromStr for Level {
    type Err = ParseError;

    /// `"warn"`, `"Warn"` and `"WARN"` are all `Level::Warn`; anything else is
    /// `ParseError::BadLevel` holding the text.
    fn from_str(s: &str) -> Result<Level, ParseError> {
        todo!("find the level whose name matches s, ignoring case")
    }
}

/// One parsed line. `target` is the part before the colon, without it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub timestamp: String,
    pub level: Level,
    pub target: String,
    pub message: String,
}

/// Why a line isn't an entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Nothing but spaces.
    Empty,
    /// The line ended before this field: "level" or "target".
    MissingField(&'static str),
    /// Not one of the five levels.
    BadLevel(String),
    /// The target doesn't end in a colon, or is only one.
    BadTarget(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("a short message for each kind, naming the field, level or target")
    }
}

impl std::error::Error for ParseError {}

/// Parse one line: a timestamp, a level, a target ending in `:`, then the
/// message (which may be empty). Words are separated by any amount of space,
/// and space around the line or the message is trimmed.
pub fn parse_line(line: &str) -> Result<Entry, ParseError> {
    todo!()
}
