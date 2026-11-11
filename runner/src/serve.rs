//! `learn serve`: a local web UI over the same specs, backends and progress as
//! the CLI. Foreground only, bound to 127.0.0.1, gone when the command exits.
//!
//! The UI is a static Nuxt build embedded in the binary (`web/dist`). Test
//! results stream to the page over server-sent events as each case reports.

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::{Ipv6Addr, SocketAddr};
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
use crate::lessons;
use crate::views;
use crate::ctx::Ctx;
use crate::error::{Error, Result};
use crate::notes;
use crate::spec::Spec;
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

/// An exercise on a track, or 404: an unknown slug, or another track's own exercise.
fn spec_or_404<'a>(ctx: &'a Ctx, track: &str, slug: &str) -> ApiResult<&'a Spec> {
    ctx.spec_on(track, slug).map_err(|e| ApiError(StatusCode::NOT_FOUND, e.0))
}

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
    // Browsers try `learn.localhost` over IPv6 first. Claim [::1] as well, so
    // another program listening there on the same port can't answer instead.
    let listener_v6 = match tokio::net::TcpListener::bind(SocketAddr::from((Ipv6Addr::LOCALHOST, port))).await {
        Ok(l) => Some(l),
        Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
            return Err(Error(format!(
                "port {port} is already used by another program on [::1] (IPv6), so http://learn.localhost:{port} would open that one. Pick another port with `--port`."
            )));
        }
        // No IPv6 on this machine: browsers fall back to 127.0.0.1.
        Err(_) => None,
    };

    let (events, _) = broadcast::channel(256);
    let (shutdown, shutdown_rx) = watch::channel(false);
    let app = Arc::new(App { workspace, port, events, shutdown, check_lock: Mutex::new(()) });

    let watcher = {
        let app = app.clone();
        std::thread::spawn(move || watch_files(app))
    };

    let api = Router::new()
        .route("/api/state", get(state))
        .route("/api/track", post(set_track))
        .route("/api/from", post(set_from))
        .route("/api/goal", post(set_goal))
        .route("/api/path", get(path))
        .route("/api/exercise/{slug}", get(exercise))
        .route("/api/hint/{slug}", post(hint))
        .route("/api/check/{slug}", post(start_check))
        .route("/api/compare/{concept}", get(compare))
        .route("/api/reading/{id}", get(reading))
        .route("/api/read/{id}", post(mark_read))
        .route("/api/file/{slug}", get(read_file).put(write_file))
        .route("/api/events", get(event_stream))
        .route("/api/quit", post(quit));
    let router = api
        .fallback(static_file)
        .layer(middleware::from_fn_with_state(app.clone(), guard))
        .with_state(app.clone());

    let url = format!("http://learn.localhost:{port}");
    println!("polysyntax is running at {url}");
    println!("(also http://127.0.0.1:{port}). Stop it with Ctrl+C or the Quit button in the page.");
    if open {
        if let Err(e) = crate::commands::open_url(&url) {
            println!("{e}");
        }
    }

    // Ctrl+C or the Quit button stops both listeners.
    let signal = |mut rx: watch::Receiver<bool>| async move {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
            _ = rx.wait_for(|stop| *stop) => {}
        }
    };
    let v4 = axum::serve(listener, router.clone()).with_graceful_shutdown(signal(shutdown_rx.clone()));
    let v6 = async {
        match listener_v6 {
            Some(l) => axum::serve(l, router).with_graceful_shutdown(signal(shutdown_rx)).await,
            None => Ok(()),
        }
    };
    let (result, result_v6) = tokio::join!(v4, v6);
    let result = result.and(result_v6);

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
        format!("[::1]:{}", app.port),
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

// ---- views ------------------------------------------------------------------------
//
// The JSON bodies are `learn_core::views`, the same data the CLI draws.

async fn state(State(app): State<Shared>) -> ApiResult<Json<views::CourseView>> {
    let ctx = app.ctx()?;
    Ok(Json(views::course(&ctx)))
}

#[derive(Deserialize)]
struct TrackBody {
    track: String,
}

async fn set_track(State(app): State<Shared>, Json(body): Json<TrackBody>) -> ApiResult<Json<Value>> {
    let mut ctx = app.ctx()?;
    let backend = backends::get(&body.track)?;
    lessons::generate_track(&ctx, backend)?;
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
    lessons::refresh(&ctx)?;
    Ok(Json(json!({ "from": body.from })))
}

#[derive(Deserialize)]
struct GoalBody {
    goal: String,
    #[serde(default)]
    compare: Vec<String>,
}

async fn set_goal(State(app): State<Shared>, Json(body): Json<GoalBody>) -> ApiResult<Json<Value>> {
    let mut ctx = app.ctx()?;
    ctx.set_goal(&body.goal, &body.compare)?;
    lessons::refresh(&ctx)?;
    Ok(Json(json!({ "goal": ctx.goal(), "compare": ctx.config.compare })))
}

