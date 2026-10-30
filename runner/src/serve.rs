//! `learn serve`: a local web UI over the same specs, backends and progress as
//! the CLI. Foreground only, bound to 127.0.0.1, gone when the command exits.
//!
//! The UI is a static Nuxt build embedded in the binary (`web/dist`). Test
//! results stream to the page over server-sent events as each case reports.

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::extract::{Path as UrlPath, Query, Request, State};
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::sync::{broadcast, watch};

use crate::backends::{self, Backend};
use crate::check;
use crate::course::TrackMeta;
use crate::ctx::Ctx;
use crate::error::{Error, Result};
use crate::notes;
use crate::spec::{CaseKind, Outcome, Spec};
use crate::toolchain;

#[derive(rust_embed::Embed)]
#[folder = "web/dist/"]
struct Assets;

struct App {
    workspace: Option<PathBuf>,
    port: u16,
    events: broadcast::Sender<String>,
    shutdown: watch::Sender<bool>,
    /// One test run at a time: toolchains like Cargo lock their workspace anyway.
    check_lock: Mutex<()>,
}

type Shared = Arc<App>;

impl App {
    fn ctx(&self) -> Result<Ctx> {
        Ctx::discover(self.workspace.clone())
    }

    fn emit(&self, event: Value) {
        let _ = self.events.send(event.to_string());
    }
}

// ---- errors -----------------------------------------------------------------------

struct ApiError(StatusCode, String);

impl From<Error> for ApiError {
    fn from(e: Error) -> Self {
        ApiError(StatusCode::BAD_REQUEST, e.0)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({ "error": self.1 }))).into_response()
    }
}

type ApiResult<T> = std::result::Result<T, ApiError>;

// ---- entry point -------------------------------------------------------------------

pub fn serve(workspace: Option<PathBuf>, port: u16, open: bool) -> Result<()> {
    // Fail early (and readably) if the course can't be found.
    Ctx::discover(workspace.clone())?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| Error(format!("couldn't start the web server: {e}")))?;
    runtime.block_on(run(workspace, port, open))
}

async fn run(workspace: Option<PathBuf>, port: u16, open: bool) -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.map_err(|e| {
        Error(format!(
            "can't listen on port {port} ({e}). Is another `learn serve` running? Pick another port with `--port`."
        ))
    })?;

    let (events, _) = broadcast::channel(256);
    let (shutdown, mut shutdown_rx) = watch::channel(false);
    let app = Arc::new(App { workspace, port, events, shutdown, check_lock: Mutex::new(()) });

    let watcher = {
        let app = app.clone();
        std::thread::spawn(move || watch_files(app))
    };

    let api = Router::new()
        .route("/api/state", get(state))
        .route("/api/track", post(set_track))
        .route("/api/from", post(set_from))
        .route("/api/exercise/{slug}", get(exercise))
        .route("/api/hint/{slug}", post(hint))
        .route("/api/check/{slug}", post(start_check))
        .route("/api/compare/{concept}", get(compare))
        .route("/api/file/{slug}", get(read_file).put(write_file))
        .route("/api/events", get(event_stream))
        .route("/api/quit", post(quit));
    let router = api
        .fallback(static_file)
        .layer(middleware::from_fn_with_state(app.clone(), guard))
        .with_state(app.clone());

    let url = format!("http://learn.localhost:{port}");
    println!("polyglot-learn is running at {url}");
    println!("(also http://127.0.0.1:{port}). Stop it with Ctrl+C or the Quit button in the page.");
    if open {
        if let Err(e) = crate::commands::open_url(&url) {
            println!("{e}");
        }
    }

    let signal = async move {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
            _ = shutdown_rx.wait_for(|stop| *stop) => {}
        }
    };
    let result = axum::serve(listener, router).with_graceful_shutdown(signal).await;

    // Stop the watcher and any test process still running.
    let _ = app.shutdown.send(true);
    toolchain::cancel_all();
    let _ = watcher.join();
    println!("Stopped.");
    result.map_err(|e| Error(format!("the web server failed: {e}")))
}

