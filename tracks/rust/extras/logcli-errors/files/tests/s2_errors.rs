// Stage 2 of the log CLI project. Don't edit this file: `learn` overwrites it.
// Your code goes in src/error.rs.

use std::error::Error as _;
use std::io;

use log_cli::entry::{Level, ParseError};
use log_cli::error::{parse_numbered, read_all, Error};

/// A reader that always fails.
struct Broken;

impl io::Read for Broken {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::other("disk on fire"))
    }
}

#[test]
fn c08_a_parse_error_says_which_line() {
    let err = parse_numbered(3, "t1 LOUD http: hi").unwrap_err();
    assert!(matches!(err, Error::Parse { line: 3, source: ParseError::BadLevel(_) }), "{err:?}");
    let msg = err.to_string();
    assert!(msg.starts_with("line 3: "), "{msg:?} should start with \"line 3: \"");
    assert!(msg.contains("LOUD"), "{msg:?} should include the parse error");
}

#[test]
fn c09_good_lines_pass_through() {
    let e = parse_numbered(1, "t1 INFO db: up").unwrap();
    assert_eq!((e.level, e.target.as_str()), (Level::Info, "db"));
}

#[test]
fn c10_io_errors_convert_with_the_question_mark() {
    assert_eq!(read_all("hello".as_bytes()).unwrap(), "hello");
    let err = read_all(Broken).unwrap_err();
    assert!(matches!(err, Error::Io(_)), "{err:?}");
    assert!(err.to_string().contains("disk on fire"), "{:?}", err.to_string());
    let from: Error = io::Error::other("x").into();
    assert!(matches!(from, Error::Io(_)));
}

#[test]
fn c11_open_errors_name_the_file() {
    let err = Error::Open { path: "missing.log".into(), source: io::Error::from(io::ErrorKind::NotFound) };
    assert!(err.to_string().contains("missing.log"), "{:?}", err.to_string());
}

#[test]
fn c12_exit_codes_follow_sysexits() {
    let codes = [
        Error::Usage("bad".into()).exit_code(),
        parse_numbered(1, "").unwrap_err().exit_code(),
        Error::Open { path: "f".into(), source: io::Error::from(io::ErrorKind::NotFound) }.exit_code(),
        Error::Io(io::Error::other("x")).exit_code(),
    ];
    assert_eq!(codes, [2, 65, 66, 74], "usage, parse, open, io");
    assert_eq!(Error::Usage("say this".into()).to_string(), "say this");
}

#[test]
fn c13_source_points_at_the_cause() {
    let err = parse_numbered(2, "t1").unwrap_err();
    let cause = err.source().expect("a parse error has a source");
    assert_eq!(cause.downcast_ref::<ParseError>(), Some(&ParseError::MissingField("level")));
    let err = Error::Io(io::Error::other("x"));
    assert!(err.source().unwrap().downcast_ref::<io::Error>().is_some());
    assert!(Error::Usage("u".into()).source().is_none());
}
