// Part of the Rust in Practice module (tracks/rust/extras/id-registry). Don't
// edit this file: `learn` overwrites it. Your code goes in src/lib.rs.
//
// Tests run in parallel and share your statics, so they check what must always
// hold (unique, increasing, built once) rather than exact IDs.

use id_registry::*;
use std::collections::HashSet;
use std::thread;

#[test]
fn c01_next_id_starts_above_zero_and_increases() {
    let a = next_id();
    let b = next_id();
    assert!(a >= 1, "IDs start at 1");
    assert!(b > a, "{b} should be greater than {a}");
}

#[test]
fn c02_next_id_is_unique_across_threads() {
    let handles: Vec<_> = (0..8).map(|_| thread::spawn(|| (0..500).map(|_| next_id()).collect::<Vec<_>>())).collect();
    let mut seen = HashSet::new();
    for h in handles {
        for id in h.join().unwrap() {
            assert!(seen.insert(id), "ID {id} was handed out twice");
        }
    }
    assert_eq!(seen.len(), 4000);
}

#[test]
fn c03_color_code_finds_known_colours() {
    assert_eq!(color_code("red"), Some(0xff0000));
    assert_eq!(color_code("green"), Some(0x00ff00));
    assert_eq!(color_code("blue"), Some(0x0000ff));
    assert_eq!(color_code("white"), Some(0xffffff));
    assert_eq!(color_code("black"), Some(0x000000));
}

#[test]
fn c04_color_code_ignores_case() {
    assert_eq!(color_code("Red"), Some(0xff0000));
    assert_eq!(color_code("WHITE"), Some(0xffffff));
}

#[test]
fn c05_color_code_returns_none_for_an_unknown_colour() {
    assert_eq!(color_code("mauve"), None);
    assert_eq!(color_code(""), None);
}

#[test]
fn c06_the_colour_table_is_built_only_once() {
    let handles: Vec<_> = (0..8).map(|_| thread::spawn(|| (0..100).filter_map(|_| color_code("blue")).count())).collect();
    for h in handles {
        assert_eq!(h.join().unwrap(), 100);
    }
    assert_eq!(table_builds(), 1);
}

#[test]
fn c07_the_app_name_can_be_set_exactly_once() {
    assert_eq!(app_name(), None);
    assert_eq!(set_app_name("haunt"), Ok(()));
    assert_eq!(app_name(), Some("haunt"));
    assert_eq!(set_app_name("other"), Err("app name is already `haunt`".to_string()));
    assert_eq!(app_name(), Some("haunt"));
}
