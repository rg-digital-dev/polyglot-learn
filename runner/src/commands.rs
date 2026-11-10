//! CLI commands. Screens are built in `view` and printed here; anything the
//! learner must do next is spelled out in the output.

use std::collections::BTreeMap;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, SystemTime};

use anstream::{print, println};

use crate::backends::{self, Backend};
use crate::bail;
use crate::check::{self, CheckResult};
use crate::course::TrackMeta;
use crate::ctx::Ctx;
use crate::error::{Context, Error, Result};
use crate::markdown;
use crate::notes;
use crate::spec::Spec;
use crate::toolchain::{self, Invocation};
use crate::ui::{self, MARGIN};
use crate::view;
use crate::Command;

pub fn dispatch(workspace: Option<PathBuf>, command: Option<Command>) -> Result<ExitCode> {
    let mut ctx = Ctx::discover(workspace.clone())?;
    match command.unwrap_or(Command::Show { slug: None, full: false }) {
        Command::Init { from } => init(&mut ctx, from),
        Command::Doctor => doctor(&ctx),
        Command::Start { lang } => start(&mut ctx, &lang),
        Command::Show { slug, full } => show(&ctx, slug.as_deref(), full),
        Command::Lesson { slug } => lesson(&ctx, slug.as_deref()),
        Command::Read { topic, lang } => read(&mut ctx, topic.as_deref(), lang.as_deref()),
        Command::Check { slug, verbose } => check_cmd(&mut ctx, slug.as_deref(), verbose),
        Command::Watch => watch(&mut ctx),
        Command::Hint { slug } => hint(&mut ctx, slug.as_deref()),
        Command::Compare { concept, lang } => compare(&ctx, &concept, &lang),
        Command::Generate { lang } => generate(&ctx, lang.as_deref()),
        Command::Status { all } => status(&ctx, all),
        Command::Docs { concept, lang, open } => docs(&ctx, concept.as_deref(), lang.as_deref(), open),
        Command::CheckLinks => check_links(&ctx),
        Command::Serve { port, open } => crate::serve::serve(workspace, port, open).map(|_| ExitCode::SUCCESS),
        Command::Selftest { lang, solutions, require_all } => selftest(&ctx, &lang, solutions.as_deref(), require_all),
    }
}

// ---- small output helpers --------------------------------------------------

/// A path relative to the current directory when possible, for copy-paste.
fn display_path(p: &Path) -> String {
    ui::short_path(p)
}

fn track_dir(ctx: &Ctx, backend: &dyn Backend) -> PathBuf {
    ctx.workspace.join(backend.id())
}

fn from(ctx: &Ctx) -> Option<&str> {
    ctx.config.from.as_deref().filter(|f| *f != "none")
}

/// The exercise a command applies to: the named one, or the current one.
fn pick<'a>(ctx: &'a Ctx, track: &str, slug: Option<&str>) -> Result<Option<&'a Spec>> {
    match slug {
        Some(s) => Ok(Some(ctx.spec_on(track, s)?)),
        None => Ok(ctx.current(track)),
    }
}

fn all_done_message(ctx: &Ctx, backend: &dyn Backend) {
    println!("{}", view::all_done(ctx, backend, ui::get()));
}

/// Make sure the exercise's files exist (a slug named before `start` ran).
fn ensure_generated(ctx: &Ctx, backend: &dyn Backend, spec: &Spec) -> Result<()> {
    if !track_dir(ctx, backend).join(backends::main_file(backend, spec)).exists() {
        backends::generate(backend, &ctx.track_specs(backend.id()), &track_dir(ctx, backend))?;
    }
    Ok(())
}

// ---- init / doctor / start -------------------------------------------------

