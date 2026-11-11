//! Drawing the TUI from `App`: the path, the open item (Lesson / Task /
//! Compare) and the tests, plus the overlays. Text comes from the same views and
//! renderers as the CLI, turned into ratatui text by `ansi`.

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Padding, Paragraph, Wrap};
use ratatui::Frame;

use super::ansi::to_text;
use super::app::{App, Open, Overlay, Tab, Tests};
use crate::backends::{self, Backend};
use crate::ctx::GOALS;
use crate::markdown;
use crate::notes::{self, BACKGROUNDS};
use crate::ui::{self, Ui, MARGIN};
use crate::view;
use crate::views;

/// Wide enough for the path, the open item and the tests side by side.
pub const WIDE: u16 = 110;
/// Below this the tests go under the open item instead of beside it.
pub const NARROW: u16 = 76;
const PATH_WIDTH: u16 = 32;
const TESTS_WIDTH: u16 = 40;
/// A compare column needs about this many columns to read well.
const COMPARE_COLUMN: u16 = 46;

/// What the middle pane shows, rendered once per change rather than per frame.
pub enum Content {
    Text(Text<'static>),
    /// Compare: a header (the why-notes), then one column per track.
    Columns { header: Text<'static>, columns: Vec<(String, Text<'static>)> },
}

#[derive(Default)]
pub struct Cache {
    key: Option<(u64, Option<Open>, Tab, u16)>,
    content: Option<Content>,
}

/// A `Ui` for rendering into a pane `width` columns wide.
fn pane_ui(width: u16) -> Ui {
    let base = ui::get();
    Ui { color: base.color, unicode: base.unicode, ..Ui::plain((width as usize).clamp(20, 100)) }
}

pub fn draw(f: &mut Frame, app: &App, cache: &mut Cache) {
    let [top, body, bottom] = Layout::vertical([Constraint::Length(1), Constraint::Min(3), Constraint::Length(1)]).areas(f.area());
    draw_top(f, app, top);
    draw_bottom(f, app, bottom);

    let w = body.width;
    if w >= WIDE {
        let [path, main, tests] =
            Layout::horizontal([Constraint::Length(PATH_WIDTH), Constraint::Min(30), Constraint::Length(TESTS_WIDTH)]).areas(body);
        draw_path(f, app, path);
        draw_main(f, app, cache, main);
        draw_tests(f, app, tests);
    } else if w >= NARROW {
        let [main, tests] = Layout::horizontal([Constraint::Min(30), Constraint::Length(TESTS_WIDTH - 6)]).areas(body);
        draw_main(f, app, cache, main);
        draw_tests(f, app, tests);
        if app.show_path {
            let area = Rect { width: PATH_WIDTH.min(main.width), ..main };
            f.render_widget(Clear, area);
            draw_path(f, app, area);
        }
    } else {
        let [main, tests] = Layout::vertical([Constraint::Min(6), Constraint::Percentage(35)]).areas(body);
        draw_main(f, app, cache, main);
        draw_tests(f, app, tests);
        if app.show_path {
            f.render_widget(Clear, body);
            draw_path(f, app, body);
        }
    }

    if let Some(o) = &app.overlay {
        draw_overlay(f, app, o, f.area());
    }
}

fn draw_top(f: &mut Frame, app: &App, area: Rect) {
    let mut left = vec![Span::from(" learn").bold()];
    if let Some(b) = app.track() {
        left.push(Span::from(format!("  {}", b.name())).cyan());
        let goal = GOALS.iter().find(|(id, _)| *id == app.ctx.goal()).map_or("", |(_, l)| *l);
        if area.width >= 70 {
            left.push(Span::from(format!(" {} {goal}", ui::get().dot())).dim());
        }
    }
    f.render_widget(Line::from(left), area);
    let right = Line::from(vec![Span::from("? keys  q quit ").dim()]).right_aligned();
    f.render_widget(right, area);
}