/// A track's path (the active track's by default).
async fn path(State(app): State<Shared>, Query(q): Query<TrackQuery>) -> ApiResult<Json<views::PathView>> {
    let ctx = app.ctx()?;
    let b = resolve_track(&ctx, &q)?;
    Ok(Json(views::path(&ctx, b)))
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

fn rel_string(p: &std::path::Path) -> String {
    p.components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect::<Vec<_>>().join("/")
}

async fn exercise(
    State(app): State<Shared>,
    UrlPath(slug): UrlPath<String>,
    Query(q): Query<TrackQuery>,
) -> ApiResult<Json<views::ExerciseView>> {
    let ctx = app.ctx()?;
    let b = resolve_track(&ctx, &q)?;
    let spec = spec_or_404(&ctx, b.id(), &slug)?;
    let dir = ctx.workspace.join(b.id());
    if !dir.join(backends::main_file(b, spec)).exists() {
        lessons::generate_track(&ctx, b)?;
    }
    Ok(Json(views::exercise(&ctx, b, spec)?))
}

async fn hint(State(app): State<Shared>, UrlPath(slug): UrlPath<String>) -> ApiResult<Json<Value>> {
    let mut ctx = app.ctx()?;
    let spec = ctx.spec(&slug).map_err(|e| ApiError(StatusCode::NOT_FOUND, e.0))?;
    let (total, hints) = (spec.hints.len(), spec.hints.clone());
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
) -> ApiResult<Json<views::CompareView>> {
    let ctx = app.ctx()?;
    if !views::is_comparable(&ctx, &concept) {
        return Err(ApiError(StatusCode::NOT_FOUND, format!("unknown concept `{concept}`")));
    }
    let wanted: Vec<String> = q
        .get("tracks")
        .map(|t| t.split(',').filter(|s| !s.is_empty()).map(String::from).collect())
        .unwrap_or_default();
    Ok(Json(views::compare(&ctx, &concept, &wanted)?))
}

/// A reading page: the concept page and one track's note, with its neighbours.
async fn reading(
    State(app): State<Shared>,
    UrlPath(id): UrlPath<String>,
    Query(q): Query<TrackQuery>,
) -> ApiResult<Json<views::ReadingView>> {
    let ctx = app.ctx()?;
    let Some(r) = ctx.reading(&id) else {
        return Err(ApiError(StatusCode::NOT_FOUND, format!("unknown reading `{id}`")));
    };
    let backend = match &q.track {
        Some(t) => Some(backends::get(t)?),
        None => ctx.config.track.as_deref().map(backends::get).transpose()?,
    };
    Ok(Json(views::reading(&ctx, r, backend)))
}

async fn mark_read(State(app): State<Shared>, UrlPath(id): UrlPath<String>) -> ApiResult<Json<Value>> {
    let mut ctx = app.ctx()?;
    let Some(id) = ctx.reading(&id).map(|r| r.id.clone()) else {
        return Err(ApiError(StatusCode::NOT_FOUND, format!("unknown reading `{id}`")));
    };
    ctx.mark_read(&id)?;
    Ok(Json(json!({ "id": id, "read": true })))
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
fn editable_path(ctx: &Ctx, q: &FileQuery, slug: &str) -> ApiResult<PathBuf> {
    let b = match &q.track {
        Some(t) => backends::get(t)?,
        None => backends::get(&ctx.active_track()?)?,
    };
    let spec = spec_or_404(ctx, b.id(), slug)?;
    let allowed = backends::learner_files(b, spec).into_iter().find(|(rel, _)| rel_string(rel) == q.path);
    match allowed {
        Some((rel, _)) => Ok(ctx.workspace.join(b.id()).join(rel)),
        None => Err(ApiError(StatusCode::BAD_REQUEST, format!("`{}` isn't one of this exercise's files", q.path))),
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
    spec_or_404(&ctx, b.id(), &slug)?;
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
        let spec = ctx.spec_on(track, slug)?.clone();
        app.emit(json!({ "type": "check-started", "track": track, "slug": slug, "total": spec.cases().len() }));
        let dir = ctx.workspace.join(b.id());
        let result = check::run_streaming(b, &spec, &ctx.workspace_specs(track), &dir, &tool, &mut |index, passed| {
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
            "setupProblem": result.setup_problem,
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
        for spec in ctx.specs.iter().filter(|s| s.on_track(&track)) {
            let key = (track.clone(), spec.slug.clone());
            let now = crate::commands::latest_mtime(&ctx.exercise_dir(&track, &spec.slug));
            match seen.insert(key, now) {
                Some(before) if before != now && now.is_some() => {
                    std::thread::sleep(Duration::from_millis(150));
                    app.emit(json!({ "type": "file-changed", "track": track, "slug": spec.slug }));
                    run_check(&app, &track, &spec.slug);
                    // Our own run may touch files; take a fresh baseline.
                    seen.insert((track.clone(), spec.slug.clone()), crate::commands::latest_mtime(&ctx.exercise_dir(&track, &spec.slug)));
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