fn init(ctx: &mut Ctx, from_flag: Option<String>) -> Result<ExitCode> {
    let ui = ui::get();
    let choice = match from_flag {
        Some(f) => f.to_ascii_lowercase(),
        None => {
            println!("{MARGIN}{}", ui.heading("Which language do you already know best?"));
            println!("{MARGIN}{}\n", ui.muted("Notes will compare against it."));
            for (i, (_, label)) in notes::BACKGROUNDS.iter().enumerate() {
                println!("{MARGIN}{} {label}", ui.accent(&format!("{:>2})", i + 1)));
            }
            print!("\n{MARGIN}Number or name [1]: ");
            std::io::stdout().flush().ok();
            let mut line = String::new();
            std::io::stdin().lock().read_line(&mut line).context(|| "couldn't read your answer".into())?;
            let answer = line.trim().to_ascii_lowercase();
            if answer.is_empty() {
                notes::BACKGROUNDS[0].0.to_string()
            } else if let Ok(n) = answer.parse::<usize>() {
                match notes::BACKGROUNDS.get(n.wrapping_sub(1)) {
                    Some((id, _)) => id.to_string(),
                    None => bail!("pick a number from 1 to {}", notes::BACKGROUNDS.len()),
                }
            } else {
                answer
            }
        }
    };
    let choice = match choice.as_str() {
        "ts" => "typescript".to_string(),
        "js" => "javascript".to_string(),
        "py" => "python".to_string(),
        "c#" | "cs" => "csharp".to_string(),
        _ => choice,
    };
    if !notes::BACKGROUNDS.iter().any(|(id, _)| *id == choice) {
        let ids: Vec<&str> = notes::BACKGROUNDS.iter().map(|(id, _)| *id).collect();
        bail!("unknown language `{choice}`.\nChoose one of: {}", ids.join(", "));
    }
    ctx.config.from = Some(choice.clone());
    ctx.save_config()?;
    let label = notes::BACKGROUNDS.iter().find(|(id, _)| *id == choice).map(|(_, l)| *l).unwrap_or("");
    let saved = if choice == "none" {
        "Saved. Notes will use neutral explanations.".to_string()
    } else {
        format!("Saved. Notes will add \"coming from {label}\" sections where they exist.")
    };
    println!("\n{MARGIN}{} {}", ui.pass_mark(), saved);
    println!("{MARGIN}{}", ui.muted("Change it any time with learn init."));
    if ctx.config.track.is_none() {
        println!("\n{MARGIN}{} {}", ui.accent("learn start <lang>"), ui.muted(&format!("to pick a track: {}", backends::ids().join(", "))));
    }
    Ok(ExitCode::SUCCESS)
}

/// Which `[install]` keys apply to this machine.
fn os_keys() -> Vec<&'static str> {
    if cfg!(windows) {
        return vec!["windows"];
    }
    if cfg!(target_os = "macos") {
        return vec!["macos"];
    }
    let release = std::fs::read_to_string("/etc/os-release").unwrap_or_default().to_ascii_lowercase();
    let ids: String = release
        .lines()
        .filter(|l| l.starts_with("id=") || l.starts_with("id_like="))
        .collect::<Vec<_>>()
        .join(" ");
    let mut keys = Vec::new();
    if ids.contains("arch") {
        keys.push("arch");
    }
    if ids.contains("debian") || ids.contains("ubuntu") {
        keys.push("debian");
    }
    if ids.contains("fedora") || ids.contains("rhel") {
        keys.push("fedora");
    }
    if keys.is_empty() {
        keys = vec!["arch", "debian", "fedora"];
    }
    keys
}

/// The version number from a tool's banner ("go version go1.27.0 …" -> "1.27.0").
fn short_version(v: &str) -> String {
    toolchain::parse_version(v).map(|(a, b, c)| format!("{a}.{b}.{c}")).unwrap_or_default()
}