// ---- security ----------------------------------------------------------------------

/// The server runs code, so it only answers requests addressed to localhost
/// (blocks DNS rebinding), and state-changing requests must carry a custom
/// header that a cross-site page can't send without a CORS preflight.
async fn guard(State(app): State<Shared>, req: Request, next: Next) -> Response {
    let host = req.headers().get(header::HOST).and_then(|h| h.to_str().ok()).unwrap_or("");
    let allowed = [
        format!("learn.localhost:{}", app.port),
        format!("localhost:{}", app.port),
        format!("127.0.0.1:{}", app.port),
    ];
    if !allowed.iter().any(|a| a == host) {
        return (StatusCode::FORBIDDEN, "learn serve only answers on localhost").into_response();
    }
    if let Some(origin) = req.headers().get(header::ORIGIN).and_then(|o| o.to_str().ok()) {
        if !allowed.iter().any(|a| origin == format!("http://{a}")) {
            return (StatusCode::FORBIDDEN, "cross-origin requests are not allowed").into_response();
        }
    }
    if req.method() != Method::GET && req.headers().get("x-learn").is_none() {
        return (StatusCode::FORBIDDEN, "missing x-learn header").into_response();
    }
    let mut res = next.run(req).await;
    let h = res.headers_mut();
    h.insert("x-content-type-options", HeaderValue::from_static("nosniff"));
    h.insert("referrer-policy", HeaderValue::from_static("no-referrer"));
    h.insert("x-frame-options", HeaderValue::from_static("DENY"));
    res
}

// ---- static UI ---------------------------------------------------------------------

async fn static_file(req: Request) -> Response {
    let path = req.uri().path().trim_start_matches('/');
    if path.starts_with("api/") {
        return (StatusCode::NOT_FOUND, Json(json!({ "error": "unknown API route" }))).into_response();
    }
    let (file, cache) = match Assets::get(path) {
        Some(f) if !path.is_empty() => (f, path.starts_with("_nuxt/")),
        // Client-side routes (/exercise/hello) all serve the SPA shell.
        _ => match Assets::get("index.html") {
            Some(f) => (f, false),
            None => return (StatusCode::NOT_FOUND, "the web UI wasn't built into this binary").into_response(),
        },
    };
    let mime = file.metadata.mimetype().to_string();
    let mut res = Response::new(Body::from(file.data.into_owned()));
    let h = res.headers_mut();
    if let Ok(v) = HeaderValue::from_str(&mime) {
        h.insert(header::CONTENT_TYPE, v);
    }
    let cc = if cache { "public, max-age=31536000, immutable" } else { "no-cache" };
    h.insert(header::CACHE_CONTROL, HeaderValue::from_static(cc));
    res
}

// ---- JSON views ----------------------------------------------------------------------

fn track_json(ctx: &Ctx, b: &dyn Backend) -> Value {
    let tool = b.tool().find();
    json!({
        "id": b.id(),
        "name": b.name(),
        "tool": {
            "label": b.tool().label,
            "found": tool.is_some(),
            "path": tool.as_ref().map(|t| t.path.display().to_string()),
            "version": tool.as_ref().map(|t| t.version.clone()),
            "warning": tool.as_ref().and_then(|t| b.check_tool(t)),
        },
        "done": ctx.progress.done.get(b.id()).map_or(0, |d| d.len()),
        "current": ctx.current(b.id()).map(|s| s.slug.clone()),
    })
}