fn draw_bottom(f: &mut Frame, app: &App, area: Rect) {
    let line = match &app.message {
        Some(m) => Line::from(format!(" {m}")).yellow(),
        None => {
            let keys: &[(&str, &str)] = if app.exercise().is_some() {
                &[("e", "edit"), ("c", "check"), ("h", "hint"), ("n", "next"), ("Tab", "tabs"), ("t", "track"), ("/", "find")]
            } else {
                &[("n", "next"), ("r", "next unread"), ("j/k", "move"), ("Enter", "open"), ("t", "track"), ("/", "find")]
            };
            let mut spans = vec![Span::from(" ")];
            for (k, what) in keys {
                spans.push(Span::from(*k).cyan());
                spans.push(Span::from(format!(" {what}  ")).dim());
            }
            if area.width < WIDE {
                spans.push(Span::from("p").cyan());
                spans.push(Span::from(" path").dim());
            }
            Line::from(spans)
        }
    };
    f.render_widget(line, area);
}

fn block(title: &str) -> Block<'_> {
    Block::default().borders(Borders::ALL).border_style(Style::new().dim()).title(Span::from(format!(" {title} ")).bold())
}

// ---- the path ----------------------------------------------------------------------

fn draw_path(f: &mut Frame, app: &App, area: Rect) {
    let Some(path) = &app.path else {
        f.render_widget(block("Path"), area);
        return;
    };
    let u = ui::get();
    let (done, todo, current, reading) = if u.unicode { ("✓", "○", "●", "·") } else { ("ok", "-", ">", ".") };
    let open = app.open.clone();
    let mut rows: Vec<ListItem> = Vec::new();
    let mut selected = None;
    let mut index = 0;
    for stage in &path.stages {
        let finished = stage.items.iter().filter(|i| i.done).count();
        rows.push(ListItem::new(Line::from(vec![
            Span::from(stage.title.clone()).bold(),
            Span::from(format!(" {finished}/{}", stage.items.len())).dim(),
        ])));
        for item in &stage.items {
            let is_current = item.kind == "exercise" && path.current.as_deref() == Some(item.id.as_str());
            let mark = match (item.kind, item.done) {
                (_, true) => Span::from(done).green(),
                ("exercise", false) if is_current => Span::from(current).cyan(),
                ("reading", false) => Span::from(reading).dim(),
                _ => Span::from(todo).dim(),
            };
            let this = if item.kind == "reading" { Open::Reading(item.id.clone()) } else { Open::Exercise(item.id.clone()) };
            let mut title = Span::from(item.title.clone());
            if open.as_ref() == Some(&this) {
                title = title.bold().cyan();
            } else if item.done {
                title = title.dim();
            }
            if index == app.cursor {
                selected = Some(rows.len());
            }
            rows.push(ListItem::new(Line::from(vec![Span::from(" "), mark, Span::from(" "), title])));
            index += 1;
        }
    }
    let title = format!("{} {}/{}", path.track_name, path.done, path.total);
    let list = List::new(rows).block(block(&title)).highlight_style(Style::new().add_modifier(Modifier::REVERSED));
    let mut state = ListState::default().with_selected(selected);
    f.render_stateful_widget(list, area, &mut state);
}

// ---- the open item -----------------------------------------------------------------