fn doctor(ctx: &Ctx) -> Result<ExitCode> {
    let ui = ui::get();
    println!("{MARGIN}{}", ui.heading("Toolchains"));
    println!("{MARGIN}{}\n", ui.muted("learn looks on PATH and in the usual install folders"));
    let mut missing = Vec::new();
    for b in backends::all() {
        let active = if ctx.config.track.as_deref() == Some(b.id()) { format!("  {}", ui.accent("active")) } else { String::new() };
        let name = format!("{:<11}", b.id());
        match b.tool().find() {
            Some(found) => {
                let version = format!("{:<8}", short_version(&found.version));
                // The tool's path goes on its own line when it doesn't fit beside the name.
                let path = ui::short_path(&found.path);
                let line = |mark: String| {
                    let one = format!("{MARGIN}{mark} {name} {version} {}{active}", ui.muted(&path));
                    if ui::visible_width(&one) <= ui.width {
                        one
                    } else {
                        format!("{MARGIN}{mark} {name} {version}{active}\n{MARGIN}    {}", ui.muted(&path))
                    }
                };
                match b.probe(&found) {
                    Ok(()) => println!("{}", line(ui.pass_mark())),
                    Err(problem) => {
                        println!("{}", line(ui.fail_mark()));
                        for l in ui.wrap(&format!("found, but it can't run tests yet: {problem}"), "      ", "      ") {
                            println!("{}", ui.failure(&l));
                        }
                        missing.push(*b);
                    }
                }
                if let Some(warning) = b.check_tool(&found) {
                    for l in ui.wrap(&warning, "      ! ", "        ") {
                        println!("{}", ui.warning(&l));
                    }
                }
            }
            None => {
                println!("{MARGIN}{} {name} {}{active}", ui.fail_mark(), ui.failure(&format!("{} not found", b.tool().label)));
                missing.push(*b);
            }
        }
        // The type checker some track-only exercises use, on its own line under the track.
        if let Some(checker) = b.checker() {
            let label = format!("{:<11}", format!("+ {}", checker.label));
            match checker.find() {
                Some(found) => {
                    let version = format!("{:<8}", short_version(&found.version));
                    println!("{MARGIN}  {label} {version} {}", ui.muted(&ui::short_path(&found.path)));
                    if let Some(problem) = b.check_checker(&found) {
                        for l in ui.wrap(&problem, "      ! ", "        ") {
                            println!("{}", ui.warning(&l));
                        }
                    }
                }
                None => {
                    println!("{MARGIN}  {label} {}", ui.failure(&format!("{} not found (needed for type-check exercises)", checker.label)));
                    if !missing.iter().any(|m| m.id() == b.id()) {
                        missing.push(*b);
                    }
                }
            }
        }
    }
    println!();
    println!("{MARGIN}{} {}", ui.muted("Workspace"), ui.path(&ctx.workspace));
    if missing.is_empty() {
        println!("\n{MARGIN}{} {}", ui.pass_mark(), ui.success("Everything is installed."));
        return Ok(ExitCode::SUCCESS);
    }
    println!("\n{MARGIN}{}", ui.heading("To install what's missing"));
    let keys = os_keys();
    for b in missing {
        let meta = TrackMeta::load(&ctx.root, b.id())?;
        println!("\n{MARGIN}{} {}", ui.heading(b.name()), ui.muted(&format!("({})", b.tool().label)));
        let mut any = false;
        for key in &keys {
            if let Some(lines) = meta.install.get(*key) {
                if keys.len() > 1 {
                    println!("{MARGIN}  {}", ui.muted(&format!("{key}:")));
                }
                for l in lines {
                    if l.starts_with('#') {
                        println!("{MARGIN}    {}", ui.muted(l));
                    } else {
                        println!("{MARGIN}    {}", ui.code(l));
                    }
                }
                any = true;
            }
        }
        if !any {
            println!("{MARGIN}    See tracks/{}/README.md", b.id());
        }
    }
    println!("\n{MARGIN}{}", ui.muted("Open a new terminal after installing, then run learn doctor again."));
    Ok(ExitCode::SUCCESS)
}

fn start(ctx: &mut Ctx, lang: &str) -> Result<ExitCode> {
    let ui = ui::get();
    let backend = backends::get(lang)?;
    let dir = track_dir(ctx, backend);
    let report = backends::generate(backend, &ctx.track_specs(backend.id()), &dir)?;
    ctx.config.track = Some(backend.id().to_string());
    ctx.save_config()?;

    let mut facts = vec![format!("{} track", backend.name())];
    if !report.stubs_created.is_empty() {
        facts.push(format!("{} exercise files created", report.stubs_created.len()));
    }
    if !report.stubs_kept.is_empty() {
        facts.push(format!("{} of your files kept", report.stubs_kept.len()));
    }
    facts.push(display_path(&dir));
    println!("{MARGIN}{}", ui.muted(&facts.join(&format!(" {} ", ui.dot()))));
    if backend.tool().find().is_none() {
        let msg = format!("{} isn't installed yet, so tests can't run. learn doctor shows how to install it.", backend.tool().label);
        for l in ui.wrap(&msg, MARGIN, MARGIN) {
            println!("{}", ui.warning(&l));
        }
    }
    if ctx.config.from.is_none() {
        println!("{MARGIN}{}", ui.muted("Tip: learn init gives you notes that compare against a language you know."));
    }
    if let Some(hint) = view::primer_hint(ctx, backend.id(), ui) {
        println!("\n{hint}");
    }
    println!();
    show(ctx, None, false)
}

// ---- show / lesson / hint ----------------------------------------------------

