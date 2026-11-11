// Stage 1 of the log CLI project. Don't edit this file: `learn` overwrites it.
// Your code goes in src/entry.rs.

use log_cli::entry::{parse_line, Entry, Level, ParseError};

#[test]
fn c01_parses_a_full_line() {
    let e = parse_line("2026-11-11T08:00:00Z WARN http: slow request (812 ms)").unwrap();
    assert_eq!(
        e,
        Entry {
            timestamp: "2026-11-11T08:00:00Z".into(),
            level: Level::Warn,
            target: "http".into(),
            message: "slow request (812 ms)".into(),
        }
    );
}

#[test]
fn c02_levels_parse_in_any_case() {
    assert_eq!("warn".parse::<Level>(), Ok(Level::Warn));
    assert_eq!("Error".parse::<Level>(), Ok(Level::Error));
    assert_eq!("TRACE".parse::<Level>(), Ok(Level::Trace));
    assert_eq!(Level::Info.as_str(), "INFO");
    assert_eq!(Level::Debug.as_str(), "DEBUG");
}

#[test]
fn c03_levels_are_ordered() {
    assert!(Level::Trace < Level::Debug);
    assert!(Level::Debug < Level::Info);
    assert!(Level::Info < Level::Warn);
    assert!(Level::Warn < Level::Error);
    let mut levels = vec![Level::Error, Level::Trace, Level::Warn];
    levels.sort();
    assert_eq!(levels, [Level::Trace, Level::Warn, Level::Error]);
}

#[test]
fn c04_extra_spaces_and_a_newline_are_fine() {
    let e = parse_line("  t1   info    db:   connected  \r\n").unwrap();
    assert_eq!((e.timestamp.as_str(), e.level, e.target.as_str(), e.message.as_str()), ("t1", Level::Info, "db", "connected"));
    let e = parse_line("t2 ERROR db:").unwrap();
    assert_eq!(e.message, "", "a message can be empty");
}

#[test]
fn c05_an_empty_line_is_an_error() {
    assert_eq!(parse_line(""), Err(ParseError::Empty));
    assert_eq!(parse_line("   \t "), Err(ParseError::Empty));
}

#[test]
fn c06_an_unknown_level_is_an_error_naming_it() {
    let err = parse_line("t1 LOUD http: hi").unwrap_err();
    assert_eq!(err, ParseError::BadLevel("LOUD".into()));
    assert!(err.to_string().contains("LOUD"), "the message {:?} should name the level", err.to_string());
    assert_eq!("".parse::<Level>(), Err(ParseError::BadLevel("".into())));
}

#[test]
fn c07_missing_fields_and_bad_targets_are_errors() {
    assert_eq!(parse_line("t1"), Err(ParseError::MissingField("level")));
    assert_eq!(parse_line("t1 INFO"), Err(ParseError::MissingField("target")));
    assert_eq!(parse_line("t1 INFO http hi"), Err(ParseError::BadTarget("http".into())));
    assert_eq!(parse_line("t1 INFO : hi"), Err(ParseError::BadTarget(":".into())));
    let msg = ParseError::MissingField("target").to_string();
    assert!(msg.contains("target"), "{msg:?} should say which field is missing");
}
