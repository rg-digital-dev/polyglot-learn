//! The command line, parsed by hand.

use crate::entry::Level;
use crate::error::Error;

pub const USAGE: &str = "usage: logq [--level LEVEL] [--grep TEXT] [--format text|json] [--stats] [--jobs N] [FILE...]";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Args {
    /// Keep entries at this level or above.
    pub min_level: Option<Level>,
    /// Keep entries whose target or message contains this, in any case.
    pub grep: Option<String>,
    pub format: OutputFormat,
    /// Print a summary instead of the entries.
    pub stats: bool,
    /// How many files to read at once (1 unless --jobs says otherwise).
    pub jobs: usize,
    /// The files to read, in order; `-` is standard input. None given means `["-"]`.
    pub files: Vec<String>,
}

impl Args {
    /// Parse the arguments after the program name.
    ///
    /// Options: `--level LEVEL`, `--grep TEXT`, `--format text|json`,
    /// `--jobs N` (N > 0), each also as `--name=value`, and `--stats`.
    /// `-` and anything not starting with `-` is a file; after `--`
    /// everything is. Mistakes are `Error::Usage`, with a message that names
    /// the option or value and ends with a line holding USAGE.
    pub fn parse<I: IntoIterator<Item = String>>(args: I) -> Result<Args, Error> {
        todo!()
    }
}
