//! Reading several files at once.

use std::thread;

use crate::entry::Entry;
use crate::error::Error;
use crate::filter::{Filter, Stats};
use crate::input::{entries, open};

/// One input's kept entries, their stats, and its errors.
#[derive(Debug, Default)]
pub struct FileResult {
    pub path: String,
    pub entries: Vec<Entry>,
    pub stats: Stats,
    pub errors: Vec<Error>,
}

impl FileResult {
    /// Read every entry from `reader`, keeping those `filter` matches. `run`
    /// can use this for stdin too.
    pub fn read(path: &str, reader: impl std::io::BufRead, filter: &Filter) -> FileResult {
        todo!()
    }
}

/// Read every file, using up to `jobs` threads at a time. Results come back
/// in the order of `paths`, whichever thread finished first; a file that
/// won't open is a result holding that error.
pub fn process_files(paths: &[String], filter: &Filter, jobs: usize) -> Vec<FileResult> {
    todo!("thread::scope")
}