async fn state(State(app): State<Shared>) -> ApiResult<Json<Value>> {
    let ctx = app.ctx()?;
    let tracks: Vec<Value> = backends::all().iter().map(|b| track_json(&ctx, *b)).collect();
    let modules: Vec<Value> = ctx
        .course
        .modules
        .iter()
        .map(|m| {
            let exercises: Vec<Value> = m
                .exercises
                .iter()
                .filter_map(|slug| ctx.spec(slug).ok())
                .map(|s| {
                    let done: Vec<&str> = backends::ids().into_iter().filter(|t| ctx.is_done(t, &s.slug)).collect();
                    json!({ "slug": s.slug, "title": s.title, "concept": s.concept, "done": done })
                })
                .collect();
            json!({ "id": m.id, "title": m.title, "exercises": exercises })
        })
        .collect();
    let backgrounds: Vec<Value> =
        notes::BACKGROUNDS.iter().map(|(id, label)| json!({ "id": id, "label": label })).collect();
    let mut concepts: Vec<&str> = ctx.specs.iter().map(|s| s.concept.as_str()).collect();
    concepts.dedup();
    Ok(Json(json!({
        "title": ctx.course.title,
        "track": ctx.config.track,
        "from": ctx.config.from,
        "backgrounds": backgrounds,
        "tracks": tracks,
        "modules": modules,
        "concepts": concepts,
        "workspace": ctx.workspace.display().to_string(),
    })))
}

#[derive(Deserialize)]
struct TrackBody {
    track: String,
}

async fn set_track(State(app): State<Shared>, Json(body): Json<TrackBody>) -> ApiResult<Json<Value>> {
    let mut ctx = app.ctx()?;
    let backend = backends::get(&body.track)?;
    backends::generate(backend, &ctx.specs, &ctx.workspace.join(backend.id()))?;
    ctx.config.track = Some(backend.id().to_string());
    ctx.save_config()?;
    app.emit(json!({ "type": "progress" }));
    Ok(Json(json!({ "track": backend.id() })))
}

#[derive(Deserialize)]
struct FromBody {
    from: String,
}

async fn set_from(State(app): State<Shared>, Json(body): Json<FromBody>) -> ApiResult<Json<Value>> {
    let mut ctx = app.ctx()?;
    if !notes::BACKGROUNDS.iter().any(|(id, _)| *id == body.from) {
        return Err(ApiError(StatusCode::BAD_REQUEST, format!("unknown background `{}`", body.from)));
    }
    ctx.config.from = Some(body.from.clone());
    ctx.save_config()?;
    Ok(Json(json!({ "from": body.from })))
}

#[derive(Deserialize)]
struct TrackQuery {
    track: Option<String>,
}

fn resolve_track(ctx: &Ctx, q: &TrackQuery) -> Result<&'static dyn Backend> {
    match &q.track {
        Some(t) => backends::get(t),
        None => backends::get(&ctx.active_track()?),
    }
}

fn from_of(ctx: &Ctx) -> Option<&str> {
    ctx.config.from.as_deref().filter(|f| *f != "none")
}

fn links_json(meta: &TrackMeta, concept: &str) -> Vec<Value> {
    meta.links_for(concept)
        .iter()
        .map(|l| json!({ "kind": l.kind, "label": meta.kind_label(&l.kind), "title": l.title, "url": l.url }))
        .collect()
}

fn cases_json(spec: &Spec) -> Vec<Value> {
    let join = |vals: &[Value]| vals.iter().map(backends::json).collect::<Vec<_>>().join(", ");
    spec.cases()
        .into_iter()
        .map(|(f, c)| {
            let lines: Vec<String> = match &c.kind {
                CaseKind::Call { input, outcome } => {
                    let expected = match outcome {
                        Outcome::Value(v) => backends::json(v),
                        Outcome::Error => "an error".into(),
                    };
                    vec![format!("{}({}) → {expected}", f.name, join(input))]
                }
                CaseKind::Scenario { create, steps } => {
                    let mut l: Vec<String> =
                        create.iter().enumerate().map(|(i, a)| format!("v{i} = {}({})", f.name, join(a))).collect();
                    l.extend(steps.iter().map(|s| format!("v{}({}) → {}", s.on, join(&s.input), backends::json(&s.expected))));
                    l
                }
            };
            json!({ "index": c.index, "description": c.description, "function": f.name, "lines": lines })
        })
        .collect()
}

