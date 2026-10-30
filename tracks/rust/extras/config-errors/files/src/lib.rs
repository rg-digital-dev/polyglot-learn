use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub name: String,
    pub port: u16,
    pub workers: u32,
    pub debug: bool,
}

/// Everything that can go wrong while reading a config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    /// A required key isn't there.
    MissingKey(String),
    /// This line (counting from 1) isn't blank, a `#` comment, or `key = value`.
    BadLine(usize),
    /// The value can't be read as the type the key needs.
    BadValue { key: String, value: String },
}

impl fmt::Display for ConfigError {
    /// Messages, with the parts in backticks:
    /// - MissingKey: "missing key `port`"
    /// - BadLine: "line 3 is not `key = value`"
    /// - BadValue: "bad value for `port`: `abc`"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f;
        todo!("implement Display for {self:?}")
    }
}

/// Lets `?` turn a ConfigError into a `Box<dyn std::error::Error>`.
impl std::error::Error for ConfigError {}

/// Read `key = value` lines (spaces around both trimmed). Blank lines and lines
/// starting with `#` are skipped. Any other line without `=` is a BadLine.
pub fn parse_pairs(text: &str) -> Result<Vec<(String, String)>, ConfigError> {
    todo!("implement parse_pairs({text:?})")
}

/// The value for `key`, or None if it isn't set. The first match wins.
pub fn get<'a>(pairs: &'a [(String, String)], key: &str) -> Option<&'a str> {
    todo!("implement get({pairs:?}, {key:?})")
}

/// Build a Config. `name` and `port` (a u16) are required; `workers` (a u32)
/// defaults to 4 and `debug` ("true" or "false") defaults to false.
pub fn parse_config(text: &str) -> Result<Config, ConfigError> {
    todo!("implement parse_config({text:?})")
}
