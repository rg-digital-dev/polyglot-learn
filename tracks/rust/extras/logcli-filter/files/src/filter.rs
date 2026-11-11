//! Which entries to keep, and what they add up to.

use std::collections::{BTreeMap, HashMap};

use crate::args::Args;
use crate::entry::{Entry, Level};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Filter {
    /// Keep entries at this level or above.
    pub min_level: Option<Level>,
    /// Keep entries whose target or message contains this, ignoring case.
    pub grep: Option<String>,
}

impl Filter {
    pub fn from_args(args: &Args) -> Filter {
        todo!()
    }

    /// Whether `e` passes both checks (a check that's None always passes).
    pub fn matches(&self, e: &Entry) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Stats {
    pub total: usize,
    pub by_level: BTreeMap<Level, usize>,
    pub targets: HashMap<String, usize>,
}

impl Stats {
    /// Count one more entry.
    pub fn add(&mut self, e: &Entry) {
        todo!()
    }

    /// Add `other`'s counts to these.
    pub fn merge(&mut self, other: Stats) {
        todo!()
    }

    /// The `n` busiest targets: most entries first, ties by name.
    pub fn top_targets(&self, n: usize) -> Vec<(String, usize)> {
        todo!()
    }
}

/// Stats for the entries `filter` keeps.
pub fn summarize<'a>(entries: impl IntoIterator<Item = &'a Entry>, filter: &Filter) -> Stats {
    todo!("filter, then fold into a Stats")
}
