//! One backend per language. A backend turns a language-neutral spec into
//! native stubs and native tests, knows how to run those tests, and how to read
//! per-case results back from the test output.

use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::error::{Context, Result};
use crate::spec::{concrete_def, Case, Function, Spec, Type, TypeDef};
use crate::toolchain::{Found, Invocation, Tool};

mod go;
mod lua;
mod php;
mod qml;
mod rust;
mod typescript;

pub trait Backend: Sync {
    /// Track id, also the folder name under `tracks/` and `workspace/`.
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    /// Fenced-code language tag for this track's snippets.
    #[allow(dead_code)] // used by the web UI
    fn fence(&self) -> &'static str;
    fn tool(&self) -> &'static Tool;

    /// Function name in this language's convention.
    fn function_name(&self, f: &Function) -> String;
    /// Native type for a spec type. `param` selects borrowed forms where the
    /// language distinguishes them (Rust `&str` vs `String`).
    fn type_name(&self, spec: &Spec, ty: &Type, param: bool) -> String;
    /// A typed literal for `value`, as test code would write it.
    fn literal(&self, spec: &Spec, value: &Value, ty: &Type) -> String;
    /// A function's signature as it appears in the stub.
    fn signature(&self, spec: &Spec, f: &Function) -> String;

    /// The main file the learner edits, relative to the track folder.
    fn stub_path(&self, spec: &Spec) -> PathBuf;
    /// The file for a helper module, relative to the track folder.
    fn module_path(&self, spec: &Spec, module: &str) -> PathBuf;
    /// Every learner-owned file: the main stub and one per helper module.
    /// Each is written only when missing.
    fn stubs(&self, spec: &Spec) -> Vec<(PathBuf, String)>;
    /// Generated test files (relative path, contents). Always safe to overwrite.
    fn tests(&self, spec: &Spec) -> Vec<(PathBuf, String)>;
    /// Files the track folder needs (workspace manifests, harnesses). Always overwritten.
    fn shared(&self, specs: &[Spec]) -> Vec<(PathBuf, String)>;
    /// Per-exercise files the learner may customise (e.g. a crate manifest).
    /// Written only when missing, like the stubs.
    fn scaffold(&self, _spec: &Spec) -> Vec<(PathBuf, String)> {
        Vec::new()
    }

    /// How to run one exercise's tests from the track folder.
    fn command(&self, spec: &Spec, tool: &Found, track_dir: &Path) -> Invocation;
    /// Per-case results found in the output: (0-based case index, passed).
    fn parse_results(&self, output: &str) -> Vec<(usize, bool)>;
    /// The native failure message for one failing test, if it can be found.
    fn failure_reason(&self, output: &str, test_name: &str) -> Option<String> {
        reason_after_marker(output, test_name)
    }

    /// A short warning about the found toolchain (old version etc.), if any.
    fn check_tool(&self, _tool: &Found) -> Option<String> {
        None
    }
}

static BACKENDS: &[&dyn Backend] = &[
    &typescript::TypeScript,
    &rust::Rust,
    &go::Go,
    &php::Php,
    &lua::Lua,
    &qml::Qml,
];

pub fn all() -> &'static [&'static dyn Backend] {
    BACKENDS
}

pub fn ids() -> Vec<&'static str> {
    BACKENDS.iter().map(|b| b.id()).collect()
}

pub fn get(id: &str) -> Result<&'static dyn Backend> {
    let id = match id.to_ascii_lowercase().as_str() {
        "ts" | "node" => "typescript".to_string(),
        "rs" => "rust".to_string(),
        "golang" => "go".to_string(),
        "qt" | "quickshell" => "qml".to_string(),
        other => other.to_string(),
    };
    BACKENDS.iter().copied().find(|b| b.id() == id).ok_or_else(|| {
        crate::error::Error(format!("unknown track `{id}`. Tracks: {}", ids().join(", ")))
    })
}

/// What happened when generating a track.
#[derive(Default)]
pub struct Generated {
    pub stubs_created: Vec<PathBuf>,
    pub stubs_kept: Vec<PathBuf>,
}

/// Write shared files and tests for every spec, and stubs only where none exist.
pub fn generate(backend: &dyn Backend, specs: &[Spec], track_dir: &Path) -> Result<Generated> {
    let mut report = Generated::default();
    for (rel, contents) in backend.shared(specs) {
        write(&track_dir.join(rel), &contents)?;
    }
    for spec in specs {
        for (rel, contents) in backend.tests(spec) {
            write(&track_dir.join(rel), &contents)?;
        }
        for (rel, contents) in backend.scaffold(spec) {
            let path = track_dir.join(rel);
            if !path.exists() {
                write(&path, &contents)?;
            }
        }
        for (rel, contents) in backend.stubs(spec) {
            let path = track_dir.join(rel);
            if path.exists() {
                report.stubs_kept.push(path);
            } else {
                write(&path, &contents)?;
                report.stubs_created.push(path);
            }
        }
    }
    Ok(report)
}

fn write(path: &Path, contents: &str) -> Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).context(|| format!("can't create {}", dir.display()))?;
    }
    // Skip identical writes so file watchers (and editors) don't see churn.
    if std::fs::read_to_string(path).is_ok_and(|old| old == contents) {
        return Ok(());
    }
    std::fs::write(path, contents).context(|| format!("can't write {}", path.display()))
}

// ---- helpers shared by backends -------------------------------------------

