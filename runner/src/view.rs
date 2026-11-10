//! What the CLI shows: each screen built as a string from the course state and
//! a `Ui`, so it can be printed, paged or snapshot-tested with plain output.

use crate::backends::{self, Backend};
use crate::check::{CaseResult, CheckResult};
use crate::course::TrackMeta;
use crate::ctx::{Ctx, Reading};
use crate::markdown;
use crate::notes;
use crate::spec::{CaseKind, Outcome, Spec};
use crate::ui::{self, Ui, MARGIN};

/// "Foundations · 1 of 6", or empty when the exercise isn't in a module.
fn position(ctx: &Ctx, spec: &Spec, ui: &Ui) -> String {
    ctx.course
        .modules
        .iter()
        .find_map(|m| {
            let i = m.exercises.iter().position(|s| *s == spec.slug)?;
            Some(format!("{} {} {} of {}", m.title, ui.dot(), i + 1, m.exercises.len()))
        })
        .unwrap_or_default()
}

/// Left text and right text on one line, the right one aligned to the width.
fn spread(left: &str, right: &str, ui: &Ui) -> String {
    let used = ui::visible_width(left) + ui::visible_width(right);
    let gap = ui.width.saturating_sub(used).max(2);
    format!("{left}{}{right}", " ".repeat(gap))
}

fn from(ctx: &Ctx) -> Option<&str> {
    ctx.config.from.as_deref().filter(|f| *f != "none")
}

// ---- show ------------------------------------------------------------------------

/// The task card: where the exercise sits, what to do, which file, what next.
pub fn task_card(ctx: &Ctx, backend: &dyn Backend, spec: &Spec, ui: &Ui) -> String {
    let mut out = Vec::new();
    out.push(spread(&format!("{MARGIN}{}", ui.muted(&position(ctx, spec, ui))), &ui.muted(backend.name()), ui));
    let done = if ctx.is_done(backend.id(), &spec.slug) { format!("  {} {}", ui.pass_mark(), ui.success("done")) } else { String::new() };
    out.push(format!("{MARGIN}{} {} {}{done}", ui.heading(&spec.title), ui.muted(ui.dot()), ui.accent(&spec.concept)));
    out.push(String::new());
    out.extend(ui.wrap(&spec.main.summary, MARGIN, MARGIN));
    out.push(String::new());

    let dir = ctx.workspace.join(backend.id());
    let mut label = "Edit";
    for (file, sigs) in backends::learner_files(backend, spec) {
        out.push(format!("{MARGIN}{} {}", ui.heading(&format!("{label:<6}")), ui.path(&dir.join(&file))));
        for sig in sigs {
            out.push(format!("{MARGIN}       {}", ui.code(&sig)));
        }
        label = "";
    }
    out.push(String::new());

    let shown = ctx.progress.hints.get(&spec.slug).copied().unwrap_or(0).min(spec.hints.len());
    let lesson_about = if spec.native.is_some() { "read the lesson".to_string() } else { format!("read about {}", spec.concept) };
    let hints = match (spec.hints.len(), shown) {
        (0, _) => None,
        (n, 0) => Some(format!("a nudge if you're stuck ({n} hint{})", if n == 1 { "" } else { "s" })),
        (n, s) if s >= n => Some(format!("all {n} hints shown")),
        (n, s) => Some(format!("next hint ({s} of {n} shown)")),
    };
    let mut cmds = vec![("learn lesson", lesson_about)];
    if let Some(h) = hints {
        cmds.push(("learn hint", h));
    }
    cmds.push(("learn check", format!("run the tests {} learn watch re-runs on every save", ui.dot())));
    let col = " ".repeat(MARGIN.len() + 15);
    for (cmd, what) in cmds {
        let lines = ui.wrap(&what, &col, &col);
        for (i, l) in lines.iter().enumerate() {
            let text = ui.muted(l.trim_start());
            if i == 0 {
                out.push(format!("{MARGIN}{}{}{text}", ui.accent(cmd), " ".repeat(15 - cmd.len())));
            } else {
                out.push(format!("{col}{text}"));
            }
        }
    }
    out.join("\n")
}

/// The finished-the-track message.
pub fn all_done(ctx: &Ctx, backend: &dyn Backend, ui: &Ui) -> String {
    let total = ctx.specs.iter().filter(|s| s.on_track(backend.id())).count();
    [
        format!("{MARGIN}{} {}", ui.pass_mark(), ui.success(&format!("You've finished every {} exercise ({total} of {total}).", backend.name()))),
        String::new(),
        format!("{MARGIN}{}   {}", ui.accent("learn start <lang>"), ui.muted("try another track")),
        format!("{MARGIN}{}  {}", ui.accent("learn compare <concept>"), ui.muted("see one idea in every language")),
    ]
    .join("\n")
}

// ---- lesson ------------------------------------------------------------------------

/// The concept lesson, the track's note (for the learner's background) and the
/// further-reading links, rendered for the terminal.
pub fn lesson(ctx: &Ctx, backend: &dyn Backend, spec: &Spec, ui: &Ui) -> String {
    let mut parts = Vec::new();
    if spec.native.is_some() {
        parts.extend(notes::read_native_lesson(spec, from(ctx)));
    } else {
        parts.extend(notes::read_concept(&ctx.root, &spec.concept, from(ctx)));
        parts.extend(notes::read_track_note(&ctx.root, backend.id(), &spec.concept, from(ctx)));
    }
    let rule = ui.muted(&(if ui.unicode { "─" } else { "-" }).repeat(ui.width.saturating_sub(MARGIN.len())));
    let mut out: Vec<String> = parts.iter().map(|md| markdown::render(md, ui)).collect::<Vec<_>>();
    if out.is_empty() {
        out.push(format!("{MARGIN}{}", ui.muted(&format!("No lesson for {} yet.", spec.concept))));
    }
    let mut text = out.join(&format!("\n\n{MARGIN}{rule}\n\n"));
    if let Ok(meta) = TrackMeta::load(&ctx.root, backend.id()) {
        let links = go_deeper(&meta, &spec.concept, ui);
        if !links.is_empty() {
            text.push_str("\n\n");
            text.push_str(&links);
        }
    }
    text
}

// ---- readings ------------------------------------------------------------------------

/// The readings a learner on `track` (or on no track yet) works through.
pub fn readings_for<'a>(ctx: &'a Ctx, track: Option<&str>) -> Vec<&'a Reading> {
    match track {
        Some(t) => ctx.readings_on(t),
        None => ctx.readings.iter().collect(),
    }
}

