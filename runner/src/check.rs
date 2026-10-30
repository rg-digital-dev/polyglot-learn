//! Running one exercise's tests and turning native output into per-case results.
//! Used by `learn check`, `learn watch`, `learn selftest` and `learn serve`, so
//! every front end gets identical results.

use std::path::Path;

use serde::Serialize;

use crate::backends::{self, Backend};
use crate::bail;
use crate::ctx::Ctx;
use crate::error::Result;
use crate::spec::Spec;
use crate::toolchain::{self, Found};

#[derive(Debug, Clone, Serialize)]
pub struct CaseResult {
    pub index: usize,
    pub description: String,
    /// `None` when the case never ran (compile error, crash, timeout).
    pub passed: Option<bool>,
    /// The test framework's own failure message, when it could be found.
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CheckResult {
    pub cases: Vec<CaseResult>,
    pub passed: bool,
    pub timed_out: bool,
    pub command: String,
    pub output: String,
    /// When no case ran because of the toolchain rather than the code.
    pub setup_problem: Option<String>,
}

impl CheckResult {
    pub fn count(&self, state: Option<bool>) -> usize {
        self.cases.iter().filter(|c| c.passed == state).count()
    }

    pub fn none_ran(&self) -> bool {
        self.cases.iter().all(|c| c.passed.is_none())
    }
}

/// Find a backend's toolchain or explain how to get it.
pub fn require_tool(backend: &dyn Backend) -> Result<Found> {
    match backend.tool().find() {
        Some(found) => Ok(found),
        None => bail!(
            "{} isn't installed (or isn't on PATH), so the {} tests can't run.\n\
             Run `learn doctor` to see how to install it.",
            backend.tool().label,
            backend.name()
        ),
    }
}

pub fn run(backend: &dyn Backend, spec: &Spec, all_specs: &[Spec], track_dir: &Path, tool: &Found) -> Result<CheckResult> {
    run_streaming(backend, spec, all_specs, track_dir, tool, &mut |_, _| {})
}

/// Regenerate the exercise's tests (cheap, keeps them in sync with the spec)
/// and run them. `on_case` is called as soon as each case's result appears.
pub fn run_streaming(
    backend: &dyn Backend,
    spec: &Spec,
    all_specs: &[Spec],
    track_dir: &Path,
    tool: &Found,
    on_case: &mut dyn FnMut(usize, bool),
) -> Result<CheckResult> {
    backends::generate(backend, all_specs, track_dir)?;
    let inv = backend.command(spec, tool, track_dir);
    let out = toolchain::run_streaming(&inv, &mut |line| {
        for (index, passed) in backend.parse_results(line) {
            on_case(index, passed);
        }
    })?;
    let parsed = backend.parse_results(&out.output);

    let mut cases: Vec<CaseResult> = spec
        .cases()
        .into_iter()
        .map(|(_, c)| CaseResult { index: c.index, description: c.description.clone(), passed: None, message: None })
        .collect();
    for (index, passed) in parsed {
        if let Some(c) = cases.get_mut(index) {
            // A case reported twice (e.g. a summary line) only counts as passed if every report passed.
            c.passed = Some(c.passed.unwrap_or(true) && passed);
        }
    }
    for c in cases.iter_mut().filter(|c| c.passed == Some(false)) {
        c.message = backend.failure_reason(&out.output, &crate::naming::case_name(c.index, &c.description));
    }
    let passed = out.success && !out.timed_out && cases.iter().all(|c| c.passed == Some(true));
    let setup_problem = if cases.iter().all(|c| c.passed.is_none()) { backend.setup_problem(&out.output) } else { None };
    Ok(CheckResult { cases, passed, timed_out: out.timed_out, command: inv.display(), output: out.output, setup_problem })
}

/// Mark an exercise done on a track after a passing check.
pub fn record_pass(ctx: &mut Ctx, track: &str, slug: &str) -> Result<()> {
    ctx.progress.done.entry(track.to_string()).or_default().insert(slug.to_string());
    ctx.save_progress()
}
