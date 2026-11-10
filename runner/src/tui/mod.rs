//! `learn` on a terminal: the path, the open item and live tests in one screen.
//! `app` holds the state and keys, `draw` the screen, `ansi` bridges the CLI's
//! renderers; this file owns the terminal, the threads and the editor.

mod ansi;
mod app;
mod draw;

use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crossterm::event::{self, Event, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::DefaultTerminal;

use crate::backends;
use crate::check;
use crate::ctx::Ctx;
use crate::error::{Context, Error, Result};
use crate::toolchain;
use app::{App, Effect, Msg};

/// Plain `learn` opens the TUI when both ends are a terminal, unless `--no-tui`
/// or `NO_TUI` says otherwise.
pub fn wanted(no_tui: bool) -> bool {
    !no_tui
        && std::env::var_os("NO_TUI").is_none_or(|v| v.is_empty() || v == "0")
        && std::io::stdout().is_terminal()
        && std::io::stdin().is_terminal()
}

pub fn run(workspace: Option<PathBuf>) -> Result<ExitCode> {
    let ctx = Ctx::discover(workspace.clone())?;
    let mut app = App::new(workspace, ctx);
    let (tx, rx) = mpsc::channel();
    let watched: Arc<Mutex<Option<PathBuf>>> = Arc::new(Mutex::new(None));
    let stop = Arc::new(AtomicBool::new(false));
    watch(tx.clone(), watched.clone(), stop.clone());

    // try_init also restores the terminal before a panic message prints.
    let mut terminal = ratatui::try_init().context(|| "can't set up the terminal".into())?;
    let mut cache = draw::Cache::default();
    let outcome = (|| -> Result<()> {
        loop {
            *watched.lock().unwrap_or_else(|p| p.into_inner()) = app.watched_dir();
            terminal.draw(|f| draw::draw(f, &app, &mut cache)).context(|| "can't draw".into())?;
            let mut effects = Vec::new();
            if event::poll(Duration::from_millis(150)).context(|| "can't read the keyboard".into())? {
                if let Event::Key(k) = event::read().context(|| "can't read the keyboard".into())? {
                    if k.kind == KeyEventKind::Press {
                        effects.push(app.key(k));
                    }
                }
            }
            while let Ok(m) = rx.try_recv() {
                effects.push(app.on_msg(m));
            }
            for e in effects {
                match e {
                    Effect::None => {}
                    Effect::Quit => return Ok(()),
                    Effect::Check { track, slug } => run_check(tx.clone(), app.workspace.clone(), track, slug),
                    Effect::Edit(file) => {
                        if let Err(e) = edit(&mut terminal, &file) {
                            app.message = Some(e.0);
                        }
                    }
                }
            }
        }
    })();
    stop.store(true, Ordering::SeqCst);
    ratatui::restore();
    outcome.map(|()| ExitCode::SUCCESS)
}

/// Tell the app when a file in the open exercise's folder changes, as `learn
/// watch` does (polling: no platform watcher needed).
fn watch(tx: Sender<Msg>, watched: Arc<Mutex<Option<PathBuf>>>, stop: Arc<AtomicBool>) {
    std::thread::spawn(move || {
        let mut last: Option<(PathBuf, Option<std::time::SystemTime>)> = None;
        while !stop.load(Ordering::SeqCst) {
            std::thread::sleep(Duration::from_millis(400));
            let Some(dir) = watched.lock().unwrap_or_else(|p| p.into_inner()).clone() else {
                last = None;
                continue;
            };
            let now = crate::commands::latest_mtime(&dir);
            let changed = last.as_ref().is_some_and(|(d, before)| *d == dir && *before != now);
            if changed && tx.send(Msg::Saved).is_err() {
                return;
            }
            last = Some((dir, now));
        }
    });
}

/// Run one exercise's tests on a thread, sending each case as it finishes.
fn run_check(tx: Sender<Msg>, workspace: Option<PathBuf>, track: &'static str, slug: String) {
    std::thread::spawn(move || {
        let outcome = (|| -> Result<check::CheckResult> {
            let mut ctx = Ctx::discover(workspace)?;
            let b = backends::get(track)?;
            let tool = check::require_tool(b)?;
            let spec = ctx.spec_on(track, &slug)?.clone();
            let dir = ctx.workspace.join(track);
            let result = check::run_streaming(b, &spec, &ctx.track_specs(track), &dir, &tool, &mut |index, passed| {
                let _ = tx.send(Msg::Case { index, passed });
            })?;
            if result.passed {
                check::record_pass(&mut ctx, track, &slug)?;
            }
            Ok(result)
        })();
        let _ = tx.send(Msg::Finished(outcome.map_err(|e| e.0)));
    });
}

/// The editor command: $VISUAL, $EDITOR, else Zed when it's installed.
fn editor() -> Option<String> {
    ["VISUAL", "EDITOR"]
        .iter()
        .find_map(|k| std::env::var(k).ok().filter(|v| !v.trim().is_empty()))
        .or_else(|| toolchain::which("zed").map(|_| "zed".to_string()))
}

/// Open `file` in the learner's editor. The TUI steps aside while it runs, so a
/// terminal editor gets the screen; GUI editors (zed, code) return at once.
fn edit(terminal: &mut DefaultTerminal, file: &Path) -> Result<()> {
    let Some(cmd) = editor() else {
        return Err(Error(format!("Set $EDITOR (for example `export EDITOR=nvim`) to edit from here. The file is {}", file.display())));
    };
    let mut parts = cmd.split_whitespace();
    let program = parts.next().unwrap_or_default().to_string();
    let args: Vec<String> = parts.map(String::from).collect();

    let mut out = std::io::stdout();
    let _ = disable_raw_mode();
    let _ = execute!(out, LeaveAlternateScreen);
    let status = if cfg!(windows) {
        // `code` and friends are .cmd scripts, which only cmd can start.
        Command::new("cmd").arg("/C").arg(&program).args(&args).arg(file).status()
    } else {
        Command::new(&program).args(&args).arg(file).status()
    };
    let _ = enable_raw_mode();
    let _ = execute!(out, EnterAlternateScreen);
    let _ = terminal.clear();
    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(Error(format!("`{cmd}` exited with {s}"))),
        Err(e) => Err(Error(format!("can't start `{cmd}`: {e}"))),
    }
}