fn show(ctx: &Ctx, slug: Option<&str>, full: bool) -> Result<ExitCode> {
    let ui = ui::get();
    let track = ctx.active_track()?;
    let backend = backends::get(&track)?;
    let Some(spec) = pick(ctx, &track, slug)? else {
        all_done_message(ctx, backend);
        return Ok(ExitCode::SUCCESS);
    };
    ensure_generated(ctx, backend, spec)?;
    let card = view::task_card(ctx, backend, spec, ui);
    if full {
        let rule = ui.muted(&(if ui.unicode { "─" } else { "-" }).repeat(ui.width.saturating_sub(MARGIN.len())));
        ui.page(&format!("{card}\n\n{MARGIN}{rule}\n\n{}", view::lesson(ctx, backend, spec, ui)));
    } else {
        println!("{card}");
    }
    Ok(ExitCode::SUCCESS)
}

fn lesson(ctx: &Ctx, slug: Option<&str>) -> Result<ExitCode> {
    let ui = ui::get();
    if let Some(r) = slug.filter(|s| ctx.spec(s).is_err()).and_then(|s| ctx.reading(s)) {
        bail!("`{}` is a reading, not an exercise: learn read {}", r.id, r.id);
    }
    let track = ctx.active_track()?;
    let backend = backends::get(&track)?;
    let Some(spec) = pick(ctx, &track, slug)? else {
        all_done_message(ctx, backend);
        return Ok(ExitCode::SUCCESS);
    };
    ui.page(&view::lesson(ctx, backend, spec, ui));
    Ok(ExitCode::SUCCESS)
}

/// A reading: the named one, or the first unread one. Showing it marks it read.
fn read(ctx: &mut Ctx, topic: Option<&str>, lang: Option<&str>) -> Result<ExitCode> {
    let ui = ui::get();
    let backend = match lang.or(ctx.config.track.as_deref()) {
        Some(l) => Some(backends::get(l)?),
        None => None,
    };
    let track = backend.map(|b| b.id());
    let reading = match topic {
        Some(t) => match ctx.reading(t) {
            Some(r) => r.clone(),
            None => {
                let ids: Vec<&str> = ctx.readings.iter().map(|r| r.id.as_str()).collect();
                bail!("no reading called `{t}`.\nReadings: {}", ids.join(", "));
            }
        },
        None => {
            let list = view::readings_for(ctx, track);
            if list.is_empty() {
                bail!("this course has no readings.");
            }
            match list.into_iter().find(|r| !ctx.is_read(&r.id)) {
                Some(r) => r.clone(),
                None => {
                    println!("{MARGIN}{}\n", ui.success("You've read every page."));
                    println!("{}", view::reading_list(ctx, track, ui));
                    return Ok(ExitCode::SUCCESS);
                }
            }
        }
    };
    ui.page(&view::reading(ctx, &reading, backend, ui));
    ctx.mark_read(&reading.id)?;
    Ok(ExitCode::SUCCESS)
}

fn hint(ctx: &mut Ctx, slug: Option<&str>) -> Result<ExitCode> {
    let track = ctx.active_track()?;
    let Some(spec) = pick(ctx, &track, slug)?.cloned() else {
        all_done_message(ctx, backends::get(&track)?);
        return Ok(ExitCode::SUCCESS);
    };
    let ui = ui::get();
    let total = spec.hints.len();
    if total == 0 {
        println!("{MARGIN}{}", ui.muted(&format!("{} has no hints. learn lesson explains the idea with an example.", spec.title)));
        return Ok(ExitCode::SUCCESS);
    }
    let shown = ctx.progress.hints.entry(spec.slug.clone()).or_insert(0);
    let was_all = *shown >= total;
    *shown = (*shown + 1).min(total);
    let shown = *shown;
    ctx.save_progress()?;
    println!("{}", view::hints(&spec, shown, was_all, ui));
    Ok(ExitCode::SUCCESS)
}

// ---- check / watch -------------------------------------------------------------

fn print_result(spec: &Spec, result: &CheckResult, verbose: bool, track_dir: &Path) {
    println!("{}", view::check_report(spec, result, verbose, track_dir, ui::get()));
}

/// Run one check, print it, and record progress. Returns whether it passed.
fn check_once(ctx: &mut Ctx, backend: &dyn Backend, slug: &str, verbose: bool) -> Result<bool> {
    let ui = ui::get();
    let tool = check::require_tool(backend)?;
    let spec = ctx.spec_on(backend.id(), slug)?.clone();
    // A progress line while the tests run (they may compile first), erased after.
    if ui.tty {
        print!("{MARGIN}{}", ui.muted(&format!("Running {} tests for {}…", backend.name(), spec.slug)));
        std::io::stdout().flush().ok();
    }
    let result = check::run(backend, &spec, &ctx.track_specs(backend.id()), &track_dir(ctx, backend), &tool);
    if ui.tty {
        print!("\r\x1b[2K");
    }
    let result = result?;
    print_result(&spec, &result, verbose, &track_dir(ctx, backend));

    if result.passed {
        check::record_pass(ctx, backend.id(), &spec.slug)?;
        println!();
        match ctx.current(backend.id()) {
            Some(next) => println!("{}", view::next_up(ctx, next, ui)),
            None => all_done_message(ctx, backend),
        }
    }
    Ok(result.passed)
}

