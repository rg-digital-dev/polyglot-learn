//! CLI commands. Each prints plain text; anything the learner must do next is
//! spelled out in the output.

use std::collections::BTreeMap;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, SystemTime};

use crate::backends::{self, Backend};
use crate::bail;
use crate::check::{self, CheckResult};
use crate::course::TrackMeta;
use crate::ctx::Ctx;
use crate::error::{Context, Error, Result};
use crate::notes;
use crate::spec::{CaseKind, Outcome, Spec};
use crate::toolchain::{self, Invocation};
use crate::Command;

pub fn dispatch(workspace: Option<PathBuf>, command: Option<Command>) -> Result<ExitCode> {
    let mut ctx = Ctx::discover(workspace.clone())?;
    match command.unwrap_or(Command::Show { slug: None }) {
        Command::Init { from } => init(&mut ctx, from),
        Command::Doctor => doctor(&ctx),
        Command::Start { lang } => start(&mut ctx, &lang),
        Command::Show { slug } => show(&ctx, slug.as_deref()),
        Command::Check { slug, verbose } => check_cmd(&mut ctx, slug.as_deref(), verbose),
        Command::Watch => watch(&mut ctx),
        Command::Hint { slug } => hint(&mut ctx, slug.as_deref()),
        Command::Compare { concept, lang } => compare(&ctx, &concept, &lang),
        Command::Generate { lang } => generate(&ctx, lang.as_deref()),
        Command::Status => status(&ctx),
        Command::Docs { concept, lang, open } => docs(&ctx, concept.as_deref(), lang.as_deref(), open),
        Command::CheckLinks => check_links(&ctx),
        Command::Serve { port, open } => crate::serve::serve(workspace, port, open).map(|_| ExitCode::SUCCESS),
        Command::Selftest { lang, solutions, require_all } => selftest(&ctx, &lang, solutions.as_deref(), require_all),
    }
}

// ---- small output helpers --------------------------------------------------

fn rule(title: &str) {
    let line = "─".repeat(64usize.saturating_sub(title.chars().count() + 4));
    println!("\n── {title} {line}");
}

/// A path relative to the current directory when possible, for copy-paste.
fn display_path(p: &Path) -> String {
    let cwd = std::env::current_dir().unwrap_or_default();
    match p.strip_prefix(&cwd) {
        Ok(rel) if !rel.as_os_str().is_empty() => rel.display().to_string(),
        _ => p.display().to_string(),
    }
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
        Some(s) => Ok(Some(ctx.spec(s)?)),
        None => Ok(ctx.current(track)),
    }
}

fn position(ctx: &Ctx, spec: &Spec) -> (String, usize, usize) {
    for m in &ctx.course.modules {
        if let Some(i) = m.exercises.iter().position(|s| *s == spec.slug) {
            return (m.title.clone(), i + 1, m.exercises.len());
        }
    }
    (String::new(), 0, 0)
}

fn all_done_message(ctx: &Ctx, backend: &dyn Backend) {
    println!(
        "You've finished every exercise on the {} track ({} of {}).",
        backend.name(),
        ctx.specs.len(),
        ctx.specs.len()
    );
    println!("Try another track with `learn start <lang>`, or compare languages with `learn compare <concept>`.");
}

// ---- init / doctor / start -------------------------------------------------

