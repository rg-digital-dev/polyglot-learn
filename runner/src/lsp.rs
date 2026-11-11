//! `learn lsp`: a small language server for editors. It runs next to each
//! language's own server and only answers for files inside a learn workspace:
//!
//! - on save it runs the exercise's tests and shows failing cases on the
//!   function's signature line (a compile error as one error);
//! - hovering the signature shows the task, what the tests check and the hints
//!   revealed so far;
//! - code actions check the exercise, reveal the next hint, or open LESSON.md.
//!
//! Messages are plain JSON (`serde_json`) over `lsp-server`'s stdio transport.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};

use crossbeam_channel::Sender;
use lsp_server::{Connection, ErrorCode, Message, Notification, Request, RequestId, Response};
use serde_json::{json, Value};

use crate::backends::{self, Backend};
use crate::check::{self, CheckResult};
use crate::ctx::Ctx;
use crate::error::{Error, Result};
use crate::lessons;
use crate::spec::Spec;
use crate::ui::Ui;
use crate::view;
use crate::views;

pub const CHECK: &str = "learn.check";
pub const HINT: &str = "learn.hint";
pub const LESSON: &str = "learn.lesson";
/// Code action: open the next unfinished exercise on the path.
pub const NEXT: &str = "learn.next";

const ERROR: u8 = 1;
const WARNING: u8 = 2;
const INFO: u8 = 3;

pub fn run(workspace: Option<PathBuf>) -> Result<ExitCode> {
    let (conn, io) = Connection::stdio();
    serve(&conn, workspace)?;
    drop(conn);
    io.join().map_err(|e| Error(format!("learn lsp: {e}")))?;
    Ok(ExitCode::SUCCESS)
}

fn protocol(e: impl std::fmt::Display) -> Error {
    Error(format!("learn lsp: {e}"))
}

/// Answer one client until it shuts down.
pub fn serve(conn: &Connection, workspace: Option<PathBuf>) -> Result<()> {
    let caps = json!({
        "textDocumentSync": { "openClose": false, "change": 0, "save": { "includeText": false } },
        "hoverProvider": true,
        "codeActionProvider": true,
        "executeCommandProvider": { "commands": [CHECK, HINT, LESSON, NEXT] },
    });
    let params = conn.initialize(caps).map_err(protocol)?;
    let show_document = params.pointer("/capabilities/window/showDocument/support").and_then(Value::as_bool).unwrap_or(false);
    let server = Server { out: conn.sender.clone(), workspace, show_document, next_id: Arc::new(AtomicI32::new(1)), running: Arc::new(Mutex::new(())) };
    for msg in &conn.receiver {
        match msg {
            Message::Request(req) => {
                if conn.handle_shutdown(&req).map_err(protocol)? {
                    return Ok(());
                }
                server.request(req);
            }
            Message::Notification(n) => server.notification(n),
            Message::Response(_) => {}
        }
    }
    Ok(())
}

/// The exercise a file belongs to.
struct Target {
    ctx: Ctx,
    track: &'static dyn Backend,
    slug: String,
}

impl Target {
    fn spec(&self) -> &Spec {
        self.ctx.spec_on(self.track.id(), &self.slug).expect("checked in locate")
    }

    fn track_dir(&self) -> PathBuf {
        self.ctx.workspace.join(self.track.id())
    }

    /// The learner's files with their signatures; the main stub last.
    fn files(&self) -> Vec<(PathBuf, Vec<String>)> {
        backends::learner_files(self.track, self.spec()).into_iter().map(|(rel, sigs)| (self.track_dir().join(rel), sigs)).collect()
    }
}

#[derive(Clone)]
struct Server {
    out: Sender<Message>,
    workspace: Option<PathBuf>,
    show_document: bool,
    next_id: Arc<AtomicI32>,
    /// One test run at a time: two runs in one folder would trip over each other.
    running: Arc<Mutex<()>>,
}

