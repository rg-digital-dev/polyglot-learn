use std::collections::HashMap;
use std::thread;
use std::time::Duration;

/// A slow job: waits 40 ms, then returns `n * n`.
pub fn slow_square(n: u64) -> u64 {
    thread::sleep(Duration::from_millis(40));
    n * n
}

/// Two stages, each its own thread, joined by channels: the first doubles every
/// number, the second keeps only results greater than 10. The calling thread
/// sends the numbers in and collects what comes out, in the original order.
pub fn pipeline(numbers: Vec<i64>) -> Vec<i64> {
    todo!("implement pipeline({numbers:?})")
}

/// Start `workers` threads (at least 1) that take jobs from one shared queue
/// and run `slow_square` on them. Return the results in the order of `jobs`,
/// and how many jobs were finished, counted with an atomic.
pub fn squares(jobs: Vec<u64>, workers: usize) -> (Vec<u64>, usize) {
    todo!("implement squares({jobs:?}, {workers})")
}

/// Count lowercase words (split on whitespace) with one thread per chunk. Each
/// thread sends its own counts back over a channel; merge them at the end.
pub fn count_words(chunks: Vec<String>) -> HashMap<String, usize> {
    todo!("implement count_words({chunks:?})")
}
