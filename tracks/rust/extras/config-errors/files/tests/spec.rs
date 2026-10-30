// Part of the Deeper Rust module (tracks/rust/extras/config-errors). Don't
// edit this file: `learn` overwrites it. Your code goes in src/lib.rs.

use config_errors::*;

fn pair(k: &str, v: &str) -> (String, String) {
    (k.to_string(), v.to_string())
}

#[test]
fn c01_parse_pairs_reads_key_value_lines() {
    let pairs = parse_pairs("name = demo\nport=8080\n").unwrap();
    assert_eq!(pairs, vec![pair("name", "demo"), pair("port", "8080")]);
}

#[test]
fn c02_parse_pairs_skips_blank_lines_and_comments() {
    let pairs = parse_pairs("# settings\n\n  name = demo  \n   \n# end\n").unwrap();
    assert_eq!(pairs, vec![pair("name", "demo")]);
}

#[test]
fn c03_parse_pairs_reports_the_bad_line_number() {
    assert_eq!(parse_pairs("name = demo\n\nport 8080\n"), Err(ConfigError::BadLine(3)));
}

#[test]
fn c04_get_finds_a_key() {
    let pairs = vec![pair("name", "demo"), pair("port", "1"), pair("name", "second")];
    assert_eq!(get(&pairs, "port"), Some("1"));
    assert_eq!(get(&pairs, "name"), Some("demo"));
}

#[test]
fn c05_get_returns_none_for_a_missing_key() {
    assert_eq!(get(&[pair("name", "demo")], "port"), None);
    assert_eq!(get(&[], "name"), None);
}

#[test]
fn c06_parse_config_reads_every_field() {
    let cfg = parse_config("name = api\nport = 3000\nworkers = 8\ndebug = true\n").unwrap();
    assert_eq!(cfg, Config { name: "api".into(), port: 3000, workers: 8, debug: true });
}

#[test]
fn c07_parse_config_uses_defaults_for_optional_keys() {
    let cfg = parse_config("port = 80\nname = web").unwrap();
    assert_eq!(cfg, Config { name: "web".into(), port: 80, workers: 4, debug: false });
}

#[test]
fn c08_parse_config_reports_a_missing_key() {
    assert_eq!(parse_config("name = web"), Err(ConfigError::MissingKey("port".into())));
    assert_eq!(parse_config("port = 80"), Err(ConfigError::MissingKey("name".into())));
}

#[test]
fn c09_parse_config_reports_a_bad_value() {
    let bad = |key: &str, value: &str| Err(ConfigError::BadValue { key: key.into(), value: value.into() });
    assert_eq!(parse_config("name = x\nport = abc"), bad("port", "abc"));
    // 70000 doesn't fit in a u16.
    assert_eq!(parse_config("name = x\nport = 70000"), bad("port", "70000"));
    assert_eq!(parse_config("name = x\nport = 1\nworkers = -2"), bad("workers", "-2"));
    assert_eq!(parse_config("name = x\nport = 1\ndebug = yes"), bad("debug", "yes"));
    // A bad line is reported before anything else.
    assert_eq!(parse_config("oops"), Err(ConfigError::BadLine(1)));
}

#[test]
fn c10_errors_display_readable_messages() {
    assert_eq!(ConfigError::MissingKey("port".into()).to_string(), "missing key `port`");
    assert_eq!(ConfigError::BadLine(3).to_string(), "line 3 is not `key = value`");
    let bad = ConfigError::BadValue { key: "port".into(), value: "abc".into() };
    assert_eq!(bad.to_string(), "bad value for `port`: `abc`");
}

#[test]
fn c11_config_errors_work_with_the_question_mark_operator() {
    fn port_of(text: &str) -> Result<u16, Box<dyn std::error::Error>> {
        let cfg = parse_config(text)?; // ConfigError becomes a Box<dyn Error> here
        Ok(cfg.port)
    }
    assert_eq!(port_of("name = a\nport = 9").unwrap(), 9);
    assert_eq!(port_of("name = a").unwrap_err().to_string(), "missing key `port`");
}