fn draw_main(f: &mut Frame, app: &App, cache: &mut Cache, area: Rect) {
    let inner_width = area.width.saturating_sub(2);
    let key = (app.version, app.open.clone(), app.tab, inner_width);
    if cache.key.as_ref() != Some(&key) {
        cache.content = Some(content(app, inner_width));
        cache.key = Some(key);
    }
    let title = match &app.open {
        Some(Open::Exercise(slug)) => app.ctx.spec(slug).map(|s| s.title.clone()).unwrap_or_else(|_| slug.clone()),
        Some(Open::Reading(id)) => app.ctx.reading(id).map(|r| r.title.clone()).unwrap_or_else(|| id.clone()),
        None => "learn".into(),
    };
    let outer = block(&title);
    let inner = outer.inner(area);
    f.render_widget(outer, area);

    let [tabs, body] = Layout::vertical([Constraint::Length(if app.exercise().is_some() { 2 } else { 0 }), Constraint::Min(1)]).areas(inner);
    if app.exercise().is_some() {
        let mut spans = vec![Span::from(" ")];
        for (i, t) in app.tabs().into_iter().enumerate() {
            let label = format!(" {} {} ", i + 1, tab_name(t));
            spans.push(if t == app.tab { Span::from(label).reversed().bold() } else { Span::from(label).dim() });
            spans.push(Span::from(" "));
        }
        f.render_widget(Line::from(spans), tabs);
    }

    match cache.content.as_ref() {
        Some(Content::Text(t)) => scrolled(f, t, app.scroll, body),
        Some(Content::Columns { header, columns }) => draw_columns(f, app, header, columns, body),
        None => {}
    }
}

fn tab_name(t: Tab) -> &'static str {
    match t {
        Tab::Lesson => "Lesson",
        Tab::Task => "Task",
        Tab::Compare => "Compare",
    }
}

fn scrolled(f: &mut Frame, text: &Text<'static>, scroll: u16, area: Rect) {
    let max = (text.lines.len() as u16).saturating_sub(area.height / 2);
    f.render_widget(Paragraph::new(text.clone()).wrap(Wrap { trim: false }).scroll((scroll.min(max), 0)), area);
}

fn draw_columns(f: &mut Frame, app: &App, header: &Text<'static>, columns: &[(String, Text<'static>)], area: Rect) {
    let head = header.lines.len() as u16;
    let [top, rest] = Layout::vertical([Constraint::Length(head), Constraint::Min(1)]).areas(area);
    f.render_widget(Paragraph::new(header.clone()), top);
    if columns.is_empty() {
        return;
    }
    let fit = (rest.width / COMPARE_COLUMN).max(1) as usize;
    if columns.len() <= fit {
        let areas = Layout::horizontal(vec![Constraint::Ratio(1, columns.len() as u32); columns.len()]).split(rest);
        for ((name, text), a) in columns.iter().zip(areas.iter()) {
            column(f, name, text, app.scroll, *a);
        }
    } else {
        let i = app.compare_col.min(columns.len() - 1);
        let (name, text) = &columns[i];
        column(f, &format!("{name}  ({} of {}, ←/→)", i + 1, columns.len()), text, app.scroll, rest);
    }
}

fn column(f: &mut Frame, name: &str, text: &Text<'static>, scroll: u16, area: Rect) {
    let [head, body] = Layout::vertical([Constraint::Length(1), Constraint::Min(1)]).areas(area);
    f.render_widget(Line::from(format!(" {name}")).bold().cyan(), head);
    scrolled(f, text, scroll, body);
}

/// The middle pane's text for what's open.
fn content(app: &App, width: u16) -> Content {
    let Some(b) = app.track() else { return Content::Text(Text::default()) };
    match &app.open {
        None => Content::Text(Text::from("")),
        Some(Open::Reading(id)) => Content::Text(to_text(&reading(app, b, id, &pane_ui(width)))),
        Some(Open::Exercise(slug)) => {
            let Ok(spec) = app.ctx.spec_on(b.id(), slug) else { return Content::Text(Text::from(format!("unknown exercise {slug}"))) };
            match app.tab {
                Tab::Lesson => Content::Text(to_text(&view::lesson(&app.ctx, b, spec, &pane_ui(width)))),
                Tab::Task => Content::Text(to_text(&task(app, b, spec, &pane_ui(width)))),
                Tab::Compare => compare(app, b, &spec.concept, width),
            }
        }
    }
}