fn check_cmd(ctx: &mut Ctx, slug: Option<&str>, verbose: bool) -> Result<ExitCode> {
    let track = ctx.active_track()?;
    let backend = backends::get(&track)?;
    let Some(slug) = pick(ctx, &track, slug)?.map(|s| s.slug.clone()) else {
        all_done_message(ctx, backend);
        return Ok(ExitCode::SUCCESS);
    };
    let passed = check_once(ctx, backend, &slug, verbose)?;
    Ok(if passed { ExitCode::SUCCESS } else { ExitCode::FAILURE })
}

/// Latest modification time of any file under `dir` (skipping build output).
pub fn latest_mtime(dir: &Path) -> Option<SystemTime> {
    let mut latest = None;
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for e in entries.flatten() {
            let path = e.path();
            let name = e.file_name();
            if matches!(name.to_str(), Some("target" | "node_modules" | ".git")) {
                continue;
            }
            let Ok(meta) = e.metadata() else { continue };
            if meta.is_dir() {
                stack.push(path);
            } else if let Ok(m) = meta.modified() {
                latest = latest.max(Some(m));
            }
        }
    }
    latest
}

/// The watch screen's footer: when it last ran and what it watches.
fn watch_footer(ctx: &Ctx, backend: &dyn Backend, slug: &str) {
    let ui = ui::get();
    let dir = display_path(&track_dir(ctx, backend).join(slug));
    let d = ui.dot();
    println!("\n{MARGIN}{}", ui.muted(&format!("last run {} {d} watching {dir} {d} Ctrl+C to stop", ui::clock())));
}

fn watch(ctx: &mut Ctx) -> Result<ExitCode> {
    let ui = ui::get();
    let track = ctx.active_track()?;
    let backend = backends::get(&track)?;
    check::require_tool(backend)?;
    backends::generate(backend, &ctx.track_specs(backend.id()), &track_dir(ctx, backend))?;

    let Some(mut slug) = ctx.current(&track).map(|s| s.slug.clone()) else {
        all_done_message(ctx, backend);
        return Ok(ExitCode::SUCCESS);
    };
    ui.clear_screen();
    println!("{}\n", view::task_card(ctx, backend, ctx.spec(&slug)?, ui));
    let mut passed = check_once(ctx, backend, &slug, false)?;
    watch_footer(ctx, backend, &slug);

    loop {
        if passed {
            match ctx.current(&track).map(|s| s.slug.clone()) {
                Some(next) => {
                    slug = next;
                    println!();
                    println!("{}", view::task_card(ctx, backend, ctx.spec(&slug)?, ui));
                    println!("\n{MARGIN}{}", ui.muted("Waiting for you to save…"));
                }
                None => return Ok(ExitCode::SUCCESS),
            }
        }
        let dir = track_dir(ctx, backend).join(&slug);
        let before = latest_mtime(&dir);
        loop {
            std::thread::sleep(Duration::from_millis(400));
            if latest_mtime(&dir) != before {
                // Let editors finish writing (some save in several steps).
                std::thread::sleep(Duration::from_millis(150));
                break;
            }
        }
        // The latest result always starts at the top of the screen.
        ui.clear_screen();
        if !ui.tty {
            println!("\n{MARGIN}{}", ui.muted("── change detected ──"));
        }
        passed = check_once(ctx, backend, &slug, false)?;
        watch_footer(ctx, backend, &slug);
    }
}

// ---- compare / docs / generate / status -------------------------------------

fn known_concepts(ctx: &Ctx) -> Vec<String> {
    // Track-only exercises have nothing to compare across tracks.
    let mut v: Vec<String> = ctx.readings.iter().map(|r| r.id.clone()).collect();
    v.extend(ctx.specs.iter().filter(|s| s.native.is_none()).map(|s| s.concept.clone()));
    v.dedup();
    v
}