impl Server {
    fn send(&self, msg: impl Into<Message>) {
        let _ = self.out.send(msg.into());
    }

    fn notify(&self, method: &str, params: Value) {
        self.send(Notification::new(method.to_string(), params));
    }

    fn show_message(&self, kind: u8, text: &str) {
        self.notify("window/showMessage", json!({ "type": kind, "message": text }));
    }

    /// Which exercise `uri` is in, if it's in a learn workspace at all.
    fn locate(&self, uri: &str) -> Option<Target> {
        let file = uri_to_path(uri)?;
        let ctx = Ctx::discover_from(file.parent()?, self.workspace.clone()).ok()?;
        let rel = file.strip_prefix(&ctx.workspace).ok()?;
        let track = rel.components().next()?.as_os_str().to_string_lossy().to_string();
        let b = backends::get(&track).ok()?;
        if b.id() != track {
            return None;
        }
        // A file inside an exercise's folder (`NN-stage/slug/…`), not the folder itself.
        let slug = ctx.exercise_at(b.id(), rel.strip_prefix(&track).ok()?)?.slug.clone();
        Some(Target { ctx, track: b, slug })
    }

    fn notification(&self, n: Notification) {
        if n.method == "textDocument/didSave" {
            if let Some(t) = n.params.pointer("/textDocument/uri").and_then(Value::as_str).and_then(|u| self.locate(u)) {
                self.check(t);
            }
        }
    }

    fn request(&self, req: Request) {
        let uri = req.params.pointer("/textDocument/uri").or_else(|| req.params.pointer("/arguments/0")).and_then(Value::as_str);
        let target = uri.and_then(|u| self.locate(u));
        let result = match req.method.as_str() {
            "textDocument/hover" => {
                let line = req.params.pointer("/position/line").and_then(Value::as_u64).unwrap_or(0) as usize;
                target.and_then(|t| hover(&t, &uri_to_path(uri?)?, line)).unwrap_or(Value::Null)
            }
            "textDocument/codeAction" => match target {
                Some(_) => code_actions(uri.unwrap_or_default()),
                None => Value::Array(Vec::new()),
            },
            "workspace/executeCommand" => {
                let command = req.params.get("command").and_then(Value::as_str).unwrap_or_default();
                match (command, target) {
                    (CHECK, Some(t)) => self.check(t),
                    (HINT, Some(t)) => self.hint(t),
                    (LESSON, Some(t)) => self.lesson(t),
                    (NEXT, Some(t)) => self.next(t),
                    _ => self.show_message(WARNING, "That isn't a learn exercise file."),
                }
                Value::Null
            }
            _ => {
                self.send(Response::new_err(req.id, ErrorCode::MethodNotFound as i32, format!("learn lsp doesn't handle {}", req.method)));
                return;
            }
        };
        self.send(Response::new_ok(req.id, result));
    }

    /// Run the tests on a thread and publish what failed.
    fn check(&self, mut t: Target) {
        let server = self.clone();
        std::thread::spawn(move || {
            let _guard = server.running.lock().unwrap_or_else(|p| p.into_inner());
            let outcome = (|| -> Result<CheckResult> {
                let tool = check::require_tool(t.track)?;
                let spec = t.spec().clone();
                let result = check::run(t.track, &spec, &t.ctx.workspace_specs(t.track.id()), &t.track_dir(), &tool)?;
                if result.passed {
                    check::record_pass(&mut t.ctx, t.track.id(), &t.slug)?;
                }
                Ok(result)
            })();
            match outcome {
                Err(e) => server.show_message(ERROR, &e.0),
                Ok(result) => {
                    let files = t.files();
                    let main = files.len().saturating_sub(1);
                    for (i, (file, sigs)) in files.iter().enumerate() {
                        let text = std::fs::read_to_string(file).unwrap_or_default();
                        let diags = if i == main { diagnostics(t.spec(), &result, &text, sigs, &t.track_dir()) } else { Vec::new() };
                        server.notify("textDocument/publishDiagnostics", json!({ "uri": path_to_uri(file), "diagnostics": diags }));
                    }
                    if result.passed {
                        let next = t.ctx.current(t.track.id()).map(|s| format!(" Next: {}.", s.title)).unwrap_or_default();
                        server.show_message(INFO, &format!("{}: all {} passing.{next}", t.spec().title, result.cases.len()));
                    }
                }
            }
        });
    }