fn reading(app: &App, b: &dyn Backend, id: &str, ui: &Ui) -> String {
    let Some(r) = app.ctx.reading(id) else { return format!("unknown reading {id}") };
    let list = app.ctx.readings_for(Some(b.id()));
    let i = list.iter().position(|x| x.id == r.id).map_or(0, |i| i + 1);
    let module = &app.ctx.course.modules[r.module].title;
    let rule = ui.muted(&(if ui.unicode { "─" } else { "-" }).repeat(ui.width.saturating_sub(MARGIN.len() * 2)));
    let mut out = vec![format!("{MARGIN}{}", ui.muted(&format!("{module} {} page {i} of {}", ui.dot(), list.len())))];
    out.extend(notes::read_concept(&app.ctx.root, &r.id, app.ctx.note_style()).map(|md| markdown::render(&md, ui)));
    match notes::read_track_note(&app.ctx.root, b.id(), &r.id, app.ctx.note_style()) {
        Some(md) => out.push(format!("{MARGIN}{rule}\n\n{}", markdown::render(&md, ui))),
        None => out.push(format!("{MARGIN}{}", ui.muted(&format!("No {} note for this page yet.", b.name())))),
    }
    out.push(format!("{MARGIN}{}", ui.muted("n for the next item on your path")));
    out.join("\n\n")
}

/// The Task tab: what to do, which file, and what the tests check.
fn task(app: &App, b: &dyn Backend, spec: &crate::spec::Spec, ui: &Ui) -> String {
    let mut out = Vec::new();
    let done = if app.ctx.is_done(b.id(), &spec.slug) { format!("  {} {}", ui.pass_mark(), ui.success("done")) } else { String::new() };
    out.push(format!("{MARGIN}{}{done}", ui.accent(&spec.concept)));
    out.extend(ui.wrap(&spec.main.summary, MARGIN, MARGIN));
    out.push(String::new());
    let dir = app.ctx.workspace.join(b.id());
    let mut label = "Edit";
    for (file, sigs) in backends::learner_files(b, spec) {
        out.push(format!("{MARGIN}{} {}", ui.heading(&format!("{label:<6}")), ui.path(&dir.join(&file))));
        for sig in sigs {
            out.push(format!("{MARGIN}       {}", ui.code(&sig)));
        }
        label = "";
    }
    out.push(String::new());
    out.push(format!("{MARGIN}{}", ui.heading("What the tests check")));
    for c in views::cases(spec) {
        out.extend(ui.wrap(&c.description, &format!("{MARGIN}{} ", ui.todo_mark()), &format!("{MARGIN}  ")));
        for l in c.lines {
            out.push(format!("{MARGIN}    {}", ui.code(&l)));
        }
    }
    out.push(String::new());
    let tip = format!("e opens the file in your editor {d} saving runs the tests {d} c runs them now", d = ui.dot());
    out.extend(ui.wrap(&tip, MARGIN, MARGIN).iter().map(|l| ui.muted(l)));
    out.join("\n")
}

fn compare(app: &App, b: &dyn Backend, concept: &str, width: u16) -> Content {
    let wanted: Vec<String> = if app.compare_all {
        Vec::new()
    } else {
        views::default_compare_tracks(&app.ctx, concept, Some(b.id())).iter().map(|t| t.id().to_string()).collect()
    };
    let view = match views::compare(&app.ctx, concept, &wanted) {
        Ok(v) => v,
        Err(e) => return Content::Text(Text::from(e.0)),
    };
    let ui = pane_ui(width);
    let mut header = Vec::new();
    for w in &view.why {
        header.extend(ui.wrap(&w.text, MARGIN, MARGIN).iter().map(|l| ui.muted(l)));
    }
    let more = if app.compare_all { "a: back to your family" } else { "a: every track" };
    header.push(format!("{MARGIN}{}", ui.muted(more)));
    header.push(String::new());
    let fit = (width / COMPARE_COLUMN).max(1) as usize;
    let col_width = if view.columns.len() <= fit { width / view.columns.len().max(1) as u16 } else { width };
    let col_ui = pane_ui(col_width.saturating_sub(1));
    let columns = view
        .columns
        .iter()
        .map(|c| {
            let text = match &c.note {
                Some(md) => markdown::render(md, &col_ui),
                None => format!("{MARGIN}{}", col_ui.muted(&format!("No {} note yet.", c.name))),
            };
            (c.name.to_string(), to_text(&text))
        })
        .collect();
    Content::Columns { header: to_text(&header.join("\n")), columns }
}

