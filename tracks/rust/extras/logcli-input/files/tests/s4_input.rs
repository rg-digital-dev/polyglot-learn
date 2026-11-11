// Stage 4 of the log CLI project. Don't edit this file: `learn` overwrites it.
// Your code goes in src/input.rs.

use std::io::{self, BufRead, Cursor};

use log_cli::entry::Level;
use log_cli::error::Error;
use log_cli::input::{entries, open};

/// Log lines that never end.
struct Endless;

impl io::Read for Endless {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let line = b"t INFO loop: again\n";
        let n = line.len().min(buf.len());
        buf[..n].copy_from_slice(&line[..n]);
        Ok(n)
    }
}

#[test]
fn c21_entries_come_in_order() {
    let text = "t1 INFO a: one\nt2 WARN b: two\nt3 ERROR c: three\n";
    let got: Vec<(Level, String)> = entries(Cursor::new(text)).map(|e| e.unwrap()).map(|e| (e.level, e.target)).collect();
    assert_eq!(got, [(Level::Info, "a".into()), (Level::Warn, "b".into()), (Level::Error, "c".into())]);
}

#[test]
fn c22_blank_lines_are_skipped_but_counted() {
    let text = "t1 INFO a: one\n\n   \nnot a line\nt5 INFO a: five";
    let got: Vec<Result<_, Error>> = entries(Cursor::new(text)).collect();
    assert_eq!(got.len(), 3, "{got:?}");
    match &got[1] {
        Err(Error::Parse { line, .. }) => assert_eq!(*line, 4, "the bad line is line 4"),
        other => panic!("expected a parse error, got {other:?}"),
    }
}

#[test]
fn c23_a_bad_line_does_not_stop_the_rest() {
    let text = "oops\nt2 INFO a: two\nt3 LOUD a: three\nt4 INFO a: four\n";
    let ok: Vec<bool> = entries(Cursor::new(text)).map(|e| e.is_ok()).collect();
    assert_eq!(ok, [false, true, false, true]);
}

#[test]
fn c24_entries_is_lazy() {
    let reader = io::BufReader::new(Endless);
    let first: Vec<_> = entries(reader).take(3).collect();
    assert_eq!(first.len(), 3, "taking 3 from an endless log must return");
    assert!(first.iter().all(|e| e.is_ok()));
}

#[test]
fn c25_open_reads_a_file_and_names_a_missing_one() {
    let path = std::env::temp_dir().join(format!("logq-s4-{}.log", std::process::id()));
    std::fs::write(&path, "t1 INFO a: one\n").unwrap();
    let mut reader = open(path.to_str().unwrap()).unwrap();
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    assert_eq!(line, "t1 INFO a: one\n");
    let _ = std::fs::remove_file(&path);

    let err = open("no/such/file.log").unwrap_err();
    assert!(matches!(&err, Error::Open { path, .. } if path == "no/such/file.log"), "{err:?}");
    assert_eq!(err.exit_code(), 66);
}