/// The reading after `id`, in course order.
pub fn next_reading<'a>(ctx: &'a Ctx, track: Option<&str>, id: &str) -> Option<&'a Reading> {
    let list = readings_for(ctx, track);
    let i = list.iter().position(|r| r.id == id)?;
    list.get(i + 1).copied()
}

/// A reading page: where it sits, the concept page, the track's note, what next.
pub fn reading(ctx: &Ctx, r: &Reading, backend: Option<&dyn Backend>, ui: &Ui) -> String {
    let track = backend.map(|b| b.id());
    let list = readings_for(ctx, track);
    let i = list.iter().position(|x| x.id == r.id).map_or(0, |i| i + 1);
    let module = &ctx.course.modules[r.module].title;
    let rule = ui.muted(&(if ui.unicode { "─" } else { "-" }).repeat(ui.width.saturating_sub(MARGIN.len())));
    let mut out = vec![format!("{MARGIN}{}", ui.muted(&format!("{module} {} {i} of {}", ui.dot(), list.len())))];
    out.extend(notes::read_concept(&ctx.root, &r.id, from(ctx)).map(|md| markdown::render(&md, ui)));
    match backend {
        Some(b) => match notes::read_track_note(&ctx.root, b.id(), &r.id, from(ctx)) {
            Some(md) => out.push(format!("{MARGIN}{rule}\n\n{}", markdown::render(&md, ui))),
            None => out.push(format!("{MARGIN}{}", ui.muted(&format!("No {} note for this page yet.", b.name())))),
        },
        None => out.push(format!("{MARGIN}{} {}", ui.accent("learn start <lang>"), ui.muted("adds your language's note to each page"))),
    }
    let mut footer = vec![format!("{MARGIN}{rule}")];
    match next_reading(ctx, track, &r.id) {
        Some(n) => footer.push(format!("{MARGIN}{}  {}", ui.accent(&format!("learn read {}", n.id)), ui.muted(&format!("next: {}", n.title)))),
        None if track.is_some() => footer.push(format!("{MARGIN}{}  {}", ui.accent("learn"), ui.muted("that was the last page: on to the first exercise"))),
        None => footer.push(format!("{MARGIN}{}  {}", ui.accent("learn start <lang>"), ui.muted("that was the last page: pick a track for the exercises"))),
    }
    footer.push(format!("{MARGIN}{}  {}", ui.accent(&format!("learn compare {}", r.id)), ui.muted("every track side by side")));
    out.push(footer.join("\n"));
    out.join("\n\n")
}

/// Every reading with its mark, for `learn read` once they're all read.
pub fn reading_list(ctx: &Ctx, track: Option<&str>, ui: &Ui) -> String {
    let mut out = Vec::new();
    for r in readings_for(ctx, track) {
        let mark = if ctx.is_read(&r.id) { ui.pass_mark() } else { ui.todo_mark() };
        out.push(format!("{MARGIN}{mark} {}  {}", r.title, ui.muted(&r.id)));
    }
    out.push(String::new());
    out.push(format!("{MARGIN}{}  {}", ui.accent("learn read <page>"), ui.muted("to read one again")));
    out.join("\n")
}

/// The nudge towards the primer, while nothing is done on the track and nothing is read.
pub fn primer_hint(ctx: &Ctx, track: &str, ui: &Ui) -> Option<String> {
    if !ctx.is_new(track) {
        return None;
    }
    let list = ctx.readings_on(track);
    let module = &ctx.course.modules[list.first()?.module].title;
    let text = format!(
        "New to this? The {module} is {} short pages on how each language writes the basics: variables, types, loops and functions. Skip it if you know them.",
        list.len()
    );
    let mut out: Vec<String> = ui.wrap(&text, MARGIN, MARGIN).into_iter().map(|l| ui.muted(&l)).collect();
    out.push(format!("{MARGIN}{}  {}", ui.accent("learn read"), ui.muted("to start reading")));
    Some(out.join("\n"))
}

/// "Go deeper": one clickable line per link.
pub fn go_deeper(meta: &TrackMeta, concept: &str, ui: &Ui) -> String {
    let links = meta.links_for(concept);
    if links.is_empty() {
        return String::new();
    }
    let labels: Vec<String> = links.iter().map(|l| meta.kind_label(&l.kind)).collect();
    let pad = labels.iter().map(|l| l.chars().count()).max().unwrap_or(0) + 2;
    let mut out = vec![format!("{MARGIN}{}", ui.paint(ui::heading_style().fg_color(ui::accent_style().get_fg_color()), "Go deeper"))];
    for (l, label) in links.iter().zip(&labels) {
        let first = format!("{MARGIN}{}{}", ui.muted(label), " ".repeat(pad - label.chars().count()));
        let rest = " ".repeat(MARGIN.len() + pad);
        // Each word of the title links to the page; without link support the URL follows in brackets.
        let mut words: Vec<ui::Word> = l
            .title
            .split_whitespace()
            .map(|w| ui::Word { text: w.to_string(), style: Some(ui::accent_style()), link: Some(l.url.clone()), glue: false })
            .collect();
        if !ui.links {
            words.push(ui::Word { text: format!("({})", l.url), style: Some(ui::muted_style()), link: None, glue: false });
        }
        out.extend(ui::wrap_words(&words, ui.width, &first, &rest, ui));
    }
    out.join("\n")
}

// ---- hint --------------------------------------------------------------------------

/// The hints revealed so far: earlier ones dimmed, the newest one bright.
pub fn hints(spec: &Spec, shown: usize, was_all: bool, ui: &Ui) -> String {
    let total = spec.hints.len();
    let mut out = Vec::new();
    for (i, h) in spec.hints.iter().take(shown).enumerate() {
        let n = format!("{}", i + 1);
        let newest = i + 1 == shown && !was_all;
        // Wrapped so the text column (after the number) stays within the width.
        let pad = " ".repeat(MARGIN.len() + n.len() + 2);
        let lines: Vec<String> = ui.wrap(h, &pad, &pad).iter().map(|l| l.trim_start().to_string()).collect();
        for (k, line) in lines.iter().enumerate() {
            let num = if k == 0 { if newest { ui.accent(&n) } else { ui.muted(&n) } } else { " ".repeat(n.len()) };
            let text = if newest { ui.heading(line) } else { ui.muted(line) };
            out.push(format!("{MARGIN}{num}  {text}"));
        }
    }
    out.push(String::new());
    let footer = if was_all {
        format!("That's every hint for {} {} learn lesson has a worked example", spec.title, ui.dot())
    } else if shown < total {
        format!("Hint {shown} of {total} {} learn hint for the next", ui.dot())
    } else {
        format!("Hint {shown} of {total} {} that was the last one", ui.dot())
    };
    out.extend(ui.wrap(&footer, MARGIN, MARGIN).iter().map(|l| ui.muted(l)));
    out.join("\n")
}

