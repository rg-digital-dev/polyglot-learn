//! Reading entries a line at a time.

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::entry::Entry;
use crate::error::{parse_numbered, Error};

/// Every entry in `reader`, lazily: nothing is read until the iterator is.
/// Blank lines are skipped but still counted, so errors carry the right line
/// number; a bad line is an `Err` item and reading goes on after it.
pub fn entries<R: BufRead>(reader: R) -> impl Iterator<Item = Result<Entry, Error>> {
    // Replace this with an iterator chain over reader.lines().
    let _ = reader;
    std::iter::from_fn(|| todo!())
}

/// Open a file for `entries`; a failure is `Error::Open` naming the path.
pub fn open(path: &str) -> Result<BufReader<File>, Error> {
    todo!()
}
