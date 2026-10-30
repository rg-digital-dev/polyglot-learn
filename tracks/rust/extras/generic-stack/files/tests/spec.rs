// Part of the Deeper Rust module (tracks/rust/extras/generic-stack). Don't
// edit this file: `learn` overwrites it. Your code goes in src/lib.rs.

use generic_stack::*;

#[derive(Debug, PartialEq)]
struct Player {
    name: &'static str,
    score: u32,
}

#[test]
fn c01_a_new_stack_is_empty() {
    let s: Stack<i32> = Stack::new();
    assert_eq!(s.len(), 0);
    assert!(s.is_empty());
}

#[test]
fn c02_push_and_pop_are_last_in_first_out() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.len(), 3);
    assert!(!s.is_empty());
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.len(), 1);
}

#[test]
fn c03_pop_on_an_empty_stack_is_none() {
    let mut s: Stack<String> = Stack::new();
    assert_eq!(s.pop(), None);
    s.push("x".to_string());
    assert_eq!(s.pop(), Some("x".to_string()));
    assert_eq!(s.pop(), None);
}

#[test]
fn c04_peek_shows_the_top_without_removing_it() {
    let mut s = Stack::new();
    assert_eq!(s.peek(), None);
    s.push('a');
    s.push('b');
    assert_eq!(s.peek(), Some(&'b'));
    assert_eq!(s.len(), 2);
}

#[test]
fn c05_a_stack_works_with_any_type() {
    // Player isn't Display, Clone or Copy: the basic methods don't need any of that.
    let mut s = Stack::new();
    s.push(Player { name: "ada", score: 3 });
    s.push(Player { name: "lin", score: 5 });
    assert_eq!(s.peek().map(|p| p.name), Some("lin"));
    assert_eq!(s.pop(), Some(Player { name: "lin", score: 5 }));
}

#[test]
fn c06_describe_lists_items_from_bottom_to_top() {
    let mut s = Stack::new();
    assert_eq!(s.describe(), "[]");
    s.push(10);
    s.push(20);
    s.push(30);
    assert_eq!(s.describe(), "[10, 20, 30]");
    let mut words = Stack::new();
    words.push("to");
    words.push("be");
    assert_eq!(words.describe(), "[to, be]");
}

#[test]
fn c07_largest_finds_the_biggest_number() {
    assert_eq!(largest(&[3, 9, -2, 7]), Some(9));
    assert_eq!(largest(&[-5, -1, -9]), Some(-1));
}

#[test]
fn c08_largest_works_for_chars_and_floats() {
    assert_eq!(largest(&['q', 'z', 'a']), Some('z'));
    assert_eq!(largest(&[1.5, 0.25, 2.75]), Some(2.75));
}

#[test]
fn c09_largest_of_nothing_is_none() {
    let empty: [u8; 0] = [];
    assert_eq!(largest(&empty), None);
}

#[test]
fn c10_largest_by_key_returns_a_reference_to_the_best_item() {
    let players = [Player { name: "ada", score: 3 }, Player { name: "lin", score: 8 }, Player { name: "bo", score: 5 }];
    let best = largest_by_key(&players, |p| p.score).unwrap();
    assert_eq!(best.name, "lin");
    assert!(std::ptr::eq(best, &players[1]), "return a reference into the slice, not a copy");
    let longest = largest_by_key(&["pear", "fig", "banana"], |w| w.len());
    assert_eq!(longest, Some(&"banana"));
}

#[test]
fn c11_largest_by_key_keeps_the_first_on_a_tie() {
    let players = [Player { name: "ada", score: 7 }, Player { name: "lin", score: 7 }];
    assert_eq!(largest_by_key(&players, |p| p.score).map(|p| p.name), Some("ada"));
    assert_eq!(largest_by_key(&[] as &[Player], |p| p.score), None);
}