// ---- check -------------------------------------------------------------------------

/// The value the code actually produced, from a test framework's failure
/// message: Rust's `left:`, Go's `got … want`, PHP/Lua's `expected …, got …`,
/// QML's `Actual (…):` and Node's `+ actual - expected` diff.
pub fn actual_value(message: &str) -> Option<String> {
    let lines: Vec<&str> = message.lines().map(str::trim).collect();
    for (i, l) in lines.iter().enumerate() {
        if let Some(v) = l.strip_prefix("left:") {
            return Some(v.trim().to_string());
        }
        if let Some(v) = l.strip_prefix("Actual") {
            // After a check's own message, the values belong to that check, not the result.
            if lines[..i].iter().any(|p| !p.is_empty() && *p != "Compared values are not the same") {
                return None;
            }
            return v.split_once("):").map(|(_, v)| v.trim().to_string());
        }
        if *l == "+ actual - expected" {
            let (plus, minus) = (lines.get(i + 1)?, lines.get(i + 2)?);
            if plus.starts_with("+ ") && minus.starts_with("- ") && lines.get(i + 3).is_none_or(|n| !n.starts_with(['+', '-'])) {
                return Some(plus[2..].trim().to_string());
            }
            return None;
        }
        if l.starts_with("expected ") {
            if let Some((_, got)) = l.rsplit_once(", got ") {
                return Some(got.trim().to_string());
            }
        }
        if let Some(rest) = l.strip_prefix("got ") {
            if let Some((got, _)) = rest.rsplit_once(", want ") {
                return Some(got.trim().to_string());
            }
        }
    }
    None
}

/// The first line of a failure message that says something, without the
/// frameworks' standard headers.
pub fn first_meaningful_line(message: &str) -> Option<String> {
    const NOISE: &[&str] = &[
        "assertion `left == right` failed",
        "Expected values to be strictly deep-equal:",
        "Expected values to be strictly equal:",
        "+ actual - expected",
        "Compared values are not the same",
    ];
    message
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty() && !NOISE.contains(l) && !l.starts_with("at ") && !l.starts_with("stack"))
        .map(|l| l.chars().take(200).collect())
}

/// Strip one layer of matching quotes.
fn unquote(s: &str) -> Option<&str> {
    for q in ['"', '\''] {
        if s.len() >= 2 && s.starts_with(q) && s.ends_with(q) {
            return Some(&s[1..s.len() - 1]);
        }
    }
    None
}

/// `got`, with the part that differs from `expected` highlighted, when both
/// are strings. Other values are shown as they are.
fn highlight_difference(expected: &str, got: &str, ui: &Ui) -> String {
    let (Some(e), g) = (unquote(expected), unquote(got).unwrap_or(got)) else { return ui.failure(got) };
    let (ec, gc): (Vec<char>, Vec<char>) = (e.chars().collect(), g.chars().collect());
    let pre = ec.iter().zip(&gc).take_while(|(a, b)| a == b).count();
    let max_suf = ec.len().min(gc.len()) - pre;
    let suf = ec.iter().rev().zip(gc.iter().rev()).take(max_suf).take_while(|(a, b)| a == b).count();
    let quote = if got.starts_with('\'') { "'" } else if got.starts_with('"') { "\"" } else { "" };
    let s = |v: &[char]| v.iter().collect::<String>();
    let middle = s(&gc[pre..gc.len() - suf]);
    let middle = if middle.is_empty() { ui.paint(ui::failure_style().effects(anstyle::Effects::UNDERLINE), "‸") } else { ui.paint(ui::failure_style().effects(anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE), &middle) };
    if !ui.color {
        return got.to_string();
    }
    format!("{quote}{}{middle}{}{quote}", s(&gc[..pre]), s(&gc[gc.len() - suf..]))
}

/// The first compiler or runtime error in output where no test ran, trimmed
/// to about ten lines, and how many lines were left out.
pub fn first_error(output: &str) -> (Vec<String>, usize) {
    // Normalise: TAP reporters prefix the test file's own output with "# ";
    // their YAML diagnostics (between "---" and "...") and summary counts are noise.
    let mut lines: Vec<String> = Vec::new();
    let mut in_yaml = false;
    for raw in output.lines() {
        let t = raw.trim();
        if t == "---" {
            in_yaml = true;
            continue;
        }
        if in_yaml {
            in_yaml = t != "...";
            continue;
        }
        let l = raw.strip_prefix("# ").unwrap_or(raw);
        let lt = l.trim();
        let summary = ["tests ", "suites ", "pass ", "fail ", "cancelled ", "skipped ", "todo ", "duration_ms", "Subtest:"];
        if raw.starts_with("# ") && summary.iter().any(|s| lt.starts_with(s)) {
            continue;
        }
        if lt.starts_with("TAP version") || lt.starts_with("1..") || lt.starts_with("not ok ") || lt.starts_with("ok ") {
            continue;
        }
        lines.push(l.trim_end().to_string());
    }
    let total = lines.iter().filter(|l| !l.trim().is_empty()).count();
    let frame = |l: &str| {
        let t = l.trim_start();
        t.starts_with("at ") || t.starts_with("stack traceback") || t.starts_with("[C]:") || t.contains("node:internal")
    };
    let has_pos = |l: &str, sep: char| {
        // `name.ext:12:3:` (Go) or `name.qml:12,3:` (QML)
        l.split(':').skip(1).any(|p| {
            let p = p.trim();
            !p.is_empty() && p.split(sep).all(|n| n.parse::<u32>().is_ok())
        })
    };

    let picked: Vec<String> = if let Some(i) = lines
        .iter()
        .position(|l| (l.starts_with("error[") || l.starts_with("error:")) && !l.contains("could not compile") && !l.contains("test failed"))
    {
        // rustc: the first error block, up to the blank line after it.
        lines[i..].iter().take_while(|l| !l.trim().is_empty()).take(12).cloned().collect()
    } else if lines.iter().any(|l| l.contains(".qml:") && has_pos(l, ',')) {
        // QML: each `File.qml:line,col: message`, without the runner's banner.
        let mut v: Vec<String> = lines.iter().filter(|l| l.contains(".qml:") && has_pos(l, ',')).map(|l| l.trim().to_string()).collect();
        v.dedup();
        v.truncate(6);
        v
    } else if let Some(i) = lines.iter().position(|l| l.contains(".go:") && has_pos(l, ' ')) {
        // Go: consecutive `file.go:line:col: message` lines.
        lines[i..].iter().take_while(|l| l.contains(".go:")).take(10).cloned().collect()
    } else if let Some(i) = lines.iter().position(|l| {
        let t = l.trim();
        !frame(l) && (t.contains("Error") || t.contains("error") || t.contains("lua:") || t.starts_with("FAIL!"))
    }) {
        // Anything else: from the file reference just above the error (Node
        // prints file, code and a caret first) down to the stack trace.
        let start = (i.saturating_sub(5)..i).find(|&k| lines[k].trim_start().starts_with("file:") || has_pos(&lines[k], ' ')).unwrap_or(i);
        lines[start..].iter().take_while(|l| !frame(l)).take(10).cloned().collect()
    } else {
        let kept: Vec<String> = lines.iter().filter(|l| !l.trim().is_empty()).cloned().collect();
        kept[kept.len().saturating_sub(10)..].to_vec()
    };
    // No blank lines at the start or end of the excerpt.
    let mut picked = picked;
    while picked.first().is_some_and(|l| l.trim().is_empty()) {
        picked.remove(0);
    }
    while picked.last().is_some_and(|l| l.trim().is_empty()) {
        picked.pop();
    }
    let shown = picked.iter().filter(|l| !l.trim().is_empty()).count();
    (picked, total.saturating_sub(shown))
}

