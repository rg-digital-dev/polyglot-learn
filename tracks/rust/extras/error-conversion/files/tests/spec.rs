// Part of the Rust in Practice module (tracks/rust/extras/error-conversion).
// Don't edit this file: `learn` overwrites it. Your code goes in src/lib.rs.

use error_conversion::*;
use std::error::Error;
use std::num::ParseIntError;

fn int_error(text: &str) -> ParseIntError {
    text.parse::<i64>().unwrap_err()
}

#[test]
fn c01_from_turns_a_parse_error_into_an_app_error() {
    let e: AppError = int_error("x").into();
    assert_eq!(e, AppError::Parse(int_error("x")));
    assert_eq!(AppError::from(int_error("")), AppError::Parse(int_error("")));
}

#[test]
fn c02_parse_field_converts_errors_with_the_question_mark() {
    assert_eq!(parse_field(" 42 "), Ok(42));
    assert_eq!(parse_field("-7"), Ok(-7));
    assert_eq!(parse_field("4x"), Err(AppError::Parse(int_error("4x"))));
}

#[test]
fn c03_parse_rgb_reads_three_numbers() {
    assert_eq!(parse_rgb("12, 34, 56"), Ok((12, 34, 56)));
    assert_eq!(parse_rgb("0,0,255"), Ok((0, 0, 255)));
}

#[test]
fn c04_parse_rgb_reports_missing_and_extra_parts() {
    assert_eq!(parse_rgb("1, 2"), Err(AppError::Missing("blue".into())));
    assert_eq!(parse_rgb("1, 2, 3, 4"), Err(AppError::TooMany(4)));
}

#[test]
fn c05_parse_rgb_reports_a_value_out_of_range() {
    assert_eq!(parse_rgb("1, 300, 2"), Err(AppError::Range { field: "green".into(), value: 300 }));
    assert_eq!(parse_rgb("-1, 0, 0"), Err(AppError::Range { field: "red".into(), value: -1 }));
}

#[test]
fn c06_parse_rgb_passes_parse_errors_up() {
    assert_eq!(parse_rgb("1, x, 2"), Err(AppError::Parse(int_error("x"))));
    assert_eq!(parse_rgb("1,,2"), Err(AppError::Parse(int_error(""))));
}

#[test]
fn c07_errors_display_readable_messages() {
    let parse = AppError::Parse(int_error("x"));
    assert_eq!(parse.to_string(), format!("not a number: {}", int_error("x")));
    assert_eq!(AppError::Range { field: "green".into(), value: 300 }.to_string(), "`green` out of range: 300");
    assert_eq!(AppError::Missing("blue".into()).to_string(), "missing `blue`");
    assert_eq!(AppError::TooMany(4).to_string(), "expected 3 parts, got 4");
}

#[test]
fn c08_source_returns_the_underlying_parse_error() {
    let e = AppError::Parse(int_error("x"));
    let source = e.source().expect("Parse should have a source");
    assert_eq!(source.downcast_ref::<ParseIntError>(), Some(&int_error("x")));
    assert!(AppError::Missing("red".into()).source().is_none());
    assert!(AppError::TooMany(5).source().is_none());
}

#[test]
fn c09_run_formats_a_hex_colour() {
    assert_eq!(run("255, 128, 0").unwrap(), "#ff8000");
    assert_eq!(run("1,2,3").unwrap(), "#010203");
}

#[test]
fn c10_run_returns_app_errors_in_a_box() {
    let e = run("1, 2").unwrap_err();
    assert_eq!(e.to_string(), "missing `blue`");
    assert_eq!(e.downcast_ref::<AppError>(), Some(&AppError::Missing("blue".into())));
}
