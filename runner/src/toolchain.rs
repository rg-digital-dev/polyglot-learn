//! Finding toolchains (from PATH names and absolute candidate paths) and
//! running them with a timeout.

use std::io::Read;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::error::{Context, Result};

pub struct Tool {
    /// Short name shown to the learner.
    pub label: &'static str,
    /// PATH names or absolute paths. `~` and `*` (within one path segment) are expanded.
    pub candidates: &'static [&'static str],
    /// Arguments that make the tool print its version; empty to skip.
    pub version_args: &'static [&'static str],
    /// When several candidates exist, prefer one whose version output contains this.
    pub prefer: Option<&'static str>,
}

#[derive(Debug, Clone)]
pub struct Found {
    pub path: PathBuf,
    pub version: String,
}

impl Tool {
    pub fn find(&self) -> Option<Found> {
        let mut first = None;
        let mut seen = Vec::new();
        for cand in self.candidates {
            for path in resolve(cand) {
                if seen.contains(&path) {
                    continue;
                }
                seen.push(path.clone());
                let version = version_of(&path, self.version_args);
                let found = Found { path, version };
                match self.prefer {
                    None => return Some(found),
                    Some(p) if found.version.contains(p) => return Some(found),
                    Some(_) => {
                        if first.is_none() {
                            first = Some(found);
                        }
                    }
                }
            }
        }
        first
    }
}

fn version_of(path: &Path, args: &[&str]) -> String {
    if args.is_empty() {
        return String::new();
    }
    let inv = Invocation::new(path, std::env::temp_dir()).args(args.iter().copied()).timeout(Duration::from_secs(10));
    match run(&inv) {
        Ok(out) => out.output.lines().map(str::trim).find(|l| !l.is_empty()).unwrap_or("").to_string(),
        Err(_) => String::new(),
    }
}

/// Expand one candidate into existing executable paths.
fn resolve(cand: &str) -> Vec<PathBuf> {
    if !cand.contains('/') && !cand.contains('\\') {
        return which(cand).into_iter().collect();
    }
    let expanded = if let Some(rest) = cand.strip_prefix("~/") {
        match home_dir() {
            Some(h) => h.join(rest),
            None => return Vec::new(),
        }
    } else {
        PathBuf::from(cand)
    };
    glob(&expanded).into_iter().filter(|p| p.is_file()).collect()
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(PathBuf::from)
}

/// Search PATH (and PATHEXT on Windows) for an executable name.
pub fn which(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    let exts: Vec<String> = if cfg!(windows) {
        let pathext = std::env::var("PATHEXT").unwrap_or_else(|_| ".EXE;.CMD;.BAT".into());
        let mut v = vec![String::new()];
        v.extend(pathext.split(';').filter(|e| !e.is_empty()).map(|e| e.to_ascii_lowercase()));
        v
    } else {
        vec![String::new()]
    };
    for dir in std::env::split_paths(&path) {
        for ext in &exts {
            let p = dir.join(format!("{name}{ext}"));
            // On Windows only real executables: an extension-less file named `lua` isn't runnable.
            if p.is_file() && (!cfg!(windows) || !ext.is_empty()) {
                return Some(p);
            }
        }
    }
    None
}

/// Minimal glob: `*` may appear inside path segments. Matches are sorted so the
/// highest version comes first (`6.10.3` before `6.9.0`).
fn glob(pattern: &Path) -> Vec<PathBuf> {
    let mut current = vec![PathBuf::new()];
    for comp in pattern.components() {
        let part = comp.as_os_str().to_string_lossy().to_string();
        let mut next = Vec::new();
        for base in &current {
            if part.contains('*') {
                let Ok(entries) = std::fs::read_dir(if base.as_os_str().is_empty() { Path::new(".") } else { base })
                else {
                    continue;
                };
                let mut names: Vec<String> = entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .filter(|n| wildcard_match(&part, n))
                    .collect();
                names.sort_by(|a, b| natural_cmp(b, a));
                next.extend(names.into_iter().map(|n| base.join(n)));
            } else {
                next.push(base.join(comp.as_os_str()));
            }
        }
        current = next;
    }
    current
}

