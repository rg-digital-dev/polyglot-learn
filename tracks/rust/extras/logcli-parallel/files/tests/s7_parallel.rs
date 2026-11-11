// Stage 7 of the log CLI project. Don't edit this file: `learn` overwrites it.
// Your code goes in src/parallel.rs (and run.rs).

use std::io::Cursor;
use std::path::PathBuf;

use log_cli::entry::Level;
use log_cli::error::Error;
use log_cli::filter::{Filter, Stats};
use log_cli::parallel::process_files;
use log_cli::run::run;

/// Write `n` log files and return their paths; file i has i+1 lines.
fn files(name: &str, n: usize) -> Vec<String> {
    let dir: PathBuf = std::env::temp_dir().join(format!("logq-s7-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    (0..n)
        .map(|i| {
            let path = dir.join(format!("f{i}.log"));
            let lines: String = (0..=i).map(|k| format!("t{k} {} f{i}: line {k}\n", if k % 2 == 0 { "INFO" } else { "ERROR" })).collect();
            std::fs::write(&path, lines).unwrap();
            path.to_string_lossy().into_owned()
        })
        .collect()
}

#[test]
fn c39_results_keep_the_order_of_the_paths() {
    let paths = files("order", 8);
    let got = process_files(&paths, &Filter::default(), 4);
    assert_eq!(got.iter().map(|r| r.path.clone()).collect::<Vec<_>>(), paths);
    assert_eq!(got.iter().map(|r| r.entries.len()).collect::<Vec<_>>(), [1, 2, 3, 4, 5, 6, 7, 8]);
    assert!(got[3].entries.iter().all(|e| e.target == "f3"), "each result holds its own file's entries");
}

#[test]
fn c40_each_file_counts_only_its_matching_entries() {
    let paths = files("stats", 4);
    let got = process_files(&paths, &Filter { min_level: Some(Level::Error), grep: None }, 2);
    assert_eq!(got.iter().map(|r| r.stats.total).collect::<Vec<_>>(), [0, 1, 1, 2]);
    let mut all = Stats::default();
    for r in got {
        all.merge(r.stats);
    }
    assert_eq!(all.total, 4);
}

#[test]
fn c41_an_unreadable_file_does_not_stop_the_others() {
    let mut paths = files("missing", 2);
    paths.insert(1, "no/such/file.log".into());
    let got = process_files(&paths, &Filter::default(), 3);
    assert_eq!(got.len(), 3);
    assert!(matches!(got[1].errors.as_slice(), [Error::Open { .. }]), "{:?}", got[1].errors);
    assert_eq!((got[0].entries.len(), got[2].entries.len()), (1, 2));
}

#[test]
fn c42_any_number_of_jobs_gives_the_same_results() {
    let paths = files("jobs", 6);
    let summary = |jobs| process_files(&paths, &Filter::default(), jobs).into_iter().map(|r| (r.path, r.entries)).collect::<Vec<_>>();
    let one = summary(1);
    assert_eq!(summary(3), one);
    assert_eq!(summary(50), one, "more jobs than files is fine");
    assert!(process_files(&[], &Filter::default(), 4).is_empty());
}

#[test]
fn c43_run_with_jobs_prints_files_in_order() {
    let paths = files("run", 5);
    let output = |jobs: &str| {
        let mut a = vec!["--jobs".to_string(), jobs.to_string()];
        a.extend(paths.iter().cloned());
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let code = run(a, Cursor::new(String::new()), &mut out, &mut err);
        (code, String::from_utf8(out).unwrap())
    };
    let (code, one) = output("1");
    assert_eq!(code, 0);
    assert_eq!(one.lines().count(), 15);
    assert_eq!(output("4"), (0, one));
}
