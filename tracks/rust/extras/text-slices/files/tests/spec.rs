// Part of the Deeper Rust module (tracks/rust/extras/text-slices). Don't edit
// this file: `learn` overwrites it. Your code goes in src/lib.rs.

use text_slices::*;

/// Does `inner` point into the memory of `outer` (a borrow, not a copy)?
fn inside(outer: &str, inner: &str) -> bool {
    let start = outer.as_ptr() as usize;
    let p = inner.as_ptr() as usize;
    p >= start && p + inner.len() <= start + outer.len()
}

#[test]
fn c01_first_word_returns_the_text_before_the_first_space() {
    assert_eq!(first_word("hello brave new world"), "hello");
}

#[test]
fn c02_first_word_of_a_single_word_is_the_whole_text() {
    assert_eq!(first_word("rust"), "rust");
    assert_eq!(first_word(""), "");
}

#[test]
fn c03_first_word_borrows_from_the_input() {
    let text = String::from("borrowed slices");
    let word = first_word(&text);
    assert_eq!(word, "borrowed");
    assert!(inside(&text, word), "first_word should return a slice of its input, not a new String");
}

#[test]
fn c04_longest_picks_the_longer_string() {
    assert_eq!(longest("short", "much longer"), "much longer");
    assert_eq!(longest("much longer", "short"), "much longer");
}

#[test]
fn c05_longest_picks_the_first_on_a_tie() {
    let a = String::from("same");
    let b = String::from("size");
    let got = longest(&a, &b);
    assert_eq!(got, "same");
    assert!(inside(&a, got));
}

#[test]
fn c06_longest_counts_characters_not_bytes() {
    // "éé" is 4 bytes but 2 characters.
    assert_eq!(longest("éé", "abc"), "abc");
}

#[test]
fn c07_sentences_splits_on_full_stops_and_trims() {
    let e = Excerpt { text: "One fish. Two fish.  Red fish." };
    assert_eq!(e.sentences(), vec!["One fish", "Two fish", "Red fish"]);
}

#[test]
fn c08_sentences_skips_empty_pieces() {
    let e = Excerpt { text: "...Hi.. there. ." };
    assert_eq!(e.sentences(), vec!["Hi", "there"]);
    assert!(Excerpt { text: "" }.sentences().is_empty());
}

#[test]
fn c09_sentences_outlive_the_excerpt() {
    let text = String::from("First. Second.");
    let parts = {
        let excerpt = Excerpt { text: &text };
        excerpt.sentences()
        // `excerpt` is dropped here; the sentences borrow from `text`, not from it.
    };
    assert_eq!(parts, vec!["First", "Second"]);
    assert!(parts.iter().all(|p| inside(&text, p)), "sentences should be slices of the text");
}

#[test]
fn c10_longest_sentence_finds_the_longest() {
    let e = Excerpt { text: "Short. A bit longer. Tiny. Equal size." };
    assert_eq!(e.longest_sentence(), Some("A bit longer"));
}

#[test]
fn c11_longest_sentence_of_no_text_is_none() {
    assert_eq!(Excerpt { text: " . " }.longest_sentence(), None);
}
