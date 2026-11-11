//! Parsing without copying: every field borrows from the line.

use std::collections::BTreeMap;

use crate::entry::{Entry, Level, ParseError};

/// An entry whose text fields are slices of the line it came from. It can't
/// outlive that line: the `'a` says so.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntryRef<'a> {
    pub timestamp: &'a str,
    pub level: Level,
    pub target: &'a str,
    pub message: &'a str,
}

impl EntryRef<'_> {
    /// An owned copy, for when the entry has to outlive the line.
    pub fn to_entry(&self) -> Entry {
        todo!()
    }
}

/// `parse_line`, but every field is a slice of `line`: the same rules and the
/// same errors, and no `String` unless it fails.
pub fn parse_ref(line: &str) -> Result<EntryRef<'_>, ParseError> {
    todo!()
}

/// Entries per level in `text`, and how many non-blank lines didn't parse.
pub fn count_levels(text: &str) -> (BTreeMap<Level, usize>, usize) {
    todo!()
}

/// The targets of entries at `level`, each once, in the order first seen.
pub fn targets_at(text: &str, level: Level) -> Vec<&str> {
    todo!()
}
