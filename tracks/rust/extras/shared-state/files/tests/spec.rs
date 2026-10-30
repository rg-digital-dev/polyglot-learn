// Part of the Deeper Rust module (tracks/rust/extras/shared-state). Don't
// edit this file: `learn` overwrites it. Your code goes in src/lib.rs.

use std::collections::HashMap;
use std::rc::Rc;

use shared_state::*;

#[test]
fn c01_new_list_starts_empty() {
    let list = new_list();
    assert!(list.borrow().is_empty());
}

#[test]
fn c02_shoppers_share_one_list() {
    let list = new_list();
    let ada = Shopper::new("ada", &list);
    let lin = Shopper::new("lin", &list);
    ada.add("milk");
    lin.add("bread");
    ada.add("eggs");
    assert_eq!(*list.borrow(), vec!["milk (ada)", "bread (lin)", "eggs (ada)"]);
    assert_eq!(ada.count(), 3);
    assert_eq!(lin.count(), 3);
}

#[test]
fn c03_add_works_through_a_shared_reference() {
    let list = new_list();
    let shopper = Shopper::new("bo", &list); // not `mut`
    let by_ref: &Shopper = &shopper;
    by_ref.add("tea");
    assert_eq!(shopper.count(), 1);
    assert_eq!(list.borrow()[0], "tea (bo)");
}

#[test]
fn c04_each_shopper_holds_a_reference_to_the_list() {
    let list = new_list();
    assert_eq!(Rc::strong_count(&list), 1);
    let _a = Shopper::new("a", &list);
    let _b = Shopper::new("b", &list);
    assert_eq!(Rc::strong_count(&list), 3, "each shopper should Rc::clone the list");
}

#[test]
fn c05_dropping_a_shopper_releases_its_reference() {
    let list = new_list();
    let a = Shopper::new("a", &list);
    a.add("jam");
    drop(a);
    assert_eq!(Rc::strong_count(&list), 1);
    // The item outlives the shopper who added it.
    assert_eq!(*list.borrow(), vec!["jam (a)"]);
}

#[test]
fn c06_parallel_increments_adds_up_every_increment() {
    assert_eq!(parallel_increments(4, 1000), 4000);
    assert_eq!(parallel_increments(1, 7), 7);
    assert_eq!(parallel_increments(8, 0), 0);
}

#[test]
fn c07_parallel_word_count_counts_across_chunks() {
    let chunks = vec!["the cat".to_string(), "The dog  and the cat".to_string(), "dog".to_string()];
    let counts = parallel_word_count(chunks);
    let expected: HashMap<String, usize> =
        [("the", 3), ("cat", 2), ("dog", 2), ("and", 1)].into_iter().map(|(w, n)| (w.to_string(), n)).collect();
    assert_eq!(counts, expected);
}

#[test]
fn c08_parallel_word_count_of_no_chunks_is_empty() {
    assert!(parallel_word_count(Vec::new()).is_empty());
    assert!(parallel_word_count(vec!["   ".to_string()]).is_empty());
}
