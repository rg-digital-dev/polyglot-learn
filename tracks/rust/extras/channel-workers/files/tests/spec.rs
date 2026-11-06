// Part of the Rust in Practice module (tracks/rust/extras/channel-workers).
// Don't edit this file: `learn` overwrites it. Your code goes in src/lib.rs.

use channel_workers::*;
use std::collections::HashMap;
use std::time::Instant;

#[test]
fn c01_pipeline_doubles_then_keeps_big_numbers() {
    assert_eq!(pipeline(vec![1, 4, 6, 10]), vec![12, 20]);
}

#[test]
fn c02_pipeline_keeps_the_order() {
    let input: Vec<i64> = (0..200).rev().collect();
    let want: Vec<i64> = input.iter().map(|n| n * 2).filter(|n| *n > 10).collect();
    assert_eq!(pipeline(input), want);
}

#[test]
fn c03_pipeline_of_nothing_is_empty() {
    assert_eq!(pipeline(vec![]), Vec::<i64>::new());
    assert_eq!(pipeline(vec![1, 2, 3]), Vec::<i64>::new());
}

#[test]
fn c04_squares_returns_results_in_job_order() {
    let (results, _) = squares(vec![3, 1, 2, 5], 2);
    assert_eq!(results, vec![9, 1, 4, 25]);
}

#[test]
fn c05_squares_counts_every_finished_job() {
    let (results, finished) = squares(vec![1, 2, 3, 4, 5, 6], 3);
    assert_eq!(results.len(), 6);
    assert_eq!(finished, 6);
    assert_eq!(squares(vec![], 2), (vec![], 0));
}

#[test]
fn c06_squares_works_with_more_workers_than_jobs() {
    assert_eq!(squares(vec![7], 8), (vec![49], 1));
    assert_eq!(squares(vec![2, 3], 0), (vec![4, 9], 2));
}

#[test]
fn c07_squares_runs_jobs_at_the_same_time() {
    // 8 jobs of 40 ms take 320 ms on one thread, about 80 ms on four.
    let start = Instant::now();
    let (results, _) = squares((1..=8).collect(), 4);
    let elapsed = start.elapsed().as_millis();
    assert_eq!(results, vec![1, 4, 9, 16, 25, 36, 49, 64]);
    assert!(elapsed < 250, "took {elapsed} ms: are the workers taking turns instead of working in parallel?");
}

#[test]
fn c08_count_words_merges_every_thread_s_counts() {
    let counts = count_words(vec!["the cat".into(), "The dog\tand the cat".into(), "".into()]);
    let want: HashMap<String, usize> =
        [("the", 3), ("cat", 2), ("dog", 1), ("and", 1)].into_iter().map(|(w, n)| (w.to_string(), n)).collect();
    assert_eq!(counts, want);
}

#[test]
fn c09_count_words_of_no_chunks_is_empty() {
    assert!(count_words(vec![]).is_empty());
}