/// Learner-owned files for an exercise: helper modules first, then the main file.
fn learner_files(b: &dyn Backend, spec: &Spec) -> Vec<(PathBuf, Vec<String>)> {
    let mut files = Vec::new();
    for m in spec.modules() {
        let sigs = spec.helpers.iter().filter(|h| h.module.as_deref() == Some(m)).map(|f| b.signature(spec, f)).collect();
        files.push((b.module_path(spec, m), sigs));
    }
    files.push((b.stub_path(spec), vec![b.signature(spec, &spec.main)]));
    files
}

fn rel_string(p: &std::path::Path) -> String {
    p.components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect::<Vec<_>>().join("/")
}

async fn exercise(
    State(app): State<Shared>,
    UrlPath(slug): UrlPath<String>,
    Query(q): Query<TrackQuery>,
) -> ApiResult<Json<Value>> {
    let ctx = app.ctx()?;
    let b = resolve_track(&ctx, &q)?;
    let spec = ctx.spec(&slug)?;
    let dir = ctx.workspace.join(b.id());
    if !dir.join(b.stub_path(spec)).exists() {
        backends::generate(b, &ctx.specs, &dir)?;
    }
    let meta = TrackMeta::load(&ctx.root, b.id())?;
    let shown = ctx.progress.hints.get(&slug).copied().unwrap_or(0).min(spec.hints.len());
    let files: Vec<Value> = learner_files(b, spec)
        .into_iter()
        .map(|(rel, sigs)| json!({ "path": rel_string(&rel), "absolute": dir.join(&rel).display().to_string(), "signatures": sigs }))
        .collect();
    let order: Vec<&str> = ctx.specs.iter().map(|s| s.slug.as_str()).collect();
    let pos = order.iter().position(|s| *s == slug).unwrap_or(0);
    let module = ctx.course.modules.iter().find(|m| m.exercises.contains(&slug));
    Ok(Json(json!({
        "slug": spec.slug,
        "title": spec.title,
        "concept": spec.concept,
        "summary": spec.main.summary,
        "module": module.map(|m| m.title.clone()),
        "track": b.id(),
        "trackName": b.name(),
        "fence": b.fence(),
        "toolFound": b.tool().find().is_some(),
        "toolLabel": b.tool().label,
        "done": ctx.is_done(b.id(), &slug),
        "files": files,
        "lesson": notes::read_concept(&ctx.root, &spec.concept, from_of(&ctx)),
        "note": notes::read_track_note(&ctx.root, b.id(), &spec.concept, from_of(&ctx)),
        "links": links_json(&meta, &spec.concept),
        "hints": spec.hints.iter().take(shown).collect::<Vec<_>>(),
        "hintsTotal": spec.hints.len(),
        "cases": cases_json(spec),
        "prev": pos.checked_sub(1).and_then(|i| order.get(i)),
        "next": order.get(pos + 1),
    })))
}

async fn hint(State(app): State<Shared>, UrlPath(slug): UrlPath<String>) -> ApiResult<Json<Value>> {
    let mut ctx = app.ctx()?;
    let total = ctx.spec(&slug)?.hints.len();
    let hints = ctx.spec(&slug)?.hints.clone();
    let shown = ctx.progress.hints.entry(slug.clone()).or_insert(0);
    *shown = (*shown + 1).min(total);
    let shown = *shown;
    ctx.save_progress()?;
    Ok(Json(json!({ "hints": hints.into_iter().take(shown).collect::<Vec<_>>(), "hintsTotal": total })))
}

async fn compare(
    State(app): State<Shared>,
    UrlPath(concept): UrlPath<String>,
    Query(q): Query<HashMap<String, String>>,
) -> ApiResult<Json<Value>> {
    let ctx = app.ctx()?;
    if !ctx.specs.iter().any(|s| s.concept == concept) {
        return Err(ApiError(StatusCode::NOT_FOUND, format!("unknown concept `{concept}`")));
    }
    let wanted: Vec<String> = q
        .get("tracks")
        .map(|t| t.split(',').filter(|s| !s.is_empty()).map(String::from).collect())
        .unwrap_or_default();
    let mut columns = Vec::new();
    for b in backends::all() {
        if !wanted.is_empty() && !wanted.iter().any(|w| w == b.id()) {
            continue;
        }
        let meta = TrackMeta::load(&ctx.root, b.id())?;
        columns.push(json!({
            "track": b.id(),
            "name": b.name(),
            "note": notes::read_track_note(&ctx.root, b.id(), &concept, from_of(&ctx)),
            "links": links_json(&meta, &concept),
        }));
    }
    let exercise = ctx.specs.iter().find(|s| s.concept == concept).map(|s| s.slug.clone());
    Ok(Json(json!({
        "concept": concept,
        "lesson": notes::read_concept(&ctx.root, &concept, from_of(&ctx)),
        "exercise": exercise,
        "columns": columns,
    })))
}