    /// Reveal the next hint, as `learn hint` does.
    fn hint(&self, mut t: Target) {
        let spec = t.spec().clone();
        let total = spec.hints.len();
        if total == 0 {
            self.show_message(INFO, &format!("{} has no hints. LESSON.md has a worked example.", spec.title));
            return;
        }
        let shown = t.ctx.progress.hints.entry(t.slug.clone()).or_insert(0);
        let was_all = *shown >= total;
        *shown = (*shown + 1).min(total);
        let shown = *shown;
        if let Err(e) = t.ctx.save_progress() {
            self.show_message(ERROR, &e.0);
            return;
        }
        let text = if was_all { format!("That's every hint. The last one: {}", spec.hints[total - 1]) } else { format!("Hint {shown} of {total}: {}", spec.hints[shown - 1]) };
        self.show_message(INFO, &text);
    }

    /// Open LESSON.md in the editor, or say where it is.
    /// Open the next unfinished exercise after this one (or the first one
    /// left, if everything after it is done).
    fn next(&self, t: Target) {
        let track = t.track.id();
        let specs = t.ctx.path_specs(track);
        let here = specs.iter().position(|s| s.slug == t.slug).unwrap_or(0);
        let open = |s: &&&learn_core::spec::Spec| !t.ctx.is_done(track, &s.slug) && s.slug != t.slug;
        let Some(next) = specs.iter().skip(here + 1).find(open).or_else(|| specs.iter().find(open)).copied() else {
            self.show_message(INFO, "That's every exercise on your path done. Well done!");
            return;
        };
        let path = t.track_dir().join(backends::main_file(t.track, next));
        if !path.exists() {
            if let Err(e) = lessons::generate_track(&t.ctx, t.track) {
                self.show_message(ERROR, &e.0);
                return;
            }
        }
        if self.show_document {
            let id = RequestId::from(self.next_id.fetch_add(1, Ordering::SeqCst));
            self.send(Request::new(id, "window/showDocument".into(), json!({ "uri": path_to_uri(&path), "takeFocus": true })));
        } else {
            self.show_message(INFO, &format!("Next: {}, in {}", next.title, path.display()));
        }
    }

    fn lesson(&self, t: Target) {
        let path = t.track_dir().join(lessons::path(t.track.id(), t.spec()));
        if !path.exists() {
            if let Err(e) = lessons::generate_track(&t.ctx, t.track) {
                self.show_message(ERROR, &e.0);
                return;
            }
        }
        if self.show_document {
            let id = RequestId::from(self.next_id.fetch_add(1, Ordering::SeqCst));
            self.send(Request::new(id, "window/showDocument".into(), json!({ "uri": path_to_uri(&path), "takeFocus": true })));
        } else {
            self.show_message(INFO, &format!("The lesson is in {}", path.display()));
        }
    }
}

fn code_actions(uri: &str) -> Value {
    let action = |title: &str, command: &str| json!({ "title": title, "command": { "title": title, "command": command, "arguments": [uri] } });
    json!([
        action("Check this exercise", CHECK),
        action("Show next hint", HINT),
        action("Open LESSON.md", LESSON),
        action("Open next exercise", NEXT),
    ])
}