fn resolve_concept(ctx: &Ctx, concept: &str) -> Result<String> {
    let concepts = known_concepts(ctx);
    if concepts.iter().any(|c| c == concept) {
        return Ok(concept.to_string());
    }
    // A reading by its short name: `learn compare variables`.
    if let Some(r) = ctx.reading(concept) {
        return Ok(r.id.clone());
    }
    // Accept an exercise slug too: `learn compare parse-port`.
    if let Ok(spec) = ctx.spec(concept) {
        if spec.native.is_none() {
            return Ok(spec.concept.clone());
        }
    }
    bail!("unknown concept `{concept}`.\nConcepts: {}", concepts.join(", "))
}

fn compare(ctx: &Ctx, concept: &str, langs: &[String]) -> Result<ExitCode> {
    let ui = ui::get();
    let concept = resolve_concept(ctx, concept)?;
    let selected: Vec<&dyn Backend> = if langs.is_empty() {
        // Only the tracks that have an exercise for the concept (`files` isn't on QML),
        // or, for a reading, the tracks its module is on.
        let reading = ctx.readings.iter().find(|r| r.id == concept);
        let has = |b: &&dyn Backend| match reading {
            Some(r) => ctx.course.modules[r.module].on_track(b.id()),
            None => ctx.specs.iter().any(|s| s.concept == concept && s.on_track(b.id())),
        };
        let on: Vec<&dyn Backend> = backends::all().iter().copied().filter(has).collect();
        if on.is_empty() { backends::all().to_vec() } else { on }
    } else {
        langs.iter().map(|l| backends::get(l)).collect::<Result<_>>()?
    };
    let mut out = vec![format!("{MARGIN}{}", ui.muted(&format!("Comparing {concept} across {} tracks", selected.len())))];
    for b in selected {
        // A full-width coloured bar per track, so sections are easy to find while scrolling.
        let title = format!(" {} ", b.name());
        let fill = (if ui.unicode { "━" } else { "=" }).repeat(ui.width.saturating_sub(MARGIN.len() + title.chars().count() + 2));
        out.push(String::new());
        out.push(format!("{MARGIN}{}{}{}", ui.accent(if ui.unicode { "━━" } else { "==" }), ui.paint(ui::heading_style().fg_color(ui::accent_style().get_fg_color()), &title), ui.accent(&fill)));
        out.push(String::new());
        match notes::read_track_note(&ctx.root, b.id(), &concept, from(ctx)) {
            Some(note) => out.push(markdown::render(&note, ui)),
            None => out.push(format!("{MARGIN}{}", ui.muted(&format!("No {} note for {concept} yet. Contributions welcome: see CONTRIBUTING.md.", b.name())))),
        }
    }
    ui.page(&out.join("\n"));
    Ok(ExitCode::SUCCESS)
}

pub fn open_url(url: &str) -> Result<()> {
    let (prog, args): (&str, Vec<&str>) = if cfg!(windows) {
        ("rundll32", vec!["url.dll,FileProtocolHandler", url])
    } else if cfg!(target_os = "macos") {
        ("open", vec![url])
    } else {
        ("xdg-open", vec![url])
    };
    std::process::Command::new(prog)
        .args(args)
        .spawn()
        .map(|_| ())
        .context(|| format!("couldn't open a browser; visit {url} yourself"))
}

fn docs(ctx: &Ctx, concept: Option<&str>, lang: Option<&str>, open: bool) -> Result<ExitCode> {
    let ui = ui::get();
    let track = match lang {
        Some(l) => backends::get(l)?.id().to_string(),
        None => ctx.active_track()?,
    };
    let concept = match concept {
        Some(c) => resolve_concept(ctx, c)?,
        None => match ctx.current(&track) {
            Some(s) => s.concept.clone(),
            None => bail!("name a concept.\nConcepts: {}", known_concepts(ctx).join(", ")),
        },
    };
    let meta = TrackMeta::load(&ctx.root, &track)?;
    let links = meta.links_for(&concept);
    if links.is_empty() {
        println!("{MARGIN}{}", ui.muted(&format!("No links for {concept} on the {track} track yet.")));
        return Ok(ExitCode::SUCCESS);
    }
    println!("{MARGIN}{} {}\n", ui.heading(&format!("Further reading: {concept}")), ui.muted(&format!("({track})")));
    let pad = links.iter().map(|l| meta.kind_label(&l.kind).chars().count()).max().unwrap_or(0) + 2;
    for l in links {
        let label = meta.kind_label(&l.kind);
        println!("{MARGIN}{}{}{}", ui.muted(&label), " ".repeat(pad - label.chars().count()), ui.link(&l.title, &l.url));
    }
    if open {
        for l in links {
            open_url(&l.url)?;
        }
    } else {
        println!("\n{MARGIN}{}", ui.muted(&format!("learn docs {concept} --open opens them in your browser")));
    }
    Ok(ExitCode::SUCCESS)
}

