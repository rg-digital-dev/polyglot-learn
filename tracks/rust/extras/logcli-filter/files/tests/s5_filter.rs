// Stage 5 of the log CLI project. Don't edit this file: `learn` overwrites it.
// Your code goes in src/filter.rs.

use std::collections::BTreeMap;

use log_cli::args::Args;
use log_cli::entry::{parse_line, Entry, Level};
use log_cli::filter::{summarize, Filter, Stats};

fn log() -> Vec<Entry> {
    [
        "t1 INFO http: GET /",
        "t2 WARN http: slow request",
        "t3 ERROR db: connection lost",
        "t4 DEBUG cache: miss",
        "t5 INFO db: reconnected",
        "t6 ERROR http: Timeout talking to db",
    ]
    .iter()
    .map(|l| parse_line(l).unwrap())
    .collect()
}

fn kept(f: &Filter) -> Vec<String> {
    log().into_iter().filter(|e| f.matches(e)).map(|e| e.timestamp).collect()
}

#[test]
fn c26_an_empty_filter_keeps_everything() {
    assert_eq!(kept(&Filter::default()).len(), 6);
}

#[test]
fn c27_level_keeps_that_level_and_above() {
    let f = Filter { min_level: Some(Level::Warn), grep: None };
    assert_eq!(kept(&f), ["t2", "t3", "t6"]);
}

#[test]
fn c28_grep_matches_message_or_target_in_any_case() {
    let f = Filter { min_level: None, grep: Some("DB".into()) };
    assert_eq!(kept(&f), ["t3", "t5", "t6"], "target db, or db in the message");
    let f = Filter { min_level: None, grep: Some("timeout".into()) };
    assert_eq!(kept(&f), ["t6"]);
    let args = Args::parse(["--level".to_string(), "error".into(), "--grep".into(), "lost".into()]).unwrap();
    assert_eq!(kept(&Filter::from_args(&args)), ["t3"]);
}

#[test]
fn c29_summarize_counts_matching_entries() {
    let entries = log();
    let s = summarize(&entries, &Filter { min_level: Some(Level::Info), grep: None });
    assert_eq!(s.total, 5);
    assert_eq!(s.by_level, BTreeMap::from([(Level::Info, 2), (Level::Warn, 1), (Level::Error, 2)]));
    assert_eq!(s.targets.get("http"), Some(&3));
    assert_eq!(s.targets.get("cache"), None, "cache's only entry is DEBUG");
}

#[test]
fn c30_top_targets_sorts_by_count_then_name() {
    let s = summarize(&log(), &Filter::default());
    assert_eq!(s.top_targets(10), [("http".to_string(), 3), ("db".into(), 2), ("cache".into(), 1)]);
    assert_eq!(s.top_targets(1), [("http".to_string(), 3)]);
    let mut tie = Stats::default();
    for t in ["zeta", "alpha", "mid"] {
        tie.add(&parse_line(&format!("t INFO {t}: x")).unwrap());
    }
    assert_eq!(tie.top_targets(2), [("alpha".to_string(), 1), ("mid".into(), 1)]);
}

#[test]
fn c31_merge_adds_another_stats() {
    let all = log();
    let (a, b) = all.split_at(2);
    let mut s = summarize(a, &Filter::default());
    s.merge(summarize(b, &Filter::default()));
    assert_eq!(s, summarize(&log(), &Filter::default()));
}