fn wildcard_match(pattern: &str, name: &str) -> bool {
    let parts: Vec<&str> = pattern.split('*').collect();
    let mut rest = name;
    for (i, part) in parts.iter().enumerate() {
        if i == 0 {
            match rest.strip_prefix(part) {
                Some(r) => rest = r,
                None => return false,
            }
        } else if i == parts.len() - 1 {
            return rest.ends_with(part);
        } else {
            match rest.find(part) {
                Some(pos) => rest = &rest[pos + part.len()..],
                None => return false,
            }
        }
    }
    rest.is_empty()
}

/// Compare strings treating digit runs as numbers.
pub fn natural_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    fn chunks(s: &str) -> Vec<(bool, String)> {
        let mut out: Vec<(bool, String)> = Vec::new();
        for ch in s.chars() {
            let digit = ch.is_ascii_digit();
            match out.last_mut() {
                Some((d, buf)) if *d == digit => buf.push(ch),
                _ => out.push((digit, ch.to_string())),
            }
        }
        out
    }
    let (ca, cb) = (chunks(a), chunks(b));
    for (x, y) in ca.iter().zip(&cb) {
        let ord = if x.0 && y.0 {
            x.1.parse::<u64>().unwrap_or(0).cmp(&y.1.parse::<u64>().unwrap_or(0))
        } else {
            x.1.cmp(&y.1)
        };
        if ord != std::cmp::Ordering::Equal {
            return ord;
        }
    }
    ca.len().cmp(&cb.len())
}

/// Parse the first `major.minor[.patch]` found in a version string.
pub fn parse_version(text: &str) -> Option<(u64, u64, u64)> {
    let start = text.find(|c: char| c.is_ascii_digit())?;
    let rest = &text[start..];
    let end = rest.find(|c: char| !(c.is_ascii_digit() || c == '.')).unwrap_or(rest.len());
    let mut nums = rest[..end].split('.').filter_map(|n| n.parse::<u64>().ok());
    Some((nums.next()?, nums.next().unwrap_or(0), nums.next().unwrap_or(0)))
}

pub struct Invocation {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub env: Vec<(String, String)>,
    pub timeout: Duration,
    /// A file the tool writes its report to; appended to the output and removed.
    pub output_file: Option<PathBuf>,
}

impl Invocation {
    pub fn new(program: impl Into<PathBuf>, cwd: impl Into<PathBuf>) -> Self {
        Invocation {
            program: program.into(),
            args: Vec::new(),
            cwd: cwd.into(),
            env: Vec::new(),
            timeout: Duration::from_secs(120),
            output_file: None,
        }
    }

    pub fn output_file(mut self, path: PathBuf) -> Self {
        self.output_file = Some(path);
        self
    }