/// Hovering a signature line: the task, what the tests check, hints so far.
fn hover(t: &Target, file: &Path, line: usize) -> Option<Value> {
    let (_, sigs) = t.files().into_iter().find(|(f, _)| same_file(f, file))?;
    let text = std::fs::read_to_string(file).ok()?;
    let anchors: Vec<usize> = sigs.iter().filter_map(|s| anchor(&text, s)).map(|(l, _, _)| l).collect();
    if !anchors.contains(&line) {
        return None;
    }
    let spec = t.spec();
    let mut md = vec![format!("**{}** · {} · {}", spec.title, spec.concept, t.track.name()), spec.main.summary.clone()];
    if t.ctx.is_done(t.track.id(), &spec.slug) {
        md.push("✓ Passed".into());
    }
    let cases: Vec<String> = views::cases(spec)
        .iter()
        .map(|c| match c.lines.first() {
            Some(l) => format!("- {}: `{l}`", c.description),
            None => format!("- {}", c.description),
        })
        .collect();
    md.push(format!("**The tests check**\n\n{}", cases.join("\n")));
    let shown = t.ctx.progress.hints.get(&spec.slug).copied().unwrap_or(0).min(spec.hints.len());
    if shown > 0 {
        let hints: Vec<String> = spec.hints.iter().take(shown).enumerate().map(|(i, h)| format!("{}. {h}", i + 1)).collect();
        md.push(format!("**Hints so far**\n\n{}", hints.join("\n")));
    }
    if shown < spec.hints.len() {
        md.push(format!("_Stuck? The \"Show next hint\" code action has {} more._", spec.hints.len() - shown));
    }
    Some(json!({ "contents": { "kind": "markdown", "value": md.join("\n\n") } }))
}

/// Where a signature sits in the file: (line, first column, end column), in
/// characters. Matches the text before `(` (`func Greet`), then the bare name.
fn anchor(text: &str, signature: &str) -> Option<(usize, usize, usize)> {
    let head = signature.split('(').next().unwrap_or(signature).trim();
    let name = head.split_whitespace().last().unwrap_or(head);
    let find = |needle: &str| {
        text.lines().enumerate().find(|(_, l)| !needle.is_empty() && l.contains(needle)).map(|(i, l)| {
            let start = l.chars().take_while(|c| c.is_whitespace()).count();
            (i, start, l.chars().count())
        })
    };
    find(head).or_else(|| find(name))
}

/// The check's result as diagnostics on the main file.
fn diagnostics(spec: &Spec, result: &CheckResult, text: &str, sigs: &[String], track_dir: &Path) -> Vec<Value> {
    if result.passed {
        return Vec::new();
    }
    let (line, start, end) = sigs.iter().find_map(|s| anchor(text, s)).unwrap_or((0, 0, text.lines().next().map_or(0, |l| l.chars().count())));
    let diag = |severity: u8, message: String| {
        json!({
            "range": { "start": { "line": line, "character": start }, "end": { "line": line, "character": end } },
            "severity": severity,
            "source": "learn",
            "message": message,
        })
    };
    if result.timed_out {
        return vec![diag(WARNING, format!("The tests stopped after {} s: look for an infinite loop.", result.timeout_secs))];
    }
    if result.none_ran() {
        let message = match &result.setup_problem {
            Some(p) => p.clone(),
            None => {
                let (lines, _) = view::first_error(&result.output);
                let lines: Vec<String> = lines.iter().map(|l| view::shorten_paths(l, track_dir)).collect();
                format!("No tests ran:\n{}", lines.join("\n"))
            }
        };
        return vec![diag(ERROR, message)];
    }
    let ui = Ui { unicode: false, ..Ui::plain(200) };
    result
        .cases
        .iter()
        .filter(|c| c.passed == Some(false))
        .map(|c| {
            let detail: Vec<String> = view::failure_detail(spec, c, false, &ui).iter().skip(1).map(|l| l.trim().to_string()).collect();
            let mut message = format!("case {}, {}", c.index + 1, c.description);
            if !detail.is_empty() {
                message.push('\n');
                message.push_str(&detail.join("\n"));
            }
            diag(WARNING, message)
        })
        .collect()
}

// ---- file URIs ---------------------------------------------------------------------