fn case_line(c: &CaseResult, ui: &Ui) -> Option<String> {
    match c.passed {
        Some(true) => Some(format!("{MARGIN}{} {}", ui.pass_mark(), c.description)),
        Some(false) => Some(format!("{MARGIN}{} {}", ui.fail_mark(), ui.failure(&c.description))),
        None => None,
    }
}

/// Details for one failing case: the call, what was expected, what came back.
fn failure_detail(spec: &Spec, c: &CaseResult, verbose: bool, ui: &Ui) -> Vec<String> {
    let mut out = vec![format!("{MARGIN}{} {}", ui.fail_mark(), ui.failure(&ui.heading(&c.description)))];
    let label = |l: &str| ui.muted(&format!("{l:<9}"));
    let ind = format!("{MARGIN}  ");
    let join = |vals: &[serde_json::Value]| vals.iter().map(backends::json).collect::<Vec<_>>().join(", ");
    let all = spec.cases();
    let mut expected_value = None;
    if let Some((f, case)) = all.iter().find(|(_, k)| k.index == c.index) {
        if spec.native.is_none() {
            match &case.kind {
                CaseKind::Call { input, outcome } => {
                    out.push(format!("{ind}{} {}", label("call"), ui.code(&format!("{}({})", f.name, join(input)))));
                    let expected = match outcome {
                        Outcome::Value(v) => backends::json(v),
                        Outcome::Error(contains) => crate::spec::describe_error(contains),
                    };
                    out.push(format!("{ind}{} {}", label("expected"), ui.success(&expected)));
                    expected_value = Some(expected);
                }
                CaseKind::Scenario { create, steps } => {
                    for (i, args) in create.iter().enumerate() {
                        out.push(format!("{ind}{} {}", label(if i == 0 { "calls" } else { "" }), ui.code(&format!("v{i} = {}({})", f.name, join(args)))));
                    }
                    for s in steps {
                        out.push(format!("{ind}{} {}", label(""), ui.code(&format!("v{}({}) -> {}", s.on, join(&s.input), backends::json(&s.expected)))));
                    }
                }
            }
        }
    }
    if let Some(msg) = &c.message {
        let got = actual_value(msg).filter(|_| expected_value.is_some() || spec.native.is_none());
        match (&got, &expected_value) {
            (Some(g), Some(e)) => out.push(format!("{ind}{} {}", label("got"), highlight_difference(e, g, ui))),
            _ => {
                if let Some(line) = first_meaningful_line(msg) {
                    out.push(format!("{ind}{} {}", label("failure"), line));
                }
            }
        }
        if verbose {
            for line in msg.lines().filter(|l| !l.trim().is_empty()) {
                out.push(format!("{ind}{}", ui.muted(line.trim_end())));
            }
        }
    }
    out
}

/// Paths inside the track folder, as compilers print them (absolute, with
/// either slash, or as a file:// URL), shortened to the part after it.
fn shorten_paths(line: &str, track_dir: &std::path::Path) -> String {
    let abs = std::path::absolute(track_dir).unwrap_or_else(|_| track_dir.to_path_buf()).display().to_string();
    let abs = abs.trim_end_matches(['/', '\\']).to_string();
    let forward = abs.replace('\\', "/");
    let mut out = line.to_string();
    for prefix in [format!("file:///{forward}/"), format!("file://{forward}/"), format!("{abs}{}", std::path::MAIN_SEPARATOR), format!("{forward}/")] {
        out = out.replace(&prefix, "");
    }
    out
}

