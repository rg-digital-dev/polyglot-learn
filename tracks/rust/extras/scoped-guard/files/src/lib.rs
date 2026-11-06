use std::cell::RefCell;
use std::rc::Rc;

/// A shared list of log lines. The tests keep one and read it afterwards.
pub type Log = Rc<RefCell<Vec<String>>>;

/// An empty log.
pub fn new_log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

/// A step that fails if it starts with `!`, with the error "step `!x` failed".
pub fn check_step(step: &str) -> Result<(), String> {
    if step.starts_with('!') {
        Err(format!("step `{step}` failed"))
    } else {
        Ok(())
    }
}

/// A transaction that rolls back unless it's committed.
pub struct Transaction {
    name: String,
    log: Log,
    committed: bool,
}

impl Transaction {
    /// Start a transaction: log "begin <name>" and keep a handle to the log.
    pub fn begin(name: &str, log: &Log) -> Transaction {
        todo!("implement begin({name:?})")
    }

    /// Log "<name>: <entry>".
    pub fn record(&self, entry: &str) {
        todo!("implement record({entry:?})")
    }

    /// Mark the transaction as committed. It takes `self`, so the transaction is
    /// dropped (and ended) when this returns, and can't be used afterwards.
    pub fn commit(mut self) {
        todo!("implement commit")
    }
}

impl Drop for Transaction {
    /// Runs when the transaction goes out of scope, however that happens.
    /// Log "commit <name>" if it was committed, and "rollback <name>" otherwise.
    fn drop(&mut self) {
        // TODO: write the commit or rollback line. (This body is left empty on
        // purpose: a panic inside drop during another panic aborts the tests.)
    }
}

/// Run `steps` in one transaction named "transfer": check each step with
/// `check_step` and record it. Commit if every step passes. On the first failing
/// step, return its error; the transaction then rolls back by itself.
pub fn transfer(log: &Log, steps: &[&str]) -> Result<(), String> {
    todo!("implement transfer({steps:?})")
}

/// Run `transfer` once per group and return how many groups were committed.
/// A failing group doesn't stop the ones after it.
pub fn batch(log: &Log, groups: &[Vec<&str>]) -> usize {
    todo!("implement batch({groups:?})")
}
