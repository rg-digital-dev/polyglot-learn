//! Printing entries and stats, as text or JSON.

use std::io::{self, Write};

use crate::args::OutputFormat;
use crate::entry::Entry;
use crate::filter::Stats;

/// One way of printing. `dyn Write` lets the same code print to stdout, a
/// file or a `Vec<u8>` in a test.
pub trait Format {
    fn entry(&mut self, out: &mut dyn Write, e: &Entry) -> io::Result<()>;
    fn stats(&mut self, out: &mut dyn Write, s: &Stats) -> io::Result<()>;
}

/// `t1 WARN  http: slow`: the level padded to 5 characters.
///
/// Stats: `3 entries`, then one `  LEVEL n` line per level that has entries
/// (most serious first, the level padded to 5), then `top targets` and up to
/// five `  target n` lines.
pub struct Text;

/// One JSON object per line: `{"timestamp":…,"level":…,"target":…,"message":…}`.
///
/// Stats: `{"total":3,"levels":{"ERROR":1,…},"top_targets":[["http",2],…]}`,
/// levels most serious first, up to five targets.
pub struct Json;

pub fn formatter(f: OutputFormat) -> Box<dyn Format> {
    todo!()
}

/// `s` as a JSON string, quotes included: `"`, `\`, newline, return and tab
/// are escaped as `\"`, `\\`, `\n`, `\r`, `\t`; other characters below
/// U+0020 as `\u00XX` (lower-case hex).
pub fn json_string(s: &str) -> String {
    todo!()
}

impl Format for Text {
    fn entry(&mut self, out: &mut dyn Write, e: &Entry) -> io::Result<()> {
        todo!("writeln!(out, …)")
    }

    fn stats(&mut self, out: &mut dyn Write, s: &Stats) -> io::Result<()> {
        todo!()
    }
}

impl Format for Json {
    fn entry(&mut self, out: &mut dyn Write, e: &Entry) -> io::Result<()> {
        todo!()
    }

    fn stats(&mut self, out: &mut dyn Write, s: &Stats) -> io::Result<()> {
        todo!()
    }
}