/// What `learn check` prints for a run. `track_dir` shortens paths in compiler output.
pub fn check_report(spec: &Spec, result: &CheckResult, verbose: bool, track_dir: &std::path::Path, ui: &Ui) -> String {
    let total = result.cases.len();
    let passing = result.count(Some(true));
    let dot = ui.dot();
    let mut out = Vec::new();

    if result.passed {
        out.push(format!("{MARGIN}{} {}", ui.pass_mark(), ui.paint(ui::success_style().effects(anstyle::Effects::BOLD), &format!("{} {dot} all {total} passing", spec.slug))));
    } else if result.timed_out {
        out.push(format!(
            "{MARGIN}{} {}",
            ui.heading(&spec.slug),
            ui.warning(&format!("{dot} stopped after {} s: look for an infinite loop", result.timeout_secs))
        ));
    } else if result.none_ran() {
        out.push(format!("{MARGIN}{} {}", ui.heading(&spec.slug), ui.failure(&format!("{dot} no tests ran"))));
    } else {
        out.push(format!(
            "{MARGIN}{} {dot} {}/{total} passing  {}",
            ui.heading(&spec.slug),
            passing,
            ui.bar(passing, total, 20)
        ));
    }
    out.push(String::new());

    if result.none_ran() && !result.timed_out {
        if let Some(problem) = &result.setup_problem {
            out.extend(ui.wrap(problem, MARGIN, MARGIN));
        } else {
            let (lines, more) = first_error(&result.output);
            out.extend(lines.iter().map(|l| format!("{MARGIN}{}", shorten_paths(l, track_dir))));
            if !verbose {
                out.push(String::new());
                let more = if more > 0 { format!("{more} more lines {dot} ") } else { String::new() };
                out.push(format!("{MARGIN}{}", ui.muted(&format!("{more}learn check -v for the full output"))));
            }
        }
    } else {
        // All green needs no list (unless asked for); otherwise every case, marked.
        if !result.passed || verbose {
            out.extend(result.cases.iter().filter_map(|c| case_line(c, ui)));
        }
        let failed: Vec<&CaseResult> = result.cases.iter().filter(|c| c.passed == Some(false)).collect();
        let detail_count = if verbose { failed.len() } else { 1 };
        for c in failed.iter().take(detail_count) {
            out.push(String::new());
            out.extend(failure_detail(spec, c, verbose, ui));
        }
        if !verbose && !result.passed {
            out.push(String::new());
            let more = failed.len().saturating_sub(1);
            let mut parts = Vec::new();
            if more > 0 {
                parts.push(format!("{more} more failing"));
            }
            parts.push("learn check -v for all details".to_string());
            if !result.timed_out {
                parts.push("stuck? learn hint".to_string());
            }
            for l in ui.wrap(&parts.join(&format!(" {dot} ")), MARGIN, MARGIN) {
                out.push(ui.muted(&l));
            }
        }
    }
    if verbose && !result.passed {
        out.push(String::new());
        out.push(format!("{MARGIN}{}", ui.muted(&format!("Output of `{}`:", result.command))));
        out.push(result.output.trim_end().to_string());
    }
    while out.last().is_some_and(|l| l.is_empty()) {
        out.pop();
    }
    out.join("\n")
}

/// After a pass: the next exercise, briefly.
pub fn next_up(ctx: &Ctx, next: &Spec, ui: &Ui) -> String {
    let mut out = vec![
        format!("{MARGIN}{}  {}", ui.heading("Next up"), ui.muted(&position(ctx, next, ui))),
        format!("{MARGIN}{} {} {}", ui.heading(&next.title), ui.muted(ui.dot()), ui.accent(&next.concept)),
    ];
    out.extend(ui.wrap(&next.main.summary, MARGIN, MARGIN));
    out.push(format!("{MARGIN}{} {}", ui.accent("learn show"), ui.muted("to start it")));
    out.join("\n")
}

// ---- status ------------------------------------------------------------------------

/// The active track, one with finished exercises, or one `learn start` set up.
fn started(ctx: &Ctx, track: &str) -> bool {
    ctx.config.track.as_deref() == Some(track)
        || ctx.progress.done.get(track).is_some_and(|d| !d.is_empty())
        || ctx.workspace.join(track).is_dir()
}

fn mark(ctx: &Ctx, track: &str, slug: &str, ui: &Ui) -> String {
    if ctx.is_done(track, slug) {
        ui.pass_mark()
    } else if started(ctx, track) && ctx.current(track).is_some_and(|s| s.slug == slug) {
        ui.current_mark()
    } else {
        ui.todo_mark()
    }
}

fn title_of<'a>(ctx: &'a Ctx, slug: &'a str) -> &'a str {
    ctx.spec(slug).map(|s| s.title.as_str()).unwrap_or(slug)
}

/// One track's progress: each module with a bar, each exercise by title.
pub fn status_track(ctx: &Ctx, backend: &dyn Backend, ui: &Ui) -> String {
    let id = backend.id();
    let modules: Vec<_> = ctx.course.modules.iter().filter(|m| m.on_track(id)).collect();
    let total: usize = modules.iter().map(|m| m.exercises.len()).sum();
    let done: usize = modules.iter().flat_map(|m| &m.exercises).filter(|s| ctx.is_done(id, s)).count();
    let name_w = modules.iter().map(|m| m.title.chars().count()).max().unwrap_or(0) + 2;
    let mut out = vec![format!("{MARGIN}{} {} {}", ui.heading(backend.name()), ui.muted(ui.dot()), ui.muted(&format!("{done} of {total} done"))), String::new()];
    let first_unread = ctx.readings_on(id).into_iter().find(|r| !ctx.is_read(&r.id)).map(|r| r.id.as_str());
    for m in modules {
        if !m.readings.is_empty() {
            if m.exercises.is_empty() {
                let r = m.readings.iter().filter(|r| ctx.is_read(r)).count();
                out.push(format!(
                    "{MARGIN}{}{}{}  {}",
                    ui.heading(&m.title),
                    " ".repeat(name_w - m.title.chars().count()),
                    ui.bar(r, m.readings.len(), 12),
                    ui.muted(&format!("{r}/{} read", m.readings.len()))
                ));
            }
            for rid in &m.readings {
                let title = ctx.readings.iter().find(|x| x.id == *rid).map_or(rid.as_str(), |x| x.title.as_str());
                let (mark, title) = if ctx.is_read(rid) {
                    (ui.pass_mark(), title.to_string())
                } else if first_unread == Some(rid.as_str()) {
                    (ui.todo_mark(), ui.accent(title))
                } else {
                    (ui.todo_mark(), ui.muted(title))
                };
                out.push(format!("{MARGIN}  {mark} {title}"));
            }
            if m.exercises.is_empty() {
                continue;
            }
        }
        let d = m.exercises.iter().filter(|s| ctx.is_done(id, s)).count();
        out.push(format!(
            "{MARGIN}{}{}{}  {}",
            ui.heading(&m.title),
            " ".repeat(name_w - m.title.chars().count()),
            ui.bar(d, m.exercises.len(), 12),
            ui.muted(&format!("{d}/{}", m.exercises.len()))
        ));
        for slug in &m.exercises {
            let title = title_of(ctx, slug);
            let is_current = ctx.current(id).is_some_and(|s| s.slug == *slug);
            let title = if is_current { ui.accent(title) } else if ctx.is_done(id, slug) { title.to_string() } else { ui.muted(title) };
            out.push(format!("{MARGIN}  {} {title}", mark(ctx, id, slug, ui)));
        }
    }
    out.push(String::new());
    out.push(format!("{MARGIN}{}", ui.muted("learn status --all for every track")));
    out.join("\n")
}

