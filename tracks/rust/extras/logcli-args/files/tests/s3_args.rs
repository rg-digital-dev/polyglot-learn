// Stage 3 of the log CLI project. Don't edit this file: `learn` overwrites it.
// Your code goes in src/args.rs.

use log_cli::args::{Args, OutputFormat};
use log_cli::entry::Level;
use log_cli::error::Error;

fn parse(args: &[&str]) -> Result<Args, Error> {
    Args::parse(args.iter().map(|a| a.to_string()))
}

fn usage_error(args: &[&str]) -> String {
    match parse(args) {
        Err(e @ Error::Usage(_)) => {
            assert_eq!(e.exit_code(), 2);
            e.to_string()
        }
        other => panic!("{args:?} should be a usage error, got {other:?}"),
    }
}

#[test]
fn c14_no_arguments_reads_stdin_as_text() {
    let a = parse(&[]).unwrap();
    assert_eq!(a.files, ["-"]);
    assert_eq!((a.min_level, a.grep.as_deref(), a.format, a.stats, a.jobs), (None, None, OutputFormat::Text, false, 1));
}

#[test]
fn c15_level_and_grep_take_values() {
    let a = parse(&["--level", "warn", "--grep", "timeout"]).unwrap();
    assert_eq!((a.min_level, a.grep.as_deref()), (Some(Level::Warn), Some("timeout")));
    let a = parse(&["--level=ERROR", "--grep=a b"]).unwrap();
    assert_eq!((a.min_level, a.grep.as_deref()), (Some(Level::Error), Some("a b")), "--name=value works too");
}

#[test]
fn c16_format_and_stats() {
    let a = parse(&["--format", "json", "--stats"]).unwrap();
    assert_eq!((a.format, a.stats), (OutputFormat::Json, true));
    assert_eq!(parse(&["--format=text"]).unwrap().format, OutputFormat::Text);
}

#[test]
fn c17_files_keep_their_order() {
    let a = parse(&["b.log", "--level", "info", "-", "a.log"]).unwrap();
    assert_eq!(a.files, ["b.log", "-", "a.log"]);
    let a = parse(&["--", "--stats", "-x.log"]).unwrap();
    assert_eq!(a.files, ["--stats", "-x.log"], "after -- everything is a file");
    assert!(!a.stats);
}

#[test]
fn c18_jobs_takes_a_positive_number() {
    assert_eq!(parse(&["--jobs", "4"]).unwrap().jobs, 4);
    assert!(usage_error(&["--jobs", "0"]).contains("--jobs"));
    assert!(usage_error(&["--jobs=many"]).contains("many"));
}

#[test]
fn c19_an_unknown_option_is_a_usage_error_naming_it() {
    let msg = usage_error(&["--colour"]);
    assert!(msg.contains("--colour"), "{msg:?} should name the option");
    assert!(msg.contains("usage: logq"), "{msg:?} should include the usage line");
}

#[test]
fn c20_a_missing_or_bad_value_is_a_usage_error() {
    assert!(usage_error(&["--level"]).contains("--level"));
    assert!(usage_error(&["--level", "loud"]).contains("loud"));
    assert!(usage_error(&["--format", "xml"]).contains("xml"));
    assert!(usage_error(&["--grep"]).contains("--grep"));
}
