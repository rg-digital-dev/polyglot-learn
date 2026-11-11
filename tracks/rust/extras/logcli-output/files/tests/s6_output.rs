// Stage 6 of the log CLI project. Don't edit this file: `learn` overwrites it.
// Your code goes in src/output.rs, src/run.rs and src/main.rs.

use std::io::Cursor;
use std::process::Command;

use log_cli::args::OutputFormat;
use log_cli::entry::parse_line;
use log_cli::output::{formatter, json_string};
use log_cli::run::run;

const LOG: &str = "t1 INFO http: GET /\nt2 WARN http: slow \"search\"\nt3 ERROR db: lost\n";

fn args(a: &[&str]) -> Vec<String> {
    a.iter().map(|s| s.to_string()).collect()
}

/// Run with `stdin`, returning (code, stdout, stderr).
fn logq(a: &[&str], stdin: &str) -> (i32, String, String) {
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = run(args(a), Cursor::new(stdin.to_string()), &mut out, &mut err);
    (code, String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap())
}

#[test]
fn c32_json_string_escapes() {
    assert_eq!(json_string("plain"), "\"plain\"");
    assert_eq!(json_string("say \"hi\" \\ bye"), r#""say \"hi\" \\ bye""#);
    assert_eq!(json_string("a\nb\tc\r"), r#""a\nb\tc\r""#);
    assert_eq!(json_string("bell\u{7}"), r#""bell\u0007""#);
    assert_eq!(json_string("café ✓"), "\"café ✓\"", "other characters stay as they are");
}

#[test]
fn c33_text_prints_an_entry_like_a_log_line() {
    let mut out = Vec::new();
    formatter(OutputFormat::Text).entry(&mut out, &parse_line("t1 warn http:   slow").unwrap()).unwrap();
    assert_eq!(String::from_utf8(out).unwrap(), "t1 WARN  http: slow\n", "the level is padded to 5");
}

#[test]
fn c34_json_prints_one_object_per_line() {
    let mut out = Vec::new();
    let mut f = formatter(OutputFormat::Json);
    f.entry(&mut out, &parse_line("t1 INFO http: say \"hi\"").unwrap()).unwrap();
    f.entry(&mut out, &parse_line("t2 ERROR db: x").unwrap()).unwrap();
    assert_eq!(
        String::from_utf8(out).unwrap(),
        "{\"timestamp\":\"t1\",\"level\":\"INFO\",\"target\":\"http\",\"message\":\"say \\\"hi\\\"\"}\n\
         {\"timestamp\":\"t2\",\"level\":\"ERROR\",\"target\":\"db\",\"message\":\"x\"}\n"
    );
}

#[test]
fn c35_run_filters_stdin_to_stdout() {
    let (code, out, err) = logq(&["--level", "warn"], LOG);
    assert_eq!((code, err.as_str()), (0, ""));
    assert_eq!(out, "t2 WARN  http: slow \"search\"\nt3 ERROR db: lost\n");
}

#[test]
fn c36_run_prints_stats() {
    let (code, out, _) = logq(&["--stats"], LOG);
    assert_eq!(code, 0);
    assert_eq!(out, "3 entries\n  ERROR 1\n  WARN  1\n  INFO  1\ntop targets\n  http 2\n  db 1\n");
    let (_, out, _) = logq(&["--stats", "--format", "json"], LOG);
    assert_eq!(out, "{\"total\":3,\"levels\":{\"ERROR\":1,\"WARN\":1,\"INFO\":1},\"top_targets\":[[\"http\",2],[\"db\",1]]}\n");
}

#[test]
fn c37_bad_lines_are_reported_and_skipped() {
    let (code, out, err) = logq(&[], "t1 INFO a: one\nbroken\nt3 INFO a: three\n");
    assert_eq!(out, "t1 INFO  a: one\nt3 INFO  a: three\n", "good lines still print");
    assert!(err.contains("line 2"), "stderr {err:?} should say which line");
    assert_eq!(code, 65);
    let (code, _, err) = logq(&["--bogus"], "");
    assert_eq!(code, 2);
    assert!(err.starts_with("logq: unknown option --bogus"), "{err:?}");
}

#[test]
fn c38_the_logq_binary_runs() {
    let path = std::env::temp_dir().join(format!("logq-s6-{}.log", std::process::id()));
    std::fs::write(&path, LOG).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_logq")).args(["--grep", "db"]).arg(&path).output().unwrap();
    let _ = std::fs::remove_file(&path);
    assert_eq!(String::from_utf8_lossy(&out.stdout), "t3 ERROR db: lost\n", "stderr: {}", String::from_utf8_lossy(&out.stderr));
    assert_eq!(out.status.code(), Some(0));

    let out = Command::new(env!("CARGO_BIN_EXE_logq")).arg("no/such/file.log").output().unwrap();
    assert_eq!(out.status.code(), Some(66), "a missing file exits with 66");
    assert!(String::from_utf8_lossy(&out.stderr).contains("no/such/file.log"));
}
