// Part of the Rust in Practice module (tracks/rust/extras/scoped-guard). Don't
// edit this file: `learn` overwrites it. Your code goes in src/lib.rs.

use scoped_guard::*;
use std::panic::{self, AssertUnwindSafe};

fn lines(log: &Log) -> Vec<String> {
    log.borrow().clone()
}

fn expect(log: &Log, want: &[&str]) {
    assert_eq!(lines(log), want.iter().map(|s| s.to_string()).collect::<Vec<_>>());
}

#[test]
fn c01_begin_logs_a_begin_line() {
    let log = new_log();
    let tx = Transaction::begin("save", &log);
    expect(&log, &["begin save"]);
    tx.commit();
}

#[test]
fn c02_record_logs_entries_with_the_transaction_name() {
    let log = new_log();
    let tx = Transaction::begin("save", &log);
    tx.record("write header");
    tx.record("write body");
    expect(&log, &["begin save", "save: write header", "save: write body"]);
    tx.commit();
}

#[test]
fn c03_a_transaction_rolls_back_when_it_goes_out_of_scope() {
    let log = new_log();
    {
        let tx = Transaction::begin("save", &log);
        tx.record("write");
    }
    expect(&log, &["begin save", "save: write", "rollback save"]);
}

#[test]
fn c04_commit_logs_commit_instead_of_rollback() {
    let log = new_log();
    let tx = Transaction::begin("save", &log);
    tx.commit();
    expect(&log, &["begin save", "commit save"]);
}

#[test]
fn c05_transactions_are_dropped_in_reverse_order() {
    let log = new_log();
    {
        let _first = Transaction::begin("first", &log);
        let _second = Transaction::begin("second", &log);
    }
    expect(&log, &["begin first", "begin second", "rollback second", "rollback first"]);
}

#[test]
fn c06_drop_ends_a_transaction_early() {
    let log = new_log();
    {
        let first = Transaction::begin("first", &log);
        drop(first);
        let _second = Transaction::begin("second", &log);
    }
    expect(&log, &["begin first", "rollback first", "begin second", "rollback second"]);
}

#[test]
fn c07_transfer_commits_when_every_step_succeeds() {
    let log = new_log();
    assert_eq!(transfer(&log, &["debit", "credit"]), Ok(()));
    expect(&log, &["begin transfer", "transfer: debit", "transfer: credit", "commit transfer"]);
}

#[test]
fn c08_transfer_rolls_back_and_stops_at_a_failing_step() {
    let log = new_log();
    assert_eq!(transfer(&log, &["debit", "!credit", "notify"]), Err("step `!credit` failed".to_string()));
    expect(&log, &["begin transfer", "transfer: debit", "rollback transfer"]);
}

#[test]
fn c09_transfer_with_no_steps_still_commits() {
    let log = new_log();
    assert_eq!(transfer(&log, &[]), Ok(()));
    expect(&log, &["begin transfer", "commit transfer"]);
}

#[test]
fn c10_batch_commits_good_groups_and_rolls_back_failed_ones() {
    let log = new_log();
    let groups = vec![vec!["a"], vec!["!b"], vec!["c", "d"]];
    assert_eq!(batch(&log, &groups), 2);
    expect(
        &log,
        &[
            "begin transfer", "transfer: a", "commit transfer",
            "begin transfer", "rollback transfer",
            "begin transfer", "transfer: c", "transfer: d", "commit transfer",
        ],
    );
}

#[test]
fn c11_a_panic_still_rolls_back() {
    let log = new_log();
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let tx = Transaction::begin("save", &log);
        tx.record("write");
        panic!("disk on fire");
    }));
    assert!(result.is_err());
    expect(&log, &["begin save", "save: write", "rollback save"]);
}
