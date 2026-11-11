// Stage 8 of the log CLI project. Don't edit this file: `learn` overwrites it.
// Your code goes in src/borrowed.rs.

use std::collections::BTreeMap;

use log_cli::borrowed::{count_levels, parse_ref, targets_at};
use log_cli::entry::{parse_line, Level};

/// Whether `part` points into `whole`'s memory, not at a copy.
fn inside(part: &str, whole: &str) -> bool {
    let (p, w) = (part.as_ptr() as usize, whole.as_ptr() as usize);
    p >= w && p + part.len() <= w + whole.len()
}

const LOG: &str = "t1 INFO http: GET /\nt2 WARN db: slow\n\nbroken\nt3 ERROR http: down\nt4 WARN cache: full\nt5 WARN db: slow again\n";

#[test]
fn c44_fields_borrow_from_the_line() {
    let line = String::from("  t1 WARN http:  slow request  ");
    let e = parse_ref(&line).unwrap();
    assert_eq!((e.timestamp, e.level, e.target, e.message), ("t1", Level::Warn, "http", "slow request"));
    for field in [e.timestamp, e.target, e.message] {
        assert!(inside(field, &line), "{field:?} is a copy, not a slice of the line");
    }
}

#[test]
fn c45_parse_ref_agrees_with_parse_line() {
    for line in ["t1 INFO a: b c", "t2 error  db:", "", "t3", "t4 INFO", "t5 LOUD x: y", "t6 INFO nocolon y", " t7  debug  z:  w "] {
        let owned = parse_line(line);
        let borrowed = parse_ref(line).map(|e| e.to_entry());
        assert_eq!(borrowed, owned, "for {line:?}");
    }
}

#[test]
fn c46_count_levels_counts_entries_and_bad_lines() {
    let (counts, bad) = count_levels(LOG);
    assert_eq!(counts, BTreeMap::from([(Level::Info, 1), (Level::Warn, 3), (Level::Error, 1)]));
    assert_eq!(bad, 1, "the blank line isn't counted, \"broken\" is");
    assert_eq!(count_levels(""), (BTreeMap::new(), 0));
}

#[test]
fn c47_targets_at_returns_slices_of_the_text_first_seen_first() {
    let text = String::from(LOG);
    let warn = targets_at(&text, Level::Warn);
    assert_eq!(warn, ["db", "cache"]);
    assert!(warn.iter().all(|t| inside(t, &text)), "targets should borrow from the text");
    assert!(targets_at(LOG, Level::Trace).is_empty());
}
