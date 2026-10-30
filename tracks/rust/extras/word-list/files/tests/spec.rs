// Part of the Deeper Rust module (tracks/rust/extras/word-list). Don't edit
// this file: `learn` overwrites it. Your code goes in src/lib.rs.

use word_list::*;

fn words(list: &[&str]) -> Vec<String> {
    list.iter().map(|w| w.to_string()).collect()
}

#[test]
fn c01_total_len_counts_characters_of_every_word() {
    assert_eq!(total_len(&words(&["ab", "cde", "f"])), 6);
    // Characters, not bytes: "é" is two bytes in UTF-8.
    assert_eq!(total_len(&words(&["café"])), 4);
}

#[test]
fn c02_total_len_of_no_words_is_zero() {
    assert_eq!(total_len(&[]), 0);
}

#[test]
fn c03_total_len_leaves_the_list_usable() {
    let list = words(&["one", "two"]);
    let n = total_len(&list);
    // `list` was only borrowed, so it is still ours to use.
    assert_eq!(n, 6);
    assert_eq!(list, words(&["one", "two"]));
}

#[test]
fn c04_shout_uppercases_every_word_in_place() {
    let mut list = words(&["hi", "there"]);
    shout(&mut list);
    assert_eq!(list[0].trim_end_matches('!'), "HI");
    assert_eq!(list[1].trim_end_matches('!'), "THERE");
}

#[test]
fn c05_shout_adds_an_exclamation_mark() {
    let mut list = words(&["go", "now"]);
    shout(&mut list);
    assert_eq!(list, words(&["GO!", "NOW!"]));
}

#[test]
fn c06_shout_on_an_empty_list_changes_nothing() {
    let mut list: Vec<String> = Vec::new();
    shout(&mut list);
    assert!(list.is_empty());
}

#[test]
fn c07_into_sentence_joins_with_spaces() {
    let s = into_sentence(words(&["learn", "rust", "today"]));
    assert_eq!(s.to_lowercase(), "learn rust today.");
}

#[test]
fn c08_into_sentence_capitalises_the_first_letter_and_ends_with_a_full_stop() {
    assert_eq!(into_sentence(words(&["the", "end"])), "The end.");
    assert_eq!(into_sentence(words(&["éclair"])), "Éclair.");
}

#[test]
fn c09_into_sentence_of_no_words_is_empty() {
    assert_eq!(into_sentence(Vec::new()), "");
}
