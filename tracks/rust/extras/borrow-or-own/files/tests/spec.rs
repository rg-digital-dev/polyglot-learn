// Part of the Rust in Practice module (tracks/rust/extras/borrow-or-own).
// Don't edit this file: `learn` overwrites it. Your code goes in src/lib.rs.

use borrow_or_own::*;
use std::borrow::Cow;

/// The result is a slice of `input`, starting `offset` bytes in.
fn borrowed_from(result: &Cow<str>, input: &str, offset: usize) {
    match result {
        Cow::Borrowed(s) => assert_eq!(s.as_ptr() as usize, input.as_ptr() as usize + offset, "borrowed, but not from the input"),
        Cow::Owned(s) => panic!("{s:?} was built as a new String; nothing needed to change, so borrow the input"),
    }
}

fn owned(result: &Cow<str>, want: &str) {
    assert_eq!(result, want);
    assert!(matches!(result, Cow::Owned(_)), "{result:?} can't be a slice of the input");
}

#[test]
fn c01_normalize_spaces_borrows_clean_text() {
    let text = "one two three";
    let result = normalize_spaces(text);
    assert_eq!(result, "one two three");
    borrowed_from(&result, text, 0);
}

#[test]
fn c02_normalize_spaces_borrows_when_it_only_trims() {
    let text = "   one two  ";
    let result = normalize_spaces(text);
    assert_eq!(result, "one two");
    borrowed_from(&result, text, 3);
}

#[test]
fn c03_normalize_spaces_builds_a_new_string_when_it_must() {
    owned(&normalize_spaces("one  two"), "one two");
    owned(&normalize_spaces(" a\tb\n c "), "a b c");
}

#[test]
fn c04_escape_html_borrows_text_without_special_characters() {
    let text = "plain text, 100% safe";
    let result = escape_html(text);
    assert_eq!(result, text);
    borrowed_from(&result, text, 0);
}

#[test]
fn c05_escape_html_escapes_every_special_character() {
    owned(&escape_html(r#"a < b && "c" > d"#), "a &lt; b &amp;&amp; &quot;c&quot; &gt; d");
    owned(&escape_html("&amp;"), "&amp;amp;");
}

#[test]
fn c06_with_extension_keeps_a_name_that_already_has_it() {
    let name = "notes.txt";
    let result = with_extension(name, "txt");
    assert_eq!(result, "notes.txt");
    borrowed_from(&result, name, 0);
}

#[test]
fn c07_with_extension_adds_a_missing_extension() {
    owned(&with_extension("notes", "txt"), "notes.txt");
    owned(&with_extension("archive.tar", "gz"), "archive.tar.gz");
    owned(&with_extension("mytxt", "txt"), "mytxt.txt");
}

#[test]
fn c08_total_len_accepts_strs_and_strings() {
    assert_eq!(total_len(&["ab", "cde"]), 5);
    assert_eq!(total_len(&[String::from("héllo"), String::from("!")]), 6);
    assert_eq!(total_len::<&str>(&[]), 0);
}

#[test]
fn c09_cow_values_work_like_strs() {
    let escaped = escape_html("x<y");
    assert_eq!(escaped.len(), 6);
    assert!(escaped.starts_with("x&lt;"));
    let owned: String = normalize_spaces("  a   b ").into_owned();
    assert_eq!(owned, "a b");
}
