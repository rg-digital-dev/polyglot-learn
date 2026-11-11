//! The whole program, with its input and output passed in so tests can run it.

use std::io::{BufRead, Write};

/// Run logq with `args` (without the program name) and return the exit code.
///
/// - A usage error: print `logq: <error>` on `err` and return its code.
/// - Read each file in order (`-` is `stdin`), keep the entries the filter
///   matches, and print them with the chosen format; with `--stats`, print
///   only the summary of all of them at the end.
/// - A bad line or a file that won't open: print `logq: <file>: <error>` (or
///   `logq: <error>` for an open error, which names the file) on `err`, skip
///   it and go on. The exit code is then the first error's, else 0.
pub fn run(args: Vec<String>, stdin: impl BufRead, out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    todo!()
}