fn init(ctx: &mut Ctx, from_flag: Option<String>) -> Result<ExitCode> {
    let choice = match from_flag {
        Some(f) => f.to_ascii_lowercase(),
        None => {
            println!("Which language do you already know best? Notes will compare against it.\n");
            for (i, (_, label)) in notes::BACKGROUNDS.iter().enumerate() {
                println!("  {:>2}) {label}", i + 1);
            }
            print!("\nNumber or name [1]: ");
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
        bail!("unknown language `{choice}`. Choose one of: {}", ids.join(", "));
    }
    ctx.config.from = Some(choice.clone());
    ctx.save_config()?;
    let label = notes::BACKGROUNDS.iter().find(|(id, _)| *id == choice).map(|(_, l)| *l).unwrap_or("");
    if choice == "none" {
        println!("Saved. Notes will use neutral explanations.");
    } else {
        println!("Saved. Notes will include \"coming from {label}\" sections where they exist, and neutral explanations elsewhere.");
    }
    println!("Change it any time with `learn init`.");
    if ctx.config.track.is_none() {
        println!("\nNext: pick a track with `learn start <lang>` ({}).", backends::ids().join(", "));
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

fn doctor(ctx: &Ctx) -> Result<ExitCode> {
    println!("Toolchains (learn looks on PATH and in common install folders):\n");
    let mut missing = Vec::new();
    for b in backends::all() {
        let active = ctx.config.track.as_deref() == Some(b.id());
        let marker = if active { " (active)" } else { "" };
        match b.tool().find() {
            Some(found) => {
                let version = if found.version.is_empty() { String::new() } else { format!("{}  ", found.version) };
                println!("  ✓ {:<11} {version}{}{marker}", b.id(), found.path.display());
                if let Some(warning) = b.check_tool(&found) {
                    println!("      ! {warning}");
                }
            }
            None => {
                println!("  ✗ {:<11} {} not found{marker}", b.id(), b.tool().label);
                missing.push(*b);
            }
        }
    }
    if missing.is_empty() {
        println!("\nEverything is installed.");
        return Ok(ExitCode::SUCCESS);
    }
    println!("\nTo install what's missing:");
    let keys = os_keys();
    for b in missing {
        let meta = TrackMeta::load(&ctx.root, b.id())?;
        println!("\n  {} ({})", b.name(), b.tool().label);
        let mut any = false;
        for key in &keys {
            if let Some(lines) = meta.install.get(*key) {
                if keys.len() > 1 {
                    println!("    {key}:");
                }
                for l in lines {
                    println!("      {l}");
                }
                any = true;
            }
        }
        if !any {
            println!("    See tracks/{}/README.md", b.id());
        }
    }
    println!("\nOpen a new terminal after installing, then run `learn doctor` again.");
    Ok(ExitCode::SUCCESS)
}

fn start(ctx: &mut Ctx, lang: &str) -> Result<ExitCode> {
    let backend = backends::get(lang)?;
    let dir = track_dir(ctx, backend);
    let report = backends::generate(backend, &ctx.specs, &dir)?;
    ctx.config.track = Some(backend.id().to_string());
    ctx.save_config()?;

    println!("Active track: {}", backend.name());
    println!("Workspace:    {}", display_path(&dir));
    if !report.stubs_created.is_empty() {
        println!("Created {} exercise file(s).", report.stubs_created.len());
    }
    if !report.stubs_kept.is_empty() {
        println!("Kept {} existing file(s) with your work untouched.", report.stubs_kept.len());
    }
    if backend.tool().find().is_none() {
        println!(
            "\n! {} isn't installed yet, so tests can't run. `learn doctor` shows how to install it.",
            backend.tool().label
        );
    }
    if ctx.config.from.is_none() {
        println!("\nTip: run `learn init` to get notes that compare against a language you already know.");
    }
    println!();
    show(ctx, None)
}

// ---- show / hint -------------------------------------------------------------

fn header_line(ctx: &Ctx, backend: &dyn Backend, spec: &Spec) -> String {
    let (module, n, total) = position(ctx, spec);
    let done = if ctx.is_done(backend.id(), &spec.slug) { "  ✓ done" } else { "" };
    format!("{module} {n}/{total} · {} ({}) · {}{done}", spec.title, spec.slug, backend.name())
}

fn print_brief(ctx: &Ctx, backend: &dyn Backend, spec: &Spec) {
    println!("━━ {}", header_line(ctx, backend, spec));
    println!();
    for line in backends::wrap(&spec.main.summary, 76) {
        println!("{line}");
    }
    println!();
    let dir = track_dir(ctx, backend);
    // Helper modules first: their tests run first.
    let mut label = "Edit:";
    for m in spec.modules() {
        println!("{label:<6} {}", display_path(&dir.join(backend.module_path(spec, m))));
        for f in spec.helpers.iter().filter(|h| h.module.as_deref() == Some(m)) {
            println!("       {}", backend.signature(spec, f));
        }
        label = "";
    }
    println!("{label:<6} {}", display_path(&dir.join(backend.stub_path(spec))));
    println!("       {}", backend.signature(spec, &spec.main));
    println!("Then:  learn check   (or `learn watch` to re-run on every save)");
}

fn show(ctx: &Ctx, slug: Option<&str>) -> Result<ExitCode> {
    let track = ctx.active_track()?;
    let backend = backends::get(&track)?;
    let Some(spec) = pick(ctx, &track, slug)? else {
        all_done_message(ctx, backend);
        return Ok(ExitCode::SUCCESS);
    };
    let stub = track_dir(ctx, backend).join(backend.stub_path(spec));
    if !stub.exists() {
        backends::generate(backend, &ctx.specs, &track_dir(ctx, backend))?;
    }
    print_brief(ctx, backend, spec);

    if let Some(lesson) = notes::read_concept(&ctx.root, &spec.concept, from(ctx)) {
        rule(&format!("Lesson: {}", spec.concept));
        println!("{lesson}");
    }
    let meta = TrackMeta::load(&ctx.root, backend.id())?;
    if let Some(note) = notes::read_track_note(&ctx.root, backend.id(), &spec.concept, from(ctx)) {
        rule(&format!("{} note", backend.name()));
        println!("{note}");
    }
    if let Some(links) = notes::go_deeper(&meta, &spec.concept) {
        println!("\n{links}");
    }
    let shown = ctx.progress.hints.get(&spec.slug).copied().unwrap_or(0);
    if !spec.hints.is_empty() {
        println!(
            "\nHints: {} of {} revealed. `learn hint` shows the next one.",
            shown.min(spec.hints.len()),
            spec.hints.len()
        );
    }
    Ok(ExitCode::SUCCESS)
}

fn hint(ctx: &mut Ctx, slug: Option<&str>) -> Result<ExitCode> {
    let track = ctx.active_track()?;
    let Some(spec) = pick(ctx, &track, slug)? else {
        all_done_message(ctx, backends::get(&track)?);
        return Ok(ExitCode::SUCCESS);
    };
    let (slug, total, hints) = (spec.slug.clone(), spec.hints.len(), spec.hints.clone());
    if total == 0 {
        println!("{slug} has no hints. Read the lesson with `learn show {slug}`.");
        return Ok(ExitCode::SUCCESS);
    }
    let shown = ctx.progress.hints.entry(slug.clone()).or_insert(0);
    let was_all = *shown >= total;
    *shown = (*shown + 1).min(total);
    let shown = *shown;
    ctx.save_progress()?;
    for (i, h) in hints.iter().take(shown).enumerate() {
        println!("Hint {}/{total}: {h}", i + 1);
    }
    if was_all {
        println!("\nThat's every hint for {slug}. The track note (`learn show`) has a worked example of the concept.");
    } else if shown < total {
        println!("\nRun `learn hint` again for the next one.");
    }
    Ok(ExitCode::SUCCESS)
}

// ---- check / watch -------------------------------------------------------------

fn print_result(spec: &Spec, result: &CheckResult, verbose: bool) {
    for c in &result.cases {
        let mark = match c.passed {
            Some(true) => "✓",
            Some(false) => "✗",
            None => "·",
        };
        println!("  {mark} {}", c.description);
    }
    let failed: Vec<_> = result.cases.iter().filter(|c| c.passed == Some(false)).collect();
    let all = spec.cases();
    for c in failed.iter().take(3) {
        let Some((f, case)) = all.iter().find(|(_, k)| k.index == c.index) else { continue };
        let join = |vals: &[serde_json::Value]| vals.iter().map(backends::json).collect::<Vec<_>>().join(", ");
        println!("\n  ✗ {}", c.description);
        match &case.kind {
            CaseKind::Call { input, outcome } => {
                let expected = match outcome {
                    Outcome::Value(v) => backends::json(v),
                    Outcome::Error => "an error".to_string(),
                };
                println!("    call:     {}({})\n    expected: {expected}", f.name, join(input));
            }
            CaseKind::Scenario { create, steps } => {
                for (i, args) in create.iter().enumerate() {
                    println!("    v{i} = {}({})", f.name, join(args));
                }
                for s in steps {
                    println!("    v{}({}) -> {}", s.on, join(&s.input), backends::json(&s.expected));
                }
            }
        }
        if let Some(msg) = &c.message {
            for (i, line) in msg.lines().enumerate() {
                println!("    {} {line}", if i == 0 { "failure: " } else { "         " });
            }
        }
    }
    if failed.len() > 3 {
        println!("\n  …and {} more failing case(s).", failed.len() - 3);
    }

    if result.timed_out {
        println!("\nThe tests took too long and were stopped. Look for an infinite loop.");
    }
    let show_output = verbose || (result.none_ran() && !result.passed);
    if show_output {
        if result.none_ran() && !verbose {
            println!("\nNo test cases ran, which usually means the code doesn't compile. Output of `{}`:", result.command);
            let lines: Vec<&str> = result.output.lines().collect();
            let start = lines.len().saturating_sub(40);
            for l in &lines[start..] {
                println!("  {l}");
            }
        } else {
            println!("\nOutput of `{}`:\n{}", result.command, result.output);
        }
    } else if !result.passed && !failed.is_empty() {
        println!("\n`learn check -v` shows the full test output.");
    }
}

/// Run one check, print it, and record progress. Returns whether it passed.
fn check_once(ctx: &mut Ctx, backend: &dyn Backend, slug: &str, verbose: bool) -> Result<bool> {
    let tool = check::require_tool(backend)?;
    let spec = ctx.spec(slug)?.clone();
    println!("Running {} tests for {}…", backend.name(), spec.slug);
    let result = check::run(backend, &spec, &ctx.specs, &track_dir(ctx, backend), &tool)?;
    print_result(&spec, &result, verbose);

    let total = result.cases.len();
    if result.passed {
        check::record_pass(ctx, backend.id(), &spec.slug)?;
        println!("\n✓ All {total} cases pass. {} is done.", spec.title);
        match ctx.current(backend.id()) {
            Some(next) => println!("Next up: {} ({}). Run `learn` to see it.", next.title, next.slug),
            None => all_done_message(ctx, backend),
        }
    } else {
        let passing = result.count(Some(true));
        println!("\n{passing}/{total} cases pass. Keep going: `learn hint` if you're stuck.");
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

fn watch(ctx: &mut Ctx) -> Result<ExitCode> {
    let track = ctx.active_track()?;
    let backend = backends::get(&track)?;
    check::require_tool(backend)?;
    backends::generate(backend, &ctx.specs, &track_dir(ctx, backend))?;

    let Some(mut slug) = ctx.current(&track).map(|s| s.slug.clone()) else {
        all_done_message(ctx, backend);
        return Ok(ExitCode::SUCCESS);
    };
    println!("Watching for changes. Save your file to re-run the tests; Ctrl+C to stop.\n");
    print_brief(ctx, backend, ctx.spec(&slug)?);
    println!();
    let mut passed = check_once(ctx, backend, &slug, false)?;

    loop {
        if passed {
            match ctx.current(&track).map(|s| s.slug.clone()) {
                Some(next) => {
                    slug = next;
                    println!("\n");
                    print_brief(ctx, backend, ctx.spec(&slug)?);
                    println!("\nWaiting for you to save…");
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
        println!("\n── change detected ──");
        passed = check_once(ctx, backend, &slug, false)?;
    }
}

// ---- compare / docs / generate / status -------------------------------------

fn known_concepts(ctx: &Ctx) -> Vec<String> {
    let mut v: Vec<String> = ctx.specs.iter().map(|s| s.concept.clone()).collect();
    v.dedup();
    v
}

fn resolve_concept(ctx: &Ctx, concept: &str) -> Result<String> {
    let concepts = known_concepts(ctx);
    if concepts.iter().any(|c| c == concept) {
        return Ok(concept.to_string());
    }
    // Accept an exercise slug too: `learn compare parse-port`.
    if let Ok(spec) = ctx.spec(concept) {
        return Ok(spec.concept.clone());
    }
    bail!("unknown concept `{concept}`. Concepts: {}", concepts.join(", "))
}

fn compare(ctx: &Ctx, concept: &str, langs: &[String]) -> Result<ExitCode> {
    let concept = resolve_concept(ctx, concept)?;
    let selected: Vec<&dyn Backend> = if langs.is_empty() {
        backends::all().to_vec()
    } else {
        langs.iter().map(|l| backends::get(l)).collect::<Result<_>>()?
    };
    println!("Comparing `{concept}` across {} track(s).", selected.len());
    for b in selected {
        rule(b.name());
        match notes::read_track_note(&ctx.root, b.id(), &concept, from(ctx)) {
            Some(note) => println!("{note}"),
            None => println!("(No {} note for {concept} yet. Contributions welcome: see CONTRIBUTING.md.)", b.name()),
        }
    }
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
    let track = match lang {
        Some(l) => backends::get(l)?.id().to_string(),
        None => ctx.active_track()?,
    };
    let concept = match concept {
        Some(c) => resolve_concept(ctx, c)?,
        None => match ctx.current(&track) {
            Some(s) => s.concept.clone(),
            None => bail!("name a concept: {}", known_concepts(ctx).join(", ")),
        },
    };
    let meta = TrackMeta::load(&ctx.root, &track)?;
    let links = meta.links_for(&concept);
    if links.is_empty() {
        println!("No links for `{concept}` on the {track} track yet.");
        return Ok(ExitCode::SUCCESS);
    }
    println!("Further reading for `{concept}` ({track}):\n");
    for l in links {
        println!("  {:<10} {}\n             {}", meta.kind_label(&l.kind), l.title, l.url);
    }
    if open {
        for l in links {
            open_url(&l.url)?;
        }
    } else {
        println!("\n`learn docs {concept} --open` opens them in your browser.");
    }
    Ok(ExitCode::SUCCESS)
}

fn generate(ctx: &Ctx, lang: Option<&str>) -> Result<ExitCode> {
    let selected: Vec<&dyn Backend> = match lang {
        Some("all") => backends::all().to_vec(),
        Some(l) => vec![backends::get(l)?],
        None => match &ctx.config.track {
            Some(t) => vec![backends::get(t)?],
            None => bail!("no active track. Use `learn generate --lang <lang>` or `--lang all`."),
        },
    };
    for b in selected {
        let dir = track_dir(ctx, b);
        let report = backends::generate(b, &ctx.specs, &dir)?;
        println!(
            "{:<11} tests regenerated in {}; {} new stub(s), {} kept",
            b.id(),
            display_path(&dir),
            report.stubs_created.len(),
            report.stubs_kept.len()
        );
    }
    Ok(ExitCode::SUCCESS)
}

fn status(ctx: &Ctx) -> Result<ExitCode> {
    let ids = backends::ids();
    let active = ctx.config.track.as_deref();
    let width = ctx.specs.iter().map(|s| s.slug.len()).max().unwrap_or(10).max(10);
    print!("{:width$}", "");
    for id in &ids {
        let label = if *id == "typescript" { "ts".to_string() } else { id.to_string() };
        let label = if Some(*id) == active { format!("*{label}") } else { label };
        print!("  {label:>6}");
    }
    println!();
    for m in &ctx.course.modules {
        println!("{}", m.title);
        for slug in &m.exercises {
            print!("{slug:width$}");
            for id in &ids {
                let cell = if ctx.is_done(id, slug) {
                    "✓"
                } else if Some(*id) == active && ctx.current(id).is_some_and(|s| s.slug == *slug) {
                    "▶"
                } else {
                    "·"
                };
                print!("  {cell:>6}");
            }
            println!();
        }
    }
    println!();
    let mut totals = Vec::new();
    for id in &ids {
        let n = ctx.progress.done.get(*id).map_or(0, |d| d.len());
        if n > 0 {
            totals.push(format!("{id} {n}/{}", ctx.specs.len()));
        }
    }
    if totals.is_empty() {
        println!("No exercises finished yet. ✓ done · ▶ current · * active track");
    } else {
        println!("{}   (✓ done · ▶ current · * active track)", totals.join(", "));
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
                        .args(["-A", "polyglot-learn check-links (+https://github.com/rg-digital/polyglot-learn)"])
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
        backends::generate(b, &ctx.specs, &dir)?;

        for spec in &ctx.specs {
            let r = check::run(b, spec, &ctx.specs, &dir, &tool)?;
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
            for spec in &ctx.specs {
                if !src.join(b.stub_path(spec)).exists() {
                    problems.push(format!("{}/{}: no reference solution", b.id(), spec.slug));
                    println!("   ✗ ref  {:<18} missing {}", spec.slug, b.stub_path(spec).display());
                    continue;
                }
                let r = check::run(b, spec, &ctx.specs, &dir, &tool)?;
                if r.passed {
                    println!("   ✓ ref  {:<18} passes", spec.slug);
                } else {
                    problems.push(format!("{}/{}: reference solution fails", b.id(), spec.slug));
                    println!("   ✗ ref  {:<18} fails ({}/{}):", spec.slug, r.count(Some(true)), r.cases.len());
                    print_result(spec, &r, r.none_ran());
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
