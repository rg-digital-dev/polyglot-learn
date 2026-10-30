use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
#[allow(unused_imports)] // for the two parallel_ functions
use std::sync::{Arc, Mutex};

/// One list with several owners (Rc) that any of them may change (RefCell).
pub type SharedList = Rc<RefCell<Vec<String>>>;

/// A new, empty shared list.
pub fn new_list() -> SharedList {
    todo!("implement new_list")
}

/// Someone who adds to a list they share with others.
pub struct Shopper {
    pub name: String,
    list: SharedList,
}

impl Shopper {
    /// A shopper who shares `list` (a new pointer to it, not a copy).
    pub fn new(name: &str, list: &SharedList) -> Shopper {
        let _ = list;
        todo!("implement Shopper::new({name:?})")
    }

    /// Add `"<item> (<name>)"` to the shared list, e.g. "milk (ada)".
    /// Note `&self`: the list changes even though the shopper doesn't.
    pub fn add(&self, item: &str) {
        let _ = &self.list;
        todo!("implement Shopper::add({item:?}) for {}", self.name)
    }

    /// How many items are on the shared list.
    pub fn count(&self) -> usize {
        todo!("implement Shopper::count for {}", self.name)
    }
}

/// Start `threads` threads that each add 1 to a shared counter `times` times,
/// wait for all of them, and return the total.
pub fn parallel_increments(threads: usize, times: usize) -> usize {
    todo!("implement parallel_increments({threads}, {times})")
}

/// Count the words in every chunk, one thread per chunk, all adding into one
/// shared map. Words are split on whitespace and lowercased.
pub fn parallel_word_count(chunks: Vec<String>) -> HashMap<String, usize> {
    todo!("implement parallel_word_count({chunks:?})")
}