    pub fn args<I: IntoIterator<Item = S>, S: Into<String>>(mut self, args: I) -> Self {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    pub fn env(mut self, key: &str, value: &str) -> Self {
        self.env.push((key.into(), value.into()));
        self
    }

    pub fn timeout(mut self, t: Duration) -> Self {
        self.timeout = t;
        self
    }

    pub fn display(&self) -> String {
        let prog = self.program.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
        std::iter::once(prog).chain(self.args.iter().cloned()).collect::<Vec<_>>().join(" ")
    }
}

pub struct RunOutput {
    pub success: bool,
    pub timed_out: bool,
    /// stdout followed by stderr.
    pub output: String,
}

/// Set when `learn serve` shuts down: running test processes are killed.
static CANCEL: AtomicBool = AtomicBool::new(false);

pub fn cancel_all() {
    CANCEL.store(true, Ordering::SeqCst);
}

pub fn cancelled() -> bool {
    CANCEL.load(Ordering::SeqCst)
}

pub fn run(inv: &Invocation) -> Result<RunOutput> {
    run_streaming(inv, &mut |_| {})
}

/// Run a tool, calling `on_line` for every output line as it arrives (stdout
/// and stderr), so results can be shown before the process ends.
pub fn run_streaming(inv: &Invocation, on_line: &mut dyn FnMut(&str)) -> Result<RunOutput> {
    let mut cmd = Command::new(&inv.program);
    cmd.args(&inv.args)
        .current_dir(&inv.cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (k, v) in &inv.env {
        cmd.env(k, v);
    }
    let mut child = cmd.spawn().context(|| format!("couldn't start {}", inv.program.display()))?;

    // Each reader keeps its own full copy and forwards lines as they arrive.
    let (tx, rx) = mpsc::channel::<String>();
    let reader = |mut pipe: Box<dyn Read + Send>, tx: mpsc::Sender<String>| {
        std::thread::spawn(move || {
            let mut all = Vec::new();
            let mut line = Vec::new();
            let mut buf = [0u8; 4096];
            loop {
                let n = match pipe.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => n,
                };
                all.extend_from_slice(&buf[..n]);
                for &b in &buf[..n] {
                    if b == b'\n' {
                        let _ = tx.send(String::from_utf8_lossy(&line).trim_end_matches('\r').to_string());
                        line.clear();
                    } else {
                        line.push(b);
                    }
                }
            }
            if !line.is_empty() {
                let _ = tx.send(String::from_utf8_lossy(&line).to_string());
            }
            all
        })
    };
    let out_thread = reader(Box::new(child.stdout.take().expect("piped stdout")), tx.clone());
    let err_thread = reader(Box::new(child.stderr.take().expect("piped stderr")), tx);

    let deadline = Instant::now() + inv.timeout;
    let mut timed_out = false;
    let status = loop {
        while let Ok(line) = rx.try_recv() {
            on_line(&line);
        }
        if let Some(status) = child.try_wait().context(|| "lost track of a test process".into())? {
            break Some(status);
        }
        if Instant::now() >= deadline || cancelled() {
            let _ = child.kill();
            let _ = child.wait();
            timed_out = !cancelled();
            break None;
        }
        if let Ok(line) = rx.recv_timeout(Duration::from_millis(20)) {
            on_line(&line);
        }
    };

    let mut output = String::from_utf8_lossy(&out_thread.join().unwrap_or_default()).to_string();
    let err = String::from_utf8_lossy(&err_thread.join().unwrap_or_default()).to_string();
    // Readers are done, so every remaining line is already in the channel.
    for line in rx.try_iter() {
        on_line(&line);
    }
    if !err.is_empty() {
        if !output.is_empty() && !output.ends_with('\n') {
            output.push('\n');
        }
        output.push_str(&err);
    }
    if let Some(path) = &inv.output_file {
        // Tools that only write a report file (qmltestrunner on Windows) report at the end.
        if let Ok(report) = std::fs::read_to_string(path) {
            for line in report.lines() {
                on_line(line);
            }
            if !output.is_empty() && !output.ends_with('\n') {
                output.push('\n');
            }
            output.push_str(&report);
        }
        let _ = std::fs::remove_file(path);
    }
    Ok(RunOutput { success: status.is_some_and(|s| s.success()), timed_out, output })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wildcards() {
        assert!(wildcard_match("6.*", "6.10.3"));
        assert!(wildcard_match("*", "anything"));
        assert!(wildcard_match("msvc*_64", "msvc2022_64"));
        assert!(!wildcard_match("msvc*_64", "mingw_64"));
        assert!(!wildcard_match("6.*", "5.15.2"));
    }

    #[test]
    fn natural_order() {
        let mut v = vec!["6.9.0", "6.10.3", "6.2.4"];
        v.sort_by(|a, b| natural_cmp(b, a));
        assert_eq!(v, ["6.10.3", "6.9.0", "6.2.4"]);
    }

    #[test]
    fn versions() {
        assert_eq!(parse_version("v22.18.0"), Some((22, 18, 0)));
        assert_eq!(parse_version("Lua 5.4.6  Copyright"), Some((5, 4, 6)));
        assert_eq!(parse_version("go version go1.23.1 windows/amd64"), Some((1, 23, 1)));
        assert_eq!(parse_version("none"), None);
    }
}