/// Message every stub uses when it is not implemented yet. Harnesses for
/// languages that signal errors by throwing treat it as a failure even in
/// error-expecting cases, so an untouched stub fails every case.
pub fn stub_message(name: &str) -> String {
    format!("TODO: implement {name}")
}

/// Escape a string for C-like double-quoted literals (Rust, Go, PHP, Lua).
/// `hex` formats other control characters (e.g. `\x{:02x}` or Lua's `\{:03}`).
pub fn escape(s: &str, hex: fn(u32) -> String, extra: &[(char, &str)]) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        if let Some((_, rep)) = extra.iter().find(|(c, _)| *c == ch) {
            out.push_str(rep);
            continue;
        }
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c if (c as u32) < 0x20 || c as u32 == 0x7f => out.push_str(&hex(c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// Word-wrap text into lines of at most `width` characters.
pub fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut cur = String::new();
    for word in text.split_whitespace() {
        if !cur.is_empty() && cur.len() + 1 + word.len() > width {
            lines.push(std::mem::take(&mut cur));
        }
        if !cur.is_empty() {
            cur.push(' ');
        }
        cur.push_str(word);
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    lines
}

/// Doc comment lines with a given prefix (`/// `, `// `, `--- `).
pub fn doc(prefix: &str, text: &str) -> String {
    wrap(text, 76).iter().map(|l| format!("{prefix}{l}\n")).collect()
}

pub fn header(comment: &str, spec: &Spec, edit_instead: &str) -> String {
    format!(
        "{comment} Generated from specs/{}.json by `learn generate`. Don't edit this file:\n\
         {comment} it is overwritten whenever the spec changes. Your code goes in {edit_instead}.\n",
        spec.slug
    )
}

pub fn hex_escape(c: u32) -> String {
    format!("\\x{c:02x}")
}

/// Render a value as compact JSON (used in messages and JS-family literals).
pub fn json(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_default()
}

/// Render a list of argument values against a parameter list.
pub fn render_args(params: &[crate::spec::Param], input: &[Value], render: impl Fn(&Value, &Type) -> String) -> String {
    input.iter().zip(params).map(|(v, p)| render(v, &p.ty)).collect::<Vec<_>>().join(", ")
}

/// The record/class a named-type value is (resolving interfaces via `"type"`).
pub fn def_of<'a>(spec: &'a Spec, name: &str, value: &Value) -> &'a TypeDef {
    concrete_def(name, value, &spec.types).expect("spec values are validated against their types")
}

/// Field values of a record/class value, in declaration order.
pub fn field_values<'a>(def: &'a TypeDef, value: &'a Value) -> Vec<(&'a crate::spec::Param, &'a Value)> {
    def.fields.iter().map(|f| (f, value.get(&f.name).expect("validated"))).collect()
}

/// Timing bounds for concurrency cases with real sleeps, in milliseconds.
/// Generous on the upper side so busy CI machines don't flake.
pub struct Timing {
    pub max: i64,
    pub sum: i64,
    pub min_ms: i64,
    pub limit_ms: i64,
}

pub fn timing(case_input: &[Value]) -> Timing {
    let delays: Vec<i64> = case_input
        .first()
        .and_then(Value::as_array)
        .map(|a| a.iter().filter_map(Value::as_i64).collect())
        .unwrap_or_default();
    let max = delays.iter().copied().max().unwrap_or(0);
    let sum = delays.iter().sum();
    Timing { max, sum, min_ms: (max - 30).max(0), limit_ms: max + 200 }
}

/// Description of a case for failure messages ("call 2 on counter 0", etc.).
pub fn step_label(step_number: usize, on: usize, values: usize) -> String {
    if values > 1 {
        format!("call {step_number} on value {on}")
    } else {
        format!("call {step_number}")
    }
}

/// Results from TAP-like lines: `ok … cNN_…` / `not ok … cNN_…`.
pub fn parse_tap(output: &str) -> Vec<(usize, bool)> {
    let mut out = Vec::new();
    for line in output.lines() {
        let t = line.trim_start();
        let passed = if t.starts_with("not ok") {
            false
        } else if t.starts_with("ok") {
            true
        } else {
            continue;
        };
        if let Some(i) = crate::naming::find_case_index(t) {
            out.push((i, passed));
        }
    }
    out
}

/// Default failure reason: our own harnesses print `not ok <name> - <why>`.
pub fn reason_after_marker(output: &str, test_name: &str) -> Option<String> {
    let prefix = format!("not ok {test_name} - ");
    output.lines().find_map(|l| l.trim().strip_prefix(&prefix).map(|r| r.trim().to_string()))
}

/// Lines following the first line that matches `start`, up to (not including)
/// a line that matches `stop`; at most `limit` lines, trimmed, blank ones skipped.
pub fn lines_after(
    output: &str,
    start: impl Fn(&str) -> bool,
    stop: impl Fn(&str) -> bool,
    limit: usize,
) -> Option<String> {
    let mut lines = output.lines().skip_while(|l| !start(l));
    lines.next()?;
    let picked: Vec<&str> =
        lines.take_while(|l| !stop(l)).map(str::trim_end).filter(|l| !l.trim().is_empty()).take(limit).collect();
    (!picked.is_empty()).then(|| picked.join("\n"))
}

/// Test name for a case.
pub fn case_name(case: &Case) -> String {
    crate::naming::case_name(case.index, &case.description)
}

#[cfg(test)]
mod tests;