fn generate(ctx: &Ctx, lang: Option<&str>) -> Result<ExitCode> {
    let ui = ui::get();
    let selected: Vec<&dyn Backend> = match lang {
        Some("all") => backends::all().to_vec(),
        Some(l) => vec![backends::get(l)?],
        None => match &ctx.config.track {
            Some(t) => vec![backends::get(t)?],
            None => bail!("no active track.\nUse `learn generate --lang <lang>` or `--lang all`."),
        },
    };
    for b in selected {
        let dir = track_dir(ctx, b);
        let report = backends::generate(b, &ctx.track_specs(b.id()), &dir)?;
        let d = ui.dot();
        println!(
            "{MARGIN}{} {:<11} {}",
            ui.pass_mark(),
            b.id(),
            ui.muted(&format!(
                "tests regenerated {d} {} new stubs {d} {} kept {d} {}",
                report.stubs_created.len(),
                report.stubs_kept.len(),
                display_path(&dir)
            ))
        );
    }
    Ok(ExitCode::SUCCESS)
}

fn status(ctx: &Ctx, all: bool) -> Result<ExitCode> {
    let ui = ui::get();
    if all {
        println!("{}", view::status_all(ctx, ui));
        return Ok(ExitCode::SUCCESS);
    }
    match &ctx.config.track {
        Some(t) => {
            println!("{}", view::status_track(ctx, backends::get(t)?, ui));
            if let Some(hint) = view::primer_hint(ctx, t, ui) {
                println!("\n{hint}");
            }
        }
        None => {
            println!("{}", view::status_all(ctx, ui));
            println!("\n{MARGIN}{} {}", ui.accent("learn start <lang>"), ui.muted("to pick a track"));
        }
    }
    Ok(ExitCode::SUCCESS)
}

// ---- check-links -------------------------------------------------------------

fn check_links(ctx: &Ctx) -> Result<ExitCode> {
    let Some(curl) = toolchain::which("curl") else {
        bail!("check-links uses curl, which isn't on PATH. Install curl and try again.");
    };
    let mut links: Vec<(String, String, String)> = Vec::new();
    for b in backends::all() {
        let meta = TrackMeta::load(&ctx.root, b.id())?;
        if let Some(l) = &meta.lings {
            links.push((b.id().into(), l.name.clone(), l.url.clone()));
        }
        for (concept, list) in &meta.links {
            for l in list {
                links.push((b.id().into(), format!("{concept}: {}", l.title), l.url.clone()));
            }
        }
    }
    println!("Checking {} links…", links.len());
    let null = if cfg!(windows) { "NUL" } else { "/dev/null" };
    let mut dead = Vec::new();
    for chunk in links.chunks(8) {
        let handles: Vec<_> = chunk
            .iter()
            .cloned()
            .map(|(track, title, url)| {
                let curl = curl.clone();
                std::thread::spawn(move || {
                    let inv = Invocation::new(curl, std::env::temp_dir())
                        .args(["-s", "-L", "-o", null, "-w", "%{http_code}", "--max-time", "20"])
                        .args(["-A", "polyglot-learn check-links (+https://github.com/rg-digital-dev/polyglot-learn)"])
                        .args([url.as_str()])
                        .timeout(Duration::from_secs(30));
                    let code = toolchain::run(&inv).map(|o| o.output.trim().to_string()).unwrap_or_default();
                    (track, title, url, code)
                })
            })
            .collect();
        for h in handles {
            let (track, title, url, code) = h.join().map_err(|_| Error("a link check crashed".into()))?;
            let ok = code.parse::<u16>().is_ok_and(|c| (200..400).contains(&c));
            if !ok {
                dead.push((track, title, url, code));
            }
        }
    }
    if dead.is_empty() {
        println!("All {} links work.", links.len());
        return Ok(ExitCode::SUCCESS);
    }
    let mut by_track: BTreeMap<String, Vec<(String, String, String)>> = BTreeMap::new();
    for (track, title, url, code) in dead {
        by_track.entry(track).or_default().push((title, url, code));
    }
    for (track, items) in &by_track {
        println!("\ntracks/{track}/track.toml");
        for (title, url, code) in items {
            let code = if code.is_empty() || code == "000" { "no response".to_string() } else { format!("HTTP {code}") };
            println!("  ✗ {title}\n    {url}  ({code})");
        }
    }
    println!("\nSome sites block automated requests (HTTP 403/429); open those by hand before removing them.");
    Ok(ExitCode::FAILURE)
}