// ---- editor files --------------------------------------------------------------------

#[derive(Deserialize)]
struct FileQuery {
    track: Option<String>,
    path: String,
}

fn mtime_ms(p: &std::path::Path) -> u64 {
    std::fs::metadata(p)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map_or(0, |d| d.as_millis() as u64)
}

/// Only the exercise's learner-owned files can be read or written.
fn editable_path(ctx: &Ctx, q: &FileQuery, slug: &str) -> Result<PathBuf> {
    let b = match &q.track {
        Some(t) => backends::get(t)?,
        None => backends::get(&ctx.active_track()?)?,
    };
    let spec = ctx.spec(slug)?;
    let allowed = learner_files(b, spec).into_iter().find(|(rel, _)| rel_string(rel) == q.path);
    match allowed {
        Some((rel, _)) => Ok(ctx.workspace.join(b.id()).join(rel)),
        None => Err(Error(format!("`{}` isn't one of this exercise's files", q.path))),
    }
}

async fn read_file(
    State(app): State<Shared>,
    UrlPath(slug): UrlPath<String>,
    Query(q): Query<FileQuery>,
) -> ApiResult<Json<Value>> {
    let ctx = app.ctx()?;
    let path = editable_path(&ctx, &q, &slug)?;
    let content = std::fs::read_to_string(&path)
        .map_err(|e| ApiError(StatusCode::NOT_FOUND, format!("can't read {}: {e}", path.display())))?;
    Ok(Json(json!({ "content": content, "mtime": mtime_ms(&path) })))
}

#[derive(Deserialize)]
struct WriteBody {
    content: String,
    /// The mtime the editor loaded; a different one on disk means someone else saved.
    base_mtime: u64,
}

async fn write_file(
    State(app): State<Shared>,
    UrlPath(slug): UrlPath<String>,
    Query(q): Query<FileQuery>,
    Json(body): Json<WriteBody>,
) -> ApiResult<Response> {
    let ctx = app.ctx()?;
    let path = editable_path(&ctx, &q, &slug)?;
    let current = mtime_ms(&path);
    if current != body.base_mtime {
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        let conflict = json!({ "error": "the file changed on disk", "content": content, "mtime": current });
        return Ok((StatusCode::CONFLICT, Json(conflict)).into_response());
    }
    std::fs::write(&path, body.content).map_err(|e| ApiError(StatusCode::INTERNAL_SERVER_ERROR, format!("can't write {}: {e}", path.display())))?;
    Ok(Json(json!({ "mtime": mtime_ms(&path) })).into_response())
}

// ---- checks and events ---------------------------------------------------------------

async fn start_check(
    State(app): State<Shared>,
    UrlPath(slug): UrlPath<String>,
    Query(q): Query<TrackQuery>,
) -> ApiResult<(StatusCode, Json<Value>)> {
    let ctx = app.ctx()?;
    let b = resolve_track(&ctx, &q)?;
    ctx.spec(&slug)?;
    let track = b.id().to_string();
    let app2 = app.clone();
    let slug2 = slug.clone();
    std::thread::spawn(move || run_check(&app2, &track, &slug2));
    Ok((StatusCode::ACCEPTED, Json(json!({ "started": slug }))))
}