/// Every track side by side. A track-only module is folded to one line until
/// that track has been started.
pub fn status_all(ctx: &Ctx, ui: &Ui) -> String {
    let tracks: Vec<&dyn Backend> = backends::all().to_vec();
    let labels: Vec<&str> = tracks
        .iter()
        .map(|b| match b.id() {
            "typescript" => "ts",
            "python" => "py",
            id => id,
        })
        .collect();
    // Columns as narrow as the labels allow on a small terminal; titles get the rest.
    let label_w = labels.iter().map(|l| l.len()).max().unwrap_or(4);
    let col = if ui.width >= 72 { label_w + 2 } else { label_w + 1 };
    let room = ui.width.saturating_sub(MARGIN.len() + 2 + col * tracks.len());
    let title_w = (ctx.specs.iter().map(|s| s.title.chars().count()).max().unwrap_or(10) + 2).min(26).min(room).max(10);
    let cut = |t: &str| -> String {
        if t.chars().count() <= title_w - 2 { t.to_string() } else { format!("{}…", t.chars().take(title_w - 3).collect::<String>()) }
    };
    let mut out = Vec::new();
    let head: String = labels
        .iter()
        .zip(&tracks)
        .map(|(l, b)| {
            let label = format!("{l:<col$}");
            if ctx.config.track.as_deref() == Some(b.id()) { ui.accent(&label) } else { ui.muted(&label) }
        })
        .collect();
    out.push(format!("{MARGIN}  {}{head}", " ".repeat(title_w)).trim_end().to_string());
    for m in &ctx.course.modules {
        if m.exercises.is_empty() {
            // Readings only: read marks aren't per track, so one line.
            let r = m.readings.iter().filter(|r| ctx.is_read(r)).count();
            out.push(format!("{MARGIN}{}  {}", ui.heading(&m.title), ui.muted(&format!("{r} of {} read", m.readings.len()))));
            continue;
        }
        let only = m.track.as_deref();
        if let Some(t) = only {
            if !started(ctx, t) {
                out.push(format!("{MARGIN}{}  {}", ui.heading(&m.title), ui.muted(&format!("{} exercises {} not started", m.exercises.len(), ui.dot()))));
                continue;
            }
        }
        out.push(format!("{MARGIN}{}", ui.heading(&m.title)));
        for slug in &m.exercises {
            let title = cut(title_of(ctx, slug));
            let cells: String = tracks
                .iter()
                .map(|b| {
                    let cell = if m.on_track(b.id()) { mark(ctx, b.id(), slug, ui) } else { String::new() };
                    format!("{cell}{}", " ".repeat(col.saturating_sub(ui::visible_width(&cell))))
                })
                .collect();
            out.push(format!("{MARGIN}  {title}{}{}", " ".repeat(title_w - title.chars().count()), cells.trim_end()));
        }
    }
    out.push(String::new());
    out.push(format!(
        "{MARGIN}{}",
        ui.muted(&format!("{} done  {} up next  {} to do", ui::strip_escapes(&ui.pass_mark()), ui::strip_escapes(&ui.current_mark()), ui::strip_escapes(&ui.todo_mark())))
    ));
    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A course with a two-page primer and one exercise module, on the repo's
    /// specs, with nothing done and nothing read.
    fn primer_ctx() -> Ctx {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let course: crate::course::Course = toml::from_str(
            "[[module]]\nid = \"p\"\ntitle = \"Syntax Primer\"\nreadings = [\"syntax-one\", \"syntax-two\"]\n\
             [[module]]\nid = \"f\"\ntitle = \"Foundations\"\nexercises = [\"hello\"]\n",
        )
        .unwrap();
        let spec = crate::spec::Spec::load(&root.join("specs").join("hello.json")).unwrap();
        let readings = ["syntax-one", "syntax-two"]
            .iter()
            .map(|id| crate::ctx::Reading { id: id.to_string(), title: format!("Page {id}"), module: 0 })
            .collect();
        Ctx {
            workspace: root.join("workspace"),
            state_dir: std::env::temp_dir().join("learn-view-tests-unused"),
            root,
            external_workspace: false,
            course,
            specs: vec![spec],
            readings,
            config: Default::default(),
            progress: Default::default(),
        }
    }

    #[test]
    fn status_lists_the_primer_without_counting_it_as_done() {
        let mut ctx = primer_ctx();
        ctx.progress.read.insert("syntax-one".into());
        let rust = backends::get("rust").unwrap();
        let out = status_track(&ctx, rust, &Ui::plain(80));
        assert!(out.contains("0 of 1 done"), "{out}");
        assert!(out.contains("1/2 read"), "{out}");
        assert!(out.contains("Page syntax-two"), "{out}");
        assert!(status_all(&ctx, &Ui::plain(100)).contains("1 of 2 read"));
    }

    #[test]
    fn primer_hint_goes_after_one_page_or_one_exercise() {
        let mut ctx = primer_ctx();
        assert!(primer_hint(&ctx, "rust", &Ui::plain(80)).is_some_and(|h| h.contains("learn read")));
        ctx.progress.read.insert("syntax-two".into());
        assert!(primer_hint(&ctx, "rust", &Ui::plain(80)).is_none());
        ctx.progress.read.clear();
        ctx.progress.done.entry("rust".into()).or_default().insert("hello".into());
        assert!(primer_hint(&ctx, "rust", &Ui::plain(80)).is_none());
        // Another track is still new.
        assert!(primer_hint(&ctx, "go", &Ui::plain(80)).is_some());
    }

    #[test]
    fn reading_footer_points_at_the_next_page_then_the_exercises() {
        let ctx = primer_ctx();
        let rust = backends::get("rust").unwrap();
        let ui = Ui::plain(80);
        let first = reading(&ctx, &ctx.readings[0], Some(rust), &ui);
        assert!(first.contains("Syntax Primer · 1 of 2") || first.contains("Syntax Primer | 1 of 2"), "{first}");
        assert!(first.contains("learn read syntax-two"), "{first}");
        let last = reading(&ctx, &ctx.readings[1], Some(rust), &ui);
        assert!(last.contains("on to the first exercise"), "{last}");
        let no_track = reading(&ctx, &ctx.readings[1], None, &ui);
        assert!(no_track.contains("learn start <lang>"), "{no_track}");
    }

    #[test]
    fn readings_resolve_with_or_without_their_prefix() {
        let ctx = primer_ctx();
        assert_eq!(ctx.reading("syntax-two").map(|r| r.id.as_str()), Some("syntax-two"));
        assert_eq!(ctx.reading("two").map(|r| r.id.as_str()), Some("syntax-two"));
        assert!(ctx.reading("three").is_none());
    }

    #[test]
    fn reads_the_actual_value_in_every_framework() {
        assert_eq!(actual_value("assertion `left == right` failed\n  left: \"Hello, !\"\n right: \"Hello, world!\"").as_deref(), Some("\"Hello, !\""));
        assert_eq!(actual_value("got \"Hello, !\", want \"Hello, world!\"").as_deref(), Some("\"Hello, !\""));
        assert_eq!(actual_value("expected \"Hello, world!\", got \"Hello, !\"").as_deref(), Some("\"Hello, !\""));
        assert_eq!(actual_value("Compared values are not the same\nActual   (): Hello, !\nExpected (): Hello, world!").as_deref(), Some("Hello, !"));
        assert_eq!(
            actual_value("Expected values to be strictly deep-equal:\n+ actual - expected\n\n+ 'Hello, !'\n- 'Hello, world!'\n^").as_deref(),
            None,
            "a blank line sits between the header and the values"
        );
        assert_eq!(actual_value("Expected values to be strictly deep-equal:\n+ actual - expected\n+ 'Hello, !'\n- 'Hello, world!'\n^").as_deref(), Some("'Hello, !'"));
        assert_eq!(actual_value("the input list was modified: it is now []int{3, 2, 1}"), None);
        assert_eq!(actual_value("the input list was modified
Actual   (): [1]
Expected (): [2]"), None);
    }

    #[test]
    fn meaningful_line_skips_headers_and_stack_frames() {
        assert_eq!(first_meaningful_line("assertion `left == right` failed\n  left: 1").as_deref(), Some("left: 1"));
        assert_eq!(first_meaningful_line("the input list was modified\n    at Test.run (x.ts:3)").as_deref(), Some("the input list was modified"));
    }

    #[test]
    fn highlights_only_the_differing_part() {
        let ui = Ui { color: true, ..Ui::plain(80) };
        let out = highlight_difference("\"Hello, world!\"", "\"Hello, !\"", &ui);
        assert!(out.starts_with("\"Hello, ") && out.ends_with("!\""), "{out:?}");
        assert!(out.contains('‸'), "an empty middle shows where text is missing: {out:?}");
        let plain = highlight_difference("\"a\"", "\"b\"", &Ui::plain(80));
        assert_eq!(plain, "\"b\"");
    }

    #[test]
    fn first_error_finds_the_compiler_message() {
        let rust = "   Compiling hello v0.1.0\nerror[E0308]: mismatched types\n --> src/lib.rs:2:5\n  |\n2 |     42\n  |     ^^ expected `String`\n\nerror: could not compile `hello`\n";
        let (lines, more) = first_error(rust);
        assert_eq!(lines[0], "error[E0308]: mismatched types");
        assert_eq!(lines.len(), 5);
        assert_eq!(more, 2);
        let go = "# learn/hello\nhello/hello.go:5:2: undefined: x\nhello/hello.go:6:1: missing return\nFAIL\tlearn/hello [build failed]\n";
        assert_eq!(first_error(go).0[0], "hello/hello.go:5:2: undefined: x");
        let node = "TAP version 13\nfile:///w/hello/hello.ts:2\n  return 'x\n         ^\n\nSyntaxError: Invalid or unexpected token\n    at foo\n# tests 0\n";
        let (lines, _) = first_error(node);
        assert!(lines.iter().any(|l| l.starts_with("SyntaxError")), "{lines:?}");
        assert!(!lines.iter().any(|l| l.contains("TAP version")), "{lines:?}");
    }

    // ---- snapshots: every screen as plain text (`--color never`) --------------

    /// The repo's course with an empty workspace under target/, so paths are short and stable.
    fn ctx() -> Ctx {
        let ws = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target").join("view-snapshots");
        Ctx::discover(Some(ws)).unwrap()
    }

    fn go() -> &'static dyn Backend {
        backends::get("go").unwrap()
    }

    fn result(cases: &[(&str, Option<bool>, Option<&str>)], output: &str, timed_out: bool) -> CheckResult {
        let cases: Vec<CaseResult> = cases
            .iter()
            .enumerate()
            .map(|(i, (d, p, m))| CaseResult { index: i, description: d.to_string(), passed: *p, message: m.map(String::from) })
            .collect();
        let passed = !timed_out && cases.iter().all(|c| c.passed == Some(true));
        CheckResult { cases, passed, timed_out, command: "go test".into(), output: output.into(), setup_problem: None, timeout_secs: 300 }
    }

    fn here() -> &'static std::path::Path {
        std::path::Path::new(".")
    }

    #[test]
    fn snapshot_show() {
        let ctx = ctx();
        let card = task_card(&ctx, go(), ctx.spec("hello").unwrap(), &Ui::plain(80));
        let expected = [
            "  Foundations · 1 of 6                                                        Go",
            "  Hello · functions",
            "",
            "  Return a greeting for name, trimmed of surrounding whitespace, falling back to",
            "  \"world\" when nothing is left.",
            "",
            "  Edit   target/view-snapshots/go/hello/hello.go",
            "         func Greet(name string) string",
            "",
            "  learn lesson   read about functions",
            "  learn hint     a nudge if you're stuck (3 hints)",
            "  learn check    run the tests · learn watch re-runs on every save",
        ];
        assert_eq!(card, expected.join("\n"));
    }

    #[test]
    fn snapshot_check_failing() {
        let ctx = ctx();
        let msg = Some("got \"Hello, !\", want \"Hello, world!\"");
        let r = result(
            &[
                ("plain name", Some(true), None),
                ("name with surrounding spaces", Some(true), None),
                ("empty name", Some(false), msg),
                ("whitespace only", Some(false), msg),
            ],
            "",
            false,
        );
        let expected = [
            "  hello · 2/4 passing  ██████████░░░░░░░░░░",
            "",
            "  ✓ plain name",
            "  ✓ name with surrounding spaces",
            "  ✗ empty name",
            "  ✗ whitespace only",
            "",
            "  ✗ empty name",
            "    call      greet(\"\")",
            "    expected  \"Hello, world!\"",
            "    got       \"Hello, !\"",
            "",
            "  1 more failing · learn check -v for all details · stuck? learn hint",
        ];
        assert_eq!(check_report(ctx.spec("hello").unwrap(), &r, false, here(), &Ui::plain(80)), expected.join("\n"));
    }

    #[test]
    fn snapshot_check_compile_error() {
        let ctx = ctx();
        let out = "   Compiling hello v0.1.0\nerror[E0308]: mismatched types\n --> hello/src/lib.rs:2:5\n  |\n2 |     42\n  |     ^^ expected `String`, found integer\n\nerror: could not compile `hello`\n";
        let r = result(&[("plain name", None, None), ("empty name", None, None)], out, false);
        let expected = [
            "  hello · no tests ran",
            "",
            "  error[E0308]: mismatched types",
            "   --> hello/src/lib.rs:2:5",
            "    |",
            "  2 |     42",
            "    |     ^^ expected `String`, found integer",
            "",
            "  2 more lines · learn check -v for the full output",
        ];
        assert_eq!(check_report(ctx.spec("hello").unwrap(), &r, false, here(), &Ui::plain(80)), expected.join("\n"));
    }

    #[test]
    fn snapshot_check_timeout_and_pass() {
        let ctx = ctx();
        let spec = ctx.spec("hello").unwrap();
        let timeout = result(&[("plain name", Some(true), None), ("empty name", None, None)], "", true);
        let expected = ["  hello · stopped after 300 s: look for an infinite loop", "", "  ✓ plain name", "", "  learn check -v for all details"];
        assert_eq!(check_report(spec, &timeout, false, here(), &Ui::plain(80)), expected.join("\n"));
        let pass = result(&[("plain name", Some(true), None), ("empty name", Some(true), None)], "", false);
        assert_eq!(check_report(spec, &pass, false, here(), &Ui::plain(80)), "  ✓ hello · all 2 passing");
    }

    #[test]
    fn snapshot_hint() {
        let ctx = ctx();
        let out = hints(ctx.spec("leap-year").unwrap(), 2, false, &Ui::plain(80));
        let lines: Vec<&str> = out.lines().collect();
        assert!(lines[0].starts_with("  1  ") && lines.iter().any(|l| l.starts_with("  2  ")), "{out}");
        assert_eq!(lines.last().copied(), Some("  Hint 2 of 3 · learn hint for the next"));
    }

    #[test]
    fn snapshot_status() {
        let ctx = ctx();
        let out = status_track(&ctx, go(), &Ui::plain(80));
        let head: Vec<&str> = out.lines().take(5).collect();
        assert_eq!(
            head,
            ["  Go · 0 of 31 done", "", "  Syntax Primer          ░░░░░░░░░░░░  0/7 read", "    ○ A File at a Glance", "    ○ Variables and Constants"]
        );
        assert!(out.contains("  Foundations            ░░░░░░░░░░░░  0/6\n    ○ Hello\n    ○ Leap Year"), "{out}");
        assert!(out.contains("  Deeper Go              ░░░░░░░░░░░░  0/5"), "{out}");
        assert!(!out.contains("Deeper Rust"), "other tracks' modules aren't listed: {out}");
        let all = status_all(&ctx, &Ui::plain(80));
        assert!(all.lines().next().unwrap().ends_with("ts    py    rust  go    php   lua   qml"), "{all}");
        assert!(all.contains("  Deeper Rust  6 exercises · not started"), "{all}");
        assert!(all.contains("  Syntax Primer  0 of 7 read"), "{all}");
        // A module with a `tracks` list leaves the other tracks' cells empty.
        let read_file = all.lines().find(|l| l.trim_start().starts_with("Read File")).unwrap();
        assert!(read_file.ends_with("○     ○     ○     ○     ○     ○"), "QML has no read-file: {all}");
        assert!(!status_track(&ctx, crate::backends::get("qml").unwrap(), &Ui::plain(80)).contains("Files and Resources"));
    }

    #[test]
    fn snapshot_ascii() {
        let ctx = ctx();
        let ui = Ui { unicode: false, ..Ui::plain(80) };
        let out = status_track(&ctx, go(), &ui);
        assert!(out.contains("    - Hello") && out.contains("Foundations            ............  0/6"), "{out}");
        assert!(out.is_ascii(), "{out}");
    }

    #[test]
    fn colour_output_uses_the_named_styles() {
        let ctx = ctx();
        let ui = Ui { color: true, ..Ui::plain(80) };
        let card = task_card(&ctx, go(), ctx.spec("hello").unwrap(), &ui);
        assert!(card.contains("\x1b[1mHello\x1b[0m"), "title is bold: {card:?}");
        assert!(card.contains("\x1b[36mfunctions\x1b[0m"), "concept is accent: {card:?}");
        assert!(card.contains("\x1b[35mfunc Greet(name string) string\x1b[0m"), "signature is code: {card:?}");
        let msg = Some("got \"Hello, !\", want \"Hello, world!\"");
        let r = result(&[("plain name", Some(true), None), ("empty name", Some(false), msg)], "", false);
        let report = check_report(ctx.spec("hello").unwrap(), &r, false, here(), &ui);
        assert!(report.contains("\x1b[32m✓\x1b[0m plain name"), "{report:?}");
        assert!(report.contains("\x1b[31m✗\x1b[0m \x1b[31mempty name\x1b[0m"), "{report:?}");
        assert!(ui::strip_escapes(&report).contains("got       \"Hello, ‸!\""), "{report:?}");
    }

    #[test]
    fn nothing_is_wider_than_the_terminal() {
        let ctx = ctx();
        for width in [60, 80, 120] {
            let ui = Ui::plain(width.min(ui::MAX_WIDTH));
            let spec = ctx.spec("parse-port").unwrap();
            let screens = [task_card(&ctx, go(), spec, &ui), lesson(&ctx, go(), spec, &ui), status_track(&ctx, go(), &ui), status_all(&ctx, &ui)];
            for screen in screens {
                for line in screen.lines() {
                    // A URL or path is one word and can't be broken.
                    let unbreakable = line.split_whitespace().any(|w| ui::visible_width(w) > 40);
                    assert!(unbreakable || ui::visible_width(line) <= ui.width, "{width}: {line:?}");
                }
            }
        }
    }
}