// ---- selftest ------------------------------------------------------------------

fn copy_dir(src: &Path, dst: &Path) -> Result<usize> {
    let mut n = 0;
    for e in std::fs::read_dir(src).context(|| format!("can't read {}", src.display()))?.flatten() {
        let to = dst.join(e.file_name());
        if e.path().is_dir() {
            n += copy_dir(&e.path(), &to)?;
        } else {
            std::fs::create_dir_all(dst).context(|| format!("can't create {}", dst.display()))?;
            // Write the bytes rather than fs::copy: on Windows that keeps the old
            // mtime, and Cargo would then reuse the binary built from the stub.
            let bytes = std::fs::read(e.path()).context(|| format!("can't read {}", e.path().display()))?;
            std::fs::write(&to, bytes).context(|| format!("can't copy to {}", to.display()))?;
            n += 1;
        }
    }
    Ok(n)
}

fn selftest(ctx: &Ctx, langs: &[String], solutions: Option<&Path>, require_all: bool) -> Result<ExitCode> {
    let selected: Vec<&dyn Backend> = if langs.is_empty() {
        backends::all().to_vec()
    } else {
        langs.iter().map(|l| backends::get(l)).collect::<Result<_>>()?
    };
    let base = std::env::temp_dir().join(format!("learn-selftest-{}", std::process::id()));
    let mut problems = Vec::new();
    let mut skipped = Vec::new();

    for b in selected {
        let Some(tool) = b.tool().find() else {
            println!("── {}: SKIPPED, {} not installed", b.name(), b.tool().label);
            skipped.push(b.id());
            continue;
        };
        println!("── {} ({})", b.name(), tool.path.display());
        if let Some(w) = b.check_tool(&tool) {
            println!("   ! {w}");
        }
        let dir = base.join(b.id());
        let specs = ctx.track_specs(b.id());
        backends::generate(b, &specs, &dir)?;

        for spec in &specs {
            let r = check::run(b, spec, &specs, &dir, &tool)?;
            let passing = r.count(Some(true));
            if r.passed {
                problems.push(format!("{}/{}: the untouched stub PASSES", b.id(), spec.slug));
                println!("   ✗ stub {:<18} passes (it must fail)", spec.slug);
            } else if passing > 0 {
                println!("   ~ stub {:<18} fails ({passing}/{} cases pass with the stub)", spec.slug, r.cases.len());
            } else if r.none_ran() {
                problems.push(format!("{}/{}: stub tests didn't run", b.id(), spec.slug));
                println!("   ✗ stub {:<18} no cases ran:\n{}", spec.slug, r.output);
            } else {
                println!("   ✓ stub {:<18} fails", spec.slug);
            }
        }

        if let Some(sol_root) = solutions {
            let src = sol_root.join(b.id());
            if !src.is_dir() {
                problems.push(format!("{}: no reference solutions in {}", b.id(), src.display()));
                println!("   ✗ no reference solutions in {}", src.display());
                continue;
            }
            copy_dir(&src, &dir)?;
            for spec in &specs {
                let main = backends::main_file(b, spec);
                if !src.join(&main).exists() {
                    problems.push(format!("{}/{}: no reference solution", b.id(), spec.slug));
                    println!("   ✗ ref  {:<18} missing {}", spec.slug, main.display());
                    continue;
                }
                let r = check::run(b, spec, &specs, &dir, &tool)?;
                if r.passed {
                    println!("   ✓ ref  {:<18} passes", spec.slug);
                } else {
                    problems.push(format!("{}/{}: reference solution fails", b.id(), spec.slug));
                    println!("   ✗ ref  {:<18} fails ({}/{}):", spec.slug, r.count(Some(true)), r.cases.len());
                    print_result(spec, &r, r.none_ran(), &dir);
                }
            }
        }
    }
    let _ = std::fs::remove_dir_all(&base);

    println!();
    if !skipped.is_empty() {
        println!("Skipped (toolchain missing): {}", skipped.join(", "));
        if require_all {
            problems.push(format!("missing toolchains: {}", skipped.join(", ")));
        }
    }
    if problems.is_empty() {
        println!("Selftest passed.");
        Ok(ExitCode::SUCCESS)
    } else {
        println!("Selftest found {} problem(s):", problems.len());
        for p in &problems {
            println!("  - {p}");
        }
        Ok(ExitCode::FAILURE)
    }
}