// ---- the tests ---------------------------------------------------------------------

fn draw_tests(f: &mut Frame, app: &App, area: Rect) {
    let outer = block("Tests");
    let inner = outer.inner(area);
    f.render_widget(outer, area);
    let ui = pane_ui(inner.width);
    let text = match (app.track(), app.exercise()) {
        (Some(b), Some(slug)) => tests_text(app, b, slug, &ui),
        _ => format!("{MARGIN}{}", ui.muted("Open an exercise to run its tests.")),
    };
    f.render_widget(Paragraph::new(to_text(&text)).wrap(Wrap { trim: false }), inner);
}

fn tests_text(app: &App, b: &dyn Backend, slug: &str, ui: &Ui) -> String {
    let Ok(spec) = app.ctx.spec_on(b.id(), slug) else { return String::new() };
    let mut out = Vec::new();
    match app.tests.as_ref().filter(|t| t.slug == slug && t.track == b.id()) {
        None if b.tool().find().is_none() => {
            let msg = format!("{} isn't installed, so the tests can't run. learn doctor shows how to install it.", b.tool().label);
            out.extend(ui.wrap(&msg, MARGIN, MARGIN).iter().map(|l| ui.warning(l)));
        }
        None => {
            let msg = format!("Not run yet {} c runs them, and so does saving the file.", ui.dot());
            out.extend(ui.wrap(&msg, MARGIN, MARGIN).iter().map(|l| ui.muted(l)));
        }
        Some(t) => out.extend(run_lines(t, spec, &app.ctx.workspace.join(b.id()), ui)),
    }

    let shown = app.ctx.progress.hints.get(slug).copied().unwrap_or(0).min(spec.hints.len());
    if shown > 0 {
        out.push(String::new());
        out.push(format!("{MARGIN}{}", ui.heading("Hints")));
        for (i, h) in spec.hints.iter().take(shown).enumerate() {
            let n = format!("{}. ", i + 1);
            let pad = " ".repeat(MARGIN.len() + n.len());
            let lines = ui.wrap(h, &format!("{MARGIN}{n}"), &pad);
            out.extend(lines.iter().map(|l| if i + 1 == shown { l.clone() } else { ui.muted(l) }));
        }
        let left = spec.hints.len() - shown;
        if left > 0 {
            out.push(format!("{MARGIN}{}", ui.muted(&format!("h for the next ({left} left)"))));
        }
    }
    out.join("\n")
}

fn run_lines(t: &Tests, spec: &crate::spec::Spec, track_dir: &std::path::Path, ui: &Ui) -> Vec<String> {
    let mut out = Vec::new();
    let dot = ui.dot();
    if let Some(e) = &t.error {
        out.extend(ui.wrap(e, MARGIN, MARGIN).iter().map(|l| ui.failure(l)));
        return out;
    }
    let Some(r) = &t.result else {
        let ran = t.live.iter().filter(|c| c.2.is_some()).count();
        out.push(format!("{MARGIN}{}", ui.accent(&format!("Running {dot} {ran}/{}", t.live.len()))));
        out.push(String::new());
        for (_, desc, passed) in &t.live {
            let mark = match passed {
                Some(true) => ui.pass_mark(),
                Some(false) => ui.fail_mark(),
                None => ui.todo_mark(),
            };
            out.extend(ui.wrap(desc, &format!("{MARGIN}{mark} "), &format!("{MARGIN}  ")));
        }
        return out;
    };
    let total = r.cases.len();
    let passing = r.count(Some(true));
    if r.passed {
        out.push(format!("{MARGIN}{} {}", ui.pass_mark(), ui.success(&format!("all {total} passing"))));
        return out;
    }
    if r.timed_out {
        out.push(format!("{MARGIN}{}", ui.warning(&format!("stopped after {} s: look for an infinite loop", r.timeout_secs))));
    } else if r.none_ran() {
        out.push(format!("{MARGIN}{}", ui.failure("no tests ran")));
    } else {
        out.push(format!("{MARGIN}{passing}/{total} passing  {}", ui.bar(passing, total, 12)));
    }
    out.push(String::new());
    if r.none_ran() && !r.timed_out {
        match &r.setup_problem {
            Some(p) => out.extend(ui.wrap(p, MARGIN, MARGIN)),
            None => {
                let (lines, _) = view::first_error(&r.output);
                out.extend(lines.iter().map(|l| format!("{MARGIN}{}", view::shorten_paths(l, track_dir))));
            }
        }
        return out;
    }
    out.extend(r.cases.iter().filter_map(|c| view::case_line(c, ui)));
    if let Some(c) = r.cases.iter().find(|c| c.passed == Some(false)) {
        out.push(String::new());
        out.extend(view::failure_detail(spec, c, false, ui));
    }
    out
}

