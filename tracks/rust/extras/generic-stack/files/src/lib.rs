use std::fmt::Display;

/// A last-in, first-out stack of any type. The field is private: code outside
/// this module can only use the methods.
#[derive(Debug, Clone, PartialEq)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        todo!("implement Stack::new")
    }

    pub fn push(&mut self, item: T) {
        let _ = item;
        todo!("implement Stack::push")
    }

    /// Remove and return the top item, or None when the stack is empty.
    pub fn pop(&mut self) -> Option<T> {
        todo!("implement Stack::pop")
    }

    /// The top item without removing it.
    pub fn peek(&self) -> Option<&T> {
        todo!("implement Stack::peek")
    }

    pub fn len(&self) -> usize {
        todo!("implement Stack::len")
    }

    pub fn is_empty(&self) -> bool {
        todo!("implement Stack::is_empty")
    }
}

/// Only stacks of displayable items get `describe`.
impl<T: Display> Stack<T> {
    /// The items from bottom to top, like `[1, 2, 3]`; `[]` when empty.
    pub fn describe(&self) -> String {
        todo!("implement Stack::describe")
    }
}

/// The largest item, or None for an empty slice.
pub fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let _ = items;
    todo!("implement largest")
}

/// The item whose key is largest (the first one on a tie), or None.
pub fn largest_by_key<T, K: PartialOrd>(items: &[T], key: impl Fn(&T) -> K) -> Option<&T> {
    let _ = (items, key);
    todo!("implement largest_by_key")
}