fn same_file(a: &Path, b: &Path) -> bool {
    let norm = |p: &Path| p.to_string_lossy().replace('\\', "/").to_lowercase();
    if cfg!(windows) { norm(a) == norm(b) } else { a == b }
}

/// `file:///home/me/x.go` or `file:///C:/Users/me/x.go` (also `c%3A`) to a path.
pub fn uri_to_path(uri: &str) -> Option<PathBuf> {
    let rest = uri.strip_prefix("file://")?;
    let rest = rest.strip_prefix("localhost").unwrap_or(rest);
    let bytes = rest.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(b) = u8::from_str_radix(std::str::from_utf8(&bytes[i + 1..i + 3]).ok()?, 16) {
                out.push(b);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    let mut s = String::from_utf8(out).ok()?;
    // Windows: `/C:/...` → `C:/...`, with an upper-case drive letter like the rest of learn's paths.
    let b = s.as_bytes();
    if b.len() >= 3 && b[0] == b'/' && b[1].is_ascii_alphabetic() && b[2] == b':' {
        s = format!("{}{}", (b[1] as char).to_ascii_uppercase(), &s[2..]);
    }
    Some(PathBuf::from(s))
}

pub fn path_to_uri(path: &Path) -> String {
    let s = path.to_string_lossy().replace('\\', "/");
    let s = if s.starts_with('/') { s } else { format!("/{s}") };
    let mut out = String::from("file://");
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b"-._~/:".contains(&b) {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::CaseResult;

    #[test]
    fn uris_round_trip() {
        let p = if cfg!(windows) { PathBuf::from("C:/Users/me/a b/hello.go") } else { PathBuf::from("/home/me/a b/hello.go") };
        let uri = path_to_uri(&p);
        assert!(uri.contains("a%20b"), "{uri}");
        assert_eq!(uri_to_path(&uri).unwrap(), p);
        assert_eq!(uri_to_path("file:///c%3A/x/y.go").unwrap(), PathBuf::from("C:/x/y.go"));
        assert_eq!(uri_to_path("https://x.dev"), None);
    }

    #[test]
    fn the_anchor_is_the_signature_line() {
        let text = "package hello\n\n// Greet: ...\nfunc Greet(name string) string {\n\treturn \"\"\n}\n";
        assert_eq!(anchor(text, "func Greet(name string) string"), Some((3, 0, 32)));
        // A reformatted signature still finds the name.
        assert_eq!(anchor("x\n  def  greet (name):\n", "def greet(name: str) -> str").map(|a| a.0), Some(1));
    }

    fn hello() -> Spec {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        Spec::load(&root.join("specs").join("hello.json")).unwrap()
    }

    fn result(passed: &[Option<bool>], output: &str) -> CheckResult {
        let spec = hello();
        let cases = spec
            .cases()
            .iter()
            .zip(passed)
            .map(|((_, c), p)| CaseResult { index: c.index, description: c.description.clone(), passed: *p, message: Some("got \"Hello, !\"".into()) })
            .collect::<Vec<_>>();
        let all = cases.iter().all(|c| c.passed == Some(true));
        CheckResult { cases, passed: all, timed_out: false, command: "go test".into(), output: output.into(), setup_problem: None, timeout_secs: 10 }
    }

    #[test]
    fn failing_cases_become_warnings_on_the_signature() {
        let text = "package hello\n\nfunc Greet(name string) string {\n}\n";
        let sigs = vec!["func Greet(name string) string".to_string()];
        let r = result(&[Some(true), Some(false), Some(false)], "");
        let d = diagnostics(&hello(), &r, text, &sigs, Path::new("."));
        assert_eq!(d.len(), 2);
        assert_eq!(d[0]["severity"], WARNING);
        assert_eq!(d[0]["range"]["start"]["line"], 2);
        let msg = d[0]["message"].as_str().unwrap();
        assert!(msg.starts_with("case 2, "), "{msg}");
        assert!(msg.contains("expected"), "{msg}");

        let none = result(&[None, None], "./hello.go:3:1: missing return\n");
        let d = diagnostics(&hello(), &none, text, &sigs, Path::new("."));
        assert_eq!(d.len(), 1);
        assert_eq!(d[0]["severity"], ERROR);
        assert!(d[0]["message"].as_str().unwrap().contains("missing return"));

        let all: Vec<Option<bool>> = hello().cases().iter().map(|_| Some(true)).collect();
        assert!(diagnostics(&hello(), &result(&all, ""), text, &sigs, Path::new(".")).is_empty());
    }

    // ---- the protocol, over an in-memory connection --------------------------------

    struct Client {
        conn: Connection,
        next: i32,
        /// What arrived while waiting for a response.
        seen: Vec<Message>,
    }

    impl Client {
        fn request(&mut self, method: &str, params: Value) -> Value {
            self.next += 1;
            let id = RequestId::from(self.next);
            self.conn.sender.send(Request::new(id.clone(), method.into(), params).into()).unwrap();
            loop {
                match self.recv() {
                    Message::Response(r) if r.id == id => return r.response_result.unwrap_or(Value::Null),
                    other => self.seen.push(other),
                }
            }
        }

        fn notify(&self, method: &str, params: Value) {
            self.conn.sender.send(Notification::new(method.into(), params).into()).unwrap();
        }

        fn recv(&self) -> Message {
            self.conn.receiver.recv_timeout(std::time::Duration::from_secs(120)).expect("no message from the server")
        }

        /// The next notification with this method (including ones already seen).
        fn notification(&mut self, method: &str) -> Value {
            if let Some(i) = self.seen.iter().position(|m| matches!(m, Message::Notification(n) if n.method == method)) {
                if let Message::Notification(n) = self.seen.remove(i) {
                    return n.params;
                }
            }
            loop {
                if let Message::Notification(n) = self.recv() {
                    if n.method == method {
                        return n.params;
                    }
                }
            }
        }
    }

    fn workspace(name: &str) -> PathBuf {
        let ws = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target").join(format!("lsp-{name}"));
        let _ = std::fs::remove_dir_all(&ws);
        let mut ctx = Ctx::discover(Some(ws.clone())).unwrap();
        ctx.config.track = Some("go".into());
        ctx.save_config().unwrap();
        lessons::generate_track(&ctx, backends::get("go").unwrap()).unwrap();
        ws
    }

    fn start(show_document: bool) -> (Client, std::thread::JoinHandle<()>) {
        let (server, client) = Connection::memory();
        let handle = std::thread::spawn(move || serve(&server, None).unwrap());
        let mut c = Client { conn: client, next: 0, seen: Vec::new() };
        let caps = json!({ "capabilities": { "window": { "showDocument": { "support": show_document } } } });
        let init = c.request("initialize", caps);
        assert_eq!(init["capabilities"]["hoverProvider"], true);
        c.notify("initialized", json!({}));
        (c, handle)
    }

    fn stop(mut c: Client, handle: std::thread::JoinHandle<()>) {
        c.request("shutdown", Value::Null);
        c.notify("exit", Value::Null);
        handle.join().unwrap();
    }

    #[test]
    fn hover_code_actions_and_hints_over_the_protocol() {
        let ws = workspace("protocol");
        let file = ws.join("go").join("01-foundations").join("hello").join("hello.go");
        let uri = path_to_uri(&std::path::absolute(&file).unwrap());
        let text = std::fs::read_to_string(&file).unwrap();
        let line = anchor(&text, "func Greet(name string) string").unwrap().0;
        let (mut c, handle) = start(true);

        let h = c.request("textDocument/hover", json!({ "textDocument": { "uri": uri }, "position": { "line": line, "character": 6 } }));
        let md = h["contents"]["value"].as_str().unwrap_or_default();
        assert!(md.starts_with("**Hello** · functions · Go"), "{h}");
        assert!(md.contains("**The tests check**"));
        let off = c.request("textDocument/hover", json!({ "textDocument": { "uri": uri }, "position": { "line": 0, "character": 0 } }));
        assert_eq!(off, Value::Null, "only the signature line");

        let actions = c.request("textDocument/codeAction", json!({ "textDocument": { "uri": uri }, "range": {}, "context": { "diagnostics": [] } }));
        let titles: Vec<&str> = actions.as_array().unwrap().iter().map(|a| a["title"].as_str().unwrap()).collect();
        assert_eq!(titles, ["Check this exercise", "Show next hint", "Open LESSON.md", "Open next exercise"]);

        c.request("workspace/executeCommand", json!({ "command": HINT, "arguments": [uri] }));
        let msg = c.notification("window/showMessage");
        assert!(msg["message"].as_str().unwrap().starts_with("Hint 1 of "), "{msg}");
        let saved = Ctx::discover(Some(ws.clone())).unwrap();
        assert_eq!(saved.progress.hints.get("hello"), Some(&1));

        c.request("workspace/executeCommand", json!({ "command": LESSON, "arguments": [uri] }));
        let seen = std::mem::take(&mut c.seen);
        let show = match seen.into_iter().find_map(|m| match m {
            Message::Request(r) => Some(r),
            _ => None,
        }) {
            Some(r) => r,
            None => loop {
                if let Message::Request(r) = c.recv() {
                    break r;
                }
            },
        };
        assert_eq!(show.method, "window/showDocument");
        assert!(show.params["uri"].as_str().unwrap().ends_with("/go/01-foundations/hello/LESSON.md"), "{}", show.params);

        // The next exercise on the path opens in the editor.
        c.request("workspace/executeCommand", json!({ "command": NEXT, "arguments": [uri] }));
        let seen = std::mem::take(&mut c.seen);
        let show = match seen.into_iter().find_map(|m| match m {
            Message::Request(r) => Some(r),
            _ => None,
        }) {
            Some(r) => r,
            None => loop {
                if let Message::Request(r) = c.recv() {
                    break r;
                }
            },
        };
        let ctx = Ctx::discover(Some(ws.clone())).unwrap();
        let go = backends::get("go").unwrap();
        let want = backends::main_file(go, ctx.path_specs("go")[1]).components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect::<Vec<_>>().join("/");
        assert!(show.params["uri"].as_str().unwrap().ends_with(&format!("/go/{want}")), "{} should open {want}", show.params);
        stop(c, handle);
    }

    #[test]
    fn files_outside_a_workspace_get_nothing() {
        let (mut c, handle) = start(false);
        let elsewhere = path_to_uri(&std::env::temp_dir().join("not-learn").join("main.go"));
        let h = c.request("textDocument/hover", json!({ "textDocument": { "uri": elsewhere }, "position": { "line": 0, "character": 0 } }));
        assert_eq!(h, Value::Null);
        let a = c.request("textDocument/codeAction", json!({ "textDocument": { "uri": elsewhere }, "range": {}, "context": {} }));
        assert_eq!(a, json!([]));
        stop(c, handle);
    }

    #[test]
    fn saving_runs_the_tests_and_publishes() {
        let ws = workspace("save");
        let file = std::path::absolute(ws.join("go").join("01-foundations").join("hello").join("hello.go")).unwrap();
        let (c, handle) = start(false);
        c.notify("textDocument/didSave", json!({ "textDocument": { "uri": path_to_uri(&file) } }));
        // With Go installed: the untouched stub fails every case. Without it: a message says so.
        let reply = loop {
            match c.recv() {
                Message::Notification(n) if n.method == "textDocument/publishDiagnostics" || n.method == "window/showMessage" => break n,
                _ => continue,
            }
        };
        if reply.method == "textDocument/publishDiagnostics" {
            assert!(!reply.params["diagnostics"].as_array().unwrap().is_empty(), "{}", reply.params);
        } else {
            assert!(reply.params["message"].as_str().unwrap().contains("isn't installed"), "{}", reply.params);
        }
        stop(c, handle);
    }
}