// ---- overlays ----------------------------------------------------------------------

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let w = width.min(area.width.saturating_sub(2));
    let h = height.min(area.height.saturating_sub(2));
    Rect { x: area.x + (area.width - w) / 2, y: area.y + (area.height - h) / 2, width: w, height: h }
}

fn picker(f: &mut Frame, area: Rect, title: &str, intro: &str, items: Vec<Line<'static>>, cursor: usize, footer: &str) {
    let height = items.len() as u16 + 6;
    let box_area = centered(area, 64, height);
    f.render_widget(Clear, box_area);
    let outer = block(title).border_style(Style::new().cyan());
    let inner = outer.inner(box_area);
    f.render_widget(outer, box_area);
    let [intro_area, list_area, foot_area] =
        Layout::vertical([Constraint::Length(2), Constraint::Min(1), Constraint::Length(1)]).areas(inner);
    f.render_widget(Paragraph::new(Line::from(intro.to_string()).dim()).wrap(Wrap { trim: true }).block(Block::default().padding(Padding::horizontal(1))), intro_area);
    let list = List::new(items).highlight_style(Style::new().reversed()).highlight_symbol(" > ");
    f.render_stateful_widget(list, list_area, &mut ListState::default().with_selected(Some(cursor)));
    f.render_widget(Line::from(format!(" {footer}")).dim(), foot_area);
}