/// Run one check and broadcast it: `check-started`, a `case` per result as it
/// arrives, then `check-finished` with the full result (as `learn check` prints it).
fn run_check(app: &App, track: &str, slug: &str) {
    let _guard = app.check_lock.lock().unwrap_or_else(|p| p.into_inner());
    if toolchain::cancelled() {
        return;
    }
    let outcome = (|| -> Result<Value> {
        let mut ctx = app.ctx()?;
        let b = backends::get(track)?;
        let tool = check::require_tool(b)?;
        let spec = ctx.spec(slug)?.clone();
        app.emit(json!({ "type": "check-started", "track": track, "slug": slug, "total": spec.cases().len() }));
        let dir = ctx.workspace.join(b.id());
        let result = check::run_streaming(b, &spec, &ctx.specs, &dir, &tool, &mut |index, passed| {
            app.emit(json!({ "type": "case", "track": track, "slug": slug, "index": index, "passed": passed }));
        })?;
        if result.passed {
            check::record_pass(&mut ctx, track, slug)?;
        }
        let tail: Vec<&str> = result.output.lines().rev().take(60).collect::<Vec<_>>().into_iter().rev().collect();
        Ok(json!({
            "type": "check-finished",
            "track": track,
            "slug": slug,
            "passed": result.passed,
            "timedOut": result.timed_out,
            "noneRan": result.none_ran(),
            "command": result.command,
            "cases": result.cases,
            "output": tail.join("\n"),
            "next": if result.passed { ctx.current(track).map(|s| s.slug.clone()) } else { None },
        }))
    })();
    match outcome {
        Ok(event) => app.emit(event),
        Err(e) => app.emit(json!({ "type": "check-error", "track": track, "slug": slug, "error": e.0 })),
    }
    app.emit(json!({ "type": "progress" }));
}

/// `learn watch`, server-side: when a file under an exercise changes, tell the
/// page and re-run that exercise's tests.
fn watch_files(app: Arc<App>) {
    let mut seen: HashMap<(String, String), Option<SystemTime>> = HashMap::new();
    let mut shutdown = app.shutdown.subscribe();
    loop {
        if *shutdown.borrow_and_update() || toolchain::cancelled() {
            return;
        }
        std::thread::sleep(Duration::from_millis(500));
        let Ok(ctx) = app.ctx() else { continue };
        let Some(track) = ctx.config.track.clone() else { continue };
        let dir = ctx.workspace.join(&track);
        for spec in &ctx.specs {
            let key = (track.clone(), spec.slug.clone());
            let now = crate::commands::latest_mtime(&dir.join(&spec.slug));
            match seen.insert(key, now) {
                Some(before) if before != now && now.is_some() => {
                    std::thread::sleep(Duration::from_millis(150));
                    app.emit(json!({ "type": "file-changed", "track": track, "slug": spec.slug }));
                    run_check(&app, &track, &spec.slug);
                    // Our own run may touch files; take a fresh baseline.
                    seen.insert((track.clone(), spec.slug.clone()), crate::commands::latest_mtime(&dir.join(&spec.slug)));
                }
                _ => {}
            }
        }
    }
}

async fn event_stream(State(app): State<Shared>) -> Sse<impl futures_util::Stream<Item = std::result::Result<Event, Infallible>>> {
    let rx = app.events.subscribe();
    let sd = app.shutdown.subscribe();
    // Ends when the server shuts down, so graceful shutdown doesn't wait on open pages.
    let stream = futures_util::stream::unfold((rx, sd), |(mut rx, mut sd)| async move {
        loop {
            let next = tokio::select! {
                msg = rx.recv() => Some(msg),
                _ = sd.wait_for(|stop| *stop) => None,
            };
            match next {
                Some(Ok(text)) => return Some((Ok(Event::default().data(text)), (rx, sd))),
                Some(Err(broadcast::error::RecvError::Lagged(_))) => continue,
                Some(Err(broadcast::error::RecvError::Closed)) | None => return None,
            }
        }
    });
    Sse::new(stream).keep_alive(KeepAlive::default())
}

async fn quit(State(app): State<Shared>) -> Json<Value> {
    let _ = app.shutdown.send(true);
    toolchain::cancel_all();
    Json(json!({ "stopping": true }))
}