fn draw_overlay(f: &mut Frame, app: &App, overlay: &Overlay, area: Rect) {
    let u = ui::get();
    match overlay {
        Overlay::Help => {
            let keys = [
                ("j/k, ↑/↓", "move along the path"),
                ("Enter", "open the item"),
                ("n", "next item on the path"),
                ("r", "next unread page"),
                ("Tab, 1 2 3", "Lesson / Task / Compare"),
                ("PgUp/PgDn, Space", "scroll"),
                ("e", "edit the file in $EDITOR (or Zed)"),
                ("c", "run the tests (saving does too)"),
                ("h", "next hint"),
                ("←/→, a", "compare: next track, every track"),
                ("t", "switch track"),
                ("/", "find an exercise or page"),
                ("p", "show the path (narrow terminals)"),
                ("q, Ctrl+C", "quit"),
            ];
            let lines: Vec<Line> = keys
                .iter()
                .map(|(k, what)| Line::from(vec![Span::from(format!(" {k:<18}")).cyan(), Span::from(*what)]))
                .collect();
            let box_area = centered(area, 60, lines.len() as u16 + 4);
            f.render_widget(Clear, box_area);
            let outer = block("Keys").border_style(Style::new().cyan());
            let inner = outer.inner(box_area);
            f.render_widget(outer, box_area);
            let mut text = Text::from(lines);
            text.push_line(Line::from(""));
            text.push_line(Line::from(" any key closes this").dim());
            f.render_widget(Paragraph::new(text), inner);
        }
        Overlay::Tracks { cursor, first_run } => {
            let items = backends::all()
                .iter()
                .map(|b| {
                    let t = views::track(&app.ctx, *b);
                    let total = app.ctx.path_specs(b.id()).len();
                    let mut spans = vec![Span::from(format!("{:<11}", b.name())).bold()];
                    spans.push(Span::from(format!("{:<10}", t.family.unwrap_or_default())).dim());
                    spans.push(Span::from(format!("{:>3}/{:<3}", t.done, total)));
                    if !t.tool.found {
                        spans.push(Span::from(format!(" {} not installed", t.tool.label)).yellow());
                    }
                    Line::from(spans)
                })
                .collect();
            let (title, intro) = if *first_run {
                ("Pick a language to learn", "Each track has its own path. You can switch later with t; your work is kept.")
            } else {
                ("Switch track", "Your work on every track is kept.")
            };
            let esc = if *first_run { "Enter picks · Esc quits" } else { "Enter picks · Esc closes" };
            picker(f, area, title, intro, items, *cursor, esc);
        }
        Overlay::Background { cursor, .. } => {
            let items = BACKGROUNDS.iter().map(|(_, l)| Line::from(l.to_string())).collect();
            picker(f, area, "A language you already know", "Notes add a \"coming from\" section for it.", items, *cursor, "Enter picks");
        }
        Overlay::Goal { cursor } => {
            let items = GOALS.iter().map(|(_, l)| Line::from(l.to_string())).collect();
            picker(f, area, "What you're here for", "Compare views start with the tracks you pick.", items, *cursor, "Enter picks");
        }
        Overlay::CompareTracks { cursor, chosen } => {
            let (on, off) = if u.unicode { ("[✓]", "[ ]") } else { ("[x]", "[ ]") };
            let items = backends::all()
                .iter()
                .map(|b| Line::from(format!("{} {}", if chosen.contains(&b.id()) { on } else { off }, b.name())))
                .collect();
            picker(f, area, "Tracks to compare", "Two or more.", items, *cursor, "Space picks · Enter saves");
        }
        Overlay::Find { query, cursor } => {
            let found = app.found(query);
            let stages = app.path.as_ref().map(|p| &p.stages);
            let items: Vec<Line> = found
                .iter()
                .take(14)
                .map(|(s, i)| {
                    let stage = stages.and_then(|st| st.get(*s)).map(|st| st.title.clone()).unwrap_or_default();
                    Line::from(vec![Span::from(i.title.clone()), Span::from(format!("  {stage}")).dim()])
                })
                .collect();
            let shown = items.len();
            picker(f, area, "Find", &format!("/ {query}_"), items, (*cursor).min(shown.saturating_sub(1)), "type to filter · Enter opens · Esc closes");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::app::{App, Msg};
    use super::*;
    use crate::check::{CaseResult, CheckResult};
    use crate::ctx::Ctx;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn app(name: &str) -> App {
        let ws = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target").join(format!("tui-draw-{name}"));
        let _ = std::fs::remove_dir_all(&ws);
        let mut ctx = Ctx::discover(Some(ws.clone())).unwrap();
        ctx.config.track = Some("go".into());
        ctx.save_config().unwrap();
        App::new(Some(ws), ctx)
    }

    fn screen(app: &App, w: u16, h: u16) -> Vec<String> {
        let mut t = Terminal::new(TestBackend::new(w, h)).unwrap();
        let mut cache = Cache::default();
        t.draw(|f| draw(f, app, &mut cache)).unwrap();
        let buf = t.backend().buffer();
        (0..h).map(|y| (0..w).map(|x| buf[(x, y)].symbol().to_string()).collect::<String>().trim_end().to_string()).collect()
    }

    #[test]
    fn wide_screen_has_three_panes() {
        let a = app("wide");
        let s = screen(&a, 140, 30);
        assert!(s[0].starts_with(" learn  Go"), "{}", s[0]);
        assert!(s[1].contains(" Go 0/") && s[1].contains(" Hello ") && s[1].contains(" Tests "), "{}", s[1]);
        assert!(s.iter().any(|l| l.contains("Syntax Primer")), "the path lists stages");
        assert!(s.iter().any(|l| l.contains("● Hello")), "the current exercise is marked");
        assert!(s.iter().any(|l| l.contains("1 Lesson") && l.contains("2 Task") && l.contains("3 Compare")));
        assert!(s.iter().any(|l| l.contains("Not run yet")));
    }

    #[test]
    fn task_tab_snapshot() {
        let mut a = app("task");
        a.tab = Tab::Task;
        a.message = None;
        let s = screen(&a, 100, 26);
        let main: Vec<String> = s[1..25].iter().map(|l| l.chars().take(66).collect::<String>().trim_end().to_string()).collect();
        let expected = r#"┌ Hello ─────────────────────────────────────────────────────────┐
│  1 Lesson   2 Task   3 Compare                                 │
│                                                                │
│  functions                                                     │
│  Return a greeting for name, trimmed of surrounding whitespace,│
│  falling back to "world" when nothing is left.                 │
│                                                                │
│  Edit   target/tui-draw-task/go/01-foundations/hello/hello.go  │
│         func Greet(name string) string                         │
│                                                                │
│  What the tests check                                          │
│  ○ plain name                                                  │
│      greet("Alice") → "Hello, Alice!"                          │
│  ○ name with surrounding spaces                                │
│      greet("  Bob  ") → "Hello, Bob!"                          │
│  ○ empty name                                                  │
│      greet("") → "Hello, world!"                               │
│  ○ whitespace only                                             │
│      greet("   ") → "Hello, world!"                            │
│  ○ inner spaces are kept                                       │
│      greet("Mary Jane") → "Hello, Mary Jane!"                  │
│  ○ tabs and newlines are trimmed, non-ASCII is kept            │
│      greet("\tZoë\n") → "Hello, Zoë!"                          │
└────────────────────────────────────────────────────────────────┘"#;
        assert_eq!(main.join("\n"), expected, "\n{}", main.join("\n"));
    }

    #[test]
    fn failing_tests_show_marks_and_the_first_failure() {
        let mut a = app("failing");
        let first = a.tests.is_none();
        assert!(first);
        let effect = a.key(crossterm::event::KeyEvent::new(crossterm::event::KeyCode::Char('c'), crossterm::event::KeyModifiers::NONE));
        assert!(matches!(effect, super::super::app::Effect::Check { .. }));
        let live = a.tests.as_ref().unwrap().live.clone();
        a.on_msg(Msg::Case { index: live[0].0, passed: true });
        let s = screen(&a, 140, 30);
        assert!(s.iter().any(|l| l.contains(&format!("Running {} 1/{}", ui::get().dot(), live.len()))), "{}", s.join("\n"));
        let cases: Vec<CaseResult> = live
            .iter()
            .enumerate()
            .map(|(i, (index, d, _))| CaseResult { index: *index, description: d.clone(), passed: Some(i == 0), message: None })
            .collect();
        let result = CheckResult { cases, passed: false, timed_out: false, command: String::new(), output: String::new(), setup_problem: None, timeout_secs: 10 };
        a.on_msg(Msg::Finished(Ok(result)));
        let s = screen(&a, 140, 30);
        assert!(s.iter().any(|l| l.contains(&format!("1/{} passing", live.len()))), "{}", s.join("\n"));
        assert!(s.iter().any(|l| l.contains("expected")), "the first failure's detail");
    }

    #[test]
    fn narrow_screen_hides_the_path_until_p() {
        let mut a = app("narrow");
        let s = screen(&a, 70, 30);
        assert!(!s.iter().any(|l| l.contains(" Go 0/")), "no path pane");
        assert!(s.iter().any(|l| l.contains(" Tests ")), "tests below");
        a.key(crossterm::event::KeyEvent::new(crossterm::event::KeyCode::Char('p'), crossterm::event::KeyModifiers::NONE));
        let s = screen(&a, 70, 30);
        assert!(s.iter().any(|l| l.contains(" Go 0/")), "path shown");
    }
}
