//! The TUI's state and what each key does. Nothing here draws or touches the
//! terminal, so key handling is tested directly; `run.rs` carries out the
//! `Effect`s (running tests, opening an editor) and feeds back `Msg`s.

use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::backends::{self, Backend};
use crate::check::CheckResult;
use crate::ctx::{Ctx, GOALS};
use crate::error::Result;
use crate::notes::BACKGROUNDS;
use crate::views::{self, PathItem, PathView};

/// What the middle pane shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Open {
    Exercise(String),
    Reading(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tab {
    Lesson,
    Task,
    Compare,
}

/// A box over the panes that takes the keys until it closes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Overlay {
    Help,
    /// Pick a track. On first run, background and goal follow.
    Tracks { cursor: usize, first_run: bool },
    Background { cursor: usize, first_run: bool },
    Goal { cursor: usize },
    /// For the compare goal: the tracks to compare (two or more).
    CompareTracks { cursor: usize, chosen: Vec<&'static str> },
    Find { query: String, cursor: usize },
}

/// One exercise's latest test run, as it streams in.
#[derive(Debug)]
pub struct Tests {
    pub track: &'static str,
    pub slug: String,
    pub running: bool,
    /// Every case with its result so far: (index, description, passed).
    pub live: Vec<(usize, String, Option<bool>)>,
    pub result: Option<CheckResult>,
    pub error: Option<String>,
}

/// Work for the run loop.
#[derive(Debug, PartialEq, Eq)]
pub enum Effect {
    None,
    Quit,
    Check { track: &'static str, slug: String },
    Edit(PathBuf),
}

/// What comes back from the run loop's threads.
#[derive(Debug)]
pub enum Msg {
    Case { index: usize, passed: bool },
    Finished(std::result::Result<CheckResult, String>),
    /// A file in the open exercise's folder changed.
    Saved,
}

pub struct App {
    pub workspace: Option<PathBuf>,
    pub ctx: Ctx,
    pub path: Option<PathView>,
    /// Index into `items()`.
    pub cursor: usize,
    pub open: Option<Open>,
    pub tab: Tab,
    pub scroll: u16,
    /// Compare every track, not just your family (or your compare goal's tracks).
    pub compare_all: bool,
    /// The compare column shown when only one fits.
    pub compare_col: usize,
    pub tests: Option<Tests>,
    pub overlay: Option<Overlay>,
    /// The path pane on narrow terminals, where it's hidden by default.
    pub show_path: bool,
    /// One line at the bottom: what just happened.
    pub message: Option<String>,
    /// Bumped whenever what the middle pane shows may have changed.
    pub version: u64,
}

impl App {
    pub fn new(workspace: Option<PathBuf>, ctx: Ctx) -> App {
        let mut app = App {
            workspace,
            ctx,
            path: None,
            cursor: 0,
            open: None,
            tab: Tab::Lesson,
            scroll: 0,
            compare_all: false,
            compare_col: 0,
            tests: None,
            overlay: None,
            show_path: false,
            message: None,
            version: 0,
        };
        if app.track().is_none() {
            app.overlay = Some(Overlay::Tracks { cursor: 0, first_run: true });
        } else {
            app.refresh_path();
            app.open_current();
        }
        app
    }

    pub fn track(&self) -> Option<&'static dyn Backend> {
        self.ctx.config.track.as_deref().and_then(|t| backends::get(t).ok())
    }

    /// Every reading and exercise on the path, with its stage's index.
    pub fn items(&self) -> Vec<(usize, &PathItem)> {
        self.path
            .iter()
            .flat_map(|p| p.stages.iter().enumerate())
            .flat_map(|(i, s)| s.items.iter().map(move |item| (i, item)))
            .collect()
    }

    fn item_open(item: &PathItem) -> Open {
        if item.kind == "reading" { Open::Reading(item.id.clone()) } else { Open::Exercise(item.id.clone()) }
    }

    fn open_index(&self) -> Option<usize> {
        let open = self.open.as_ref()?;
        self.items().iter().position(|(_, i)| Self::item_open(i) == *open)
    }

    /// Re-read config and progress from disk (after a check, a hint, a new track).
    pub fn reload(&mut self) {
        if let Ok(ctx) = Ctx::discover(self.workspace.clone()) {
            self.ctx = ctx;
        }
        self.refresh_path();
    }

    fn refresh_path(&mut self) {
        self.path = self.track().map(|b| views::path(&self.ctx, b));
        self.version += 1;
    }

    /// The first unfinished exercise, or the first item when the track is done.
    fn open_current(&mut self) {
        let target = match self.path.as_ref().and_then(|p| p.current.clone()) {
            Some(slug) => Some(Open::Exercise(slug)),
            None => self.items().first().map(|(_, i)| Self::item_open(i)),
        };
        if let Some(t) = target {
            self.open(t);
        }
        let new = self.items().iter().all(|(_, i)| !i.done);
        if new && self.items().iter().any(|(_, i)| i.kind == "reading") {
            self.message = Some("New to this? r opens the Syntax Primer first: short pages, optional.".into());
        }
    }

    pub fn open(&mut self, target: Open) {
        let Some(b) = self.track() else { return };
        match &target {
            Open::Exercise(slug) => {
                // Like `learn serve`: make sure the files exist before showing them.
                if let Ok(spec) = self.ctx.spec_on(b.id(), slug) {
                    let dir = self.ctx.workspace.join(b.id());
                    if !dir.join(backends::main_file(b, spec)).exists() {
                        if let Err(e) = backends::generate(b, &self.ctx.track_specs(b.id()), &dir) {
                            self.message = Some(e.0);
                        }
                    }
                }
                if self.tests.as_ref().is_some_and(|t| t.slug != *slug || t.track != b.id()) {
                    self.tests = None;
                }
            }
            Open::Reading(id) => {
                self.tab = Tab::Lesson;
                if !self.ctx.is_read(id) {
                    let _ = self.ctx.mark_read(id);
                    self.refresh_path();
                }
            }
        }
        self.open = Some(target);
        self.scroll = 0;
        self.compare_col = 0;
        if let Some(i) = self.open_index() {
            self.cursor = i;
        }
        self.version += 1;
    }

    /// The open exercise's slug.
    pub fn exercise(&self) -> Option<&str> {
        match &self.open {
            Some(Open::Exercise(s)) => Some(s),
            _ => None,
        }
    }

    /// The folder to watch for saves: the open exercise's.
    pub fn watched_dir(&self) -> Option<PathBuf> {
        let b = self.track()?;
        Some(self.ctx.workspace.join(b.id()).join(self.exercise()?))
    }

    fn check(&mut self) -> Effect {
        let (Some(b), Some(slug)) = (self.track(), self.exercise().map(String::from)) else {
            self.message = Some("Open an exercise to run its tests.".into());
            return Effect::None;
        };
        if self.tests.as_ref().is_some_and(|t| t.running) {
            return Effect::None;
        }
        let live = self
            .ctx
            .spec_on(b.id(), &slug)
            .map(|s| views::cases(s).into_iter().map(|c| (c.index, c.description, None)).collect())
            .unwrap_or_default();
        self.tests = Some(Tests { track: b.id(), slug: slug.clone(), running: true, live, result: None, error: None });
        Effect::Check { track: b.id(), slug }
    }

    pub fn on_msg(&mut self, msg: Msg) -> Effect {
        match msg {
            Msg::Case { index, passed } => {
                if let Some(t) = &mut self.tests {
                    if let Some(c) = t.live.iter_mut().find(|c| c.0 == index) {
                        c.2 = Some(passed);
                    }
                }
            }
            Msg::Finished(outcome) => {
                let passed = matches!(&outcome, Ok(r) if r.passed);
                if let Some(t) = &mut self.tests {
                    t.running = false;
                    match outcome {
                        Ok(r) => t.result = Some(r),
                        Err(e) => t.error = Some(e),
                    }
                }
                if passed {
                    self.reload();
                    self.message = Some("All passing. n opens the next one.".into());
                }
            }
            Msg::Saved => {
                if self.exercise().is_some() {
                    return self.check();
                }
            }
        }
        Effect::None
    }

    fn hint(&mut self) {
        let (Some(b), Some(slug)) = (self.track(), self.exercise().map(String::from)) else { return };
        let Ok(total) = self.ctx.spec_on(b.id(), &slug).map(|s| s.hints.len()) else { return };
        if total == 0 {
            self.message = Some("This exercise has no hints. The Lesson tab has a worked example.".into());
            return;
        }
        let shown = self.ctx.progress.hints.entry(slug).or_insert(0);
        let was_all = *shown >= total;
        *shown = (*shown + 1).min(total);
        let shown = *shown;
        self.message = Some(match self.ctx.save_progress() {
            Err(e) => e.0,
            Ok(()) if was_all => format!("That's every hint ({total})."),
            Ok(()) => format!("Hint {shown} of {total}, on the right."),
        });
    }

    fn next(&mut self) {
        let items = self.items();
        let next = match self.open_index() {
            Some(i) => items.get(i + 1),
            None => items.first(),
        }
        .map(|(_, i)| Self::item_open(i));
        match next {
            Some(n) => self.open(n),
            None => self.message = Some("That's the end of the path.".into()),
        }
    }

    fn read(&mut self) {
        let next = self.items().iter().find(|(_, i)| i.kind == "reading" && !i.done).map(|(_, i)| Self::item_open(i));
        match next {
            Some(r) => self.open(r),
            None => self.message = Some("You've read every page on this path.".into()),
        }
    }

    fn edit(&mut self) -> Effect {
        let (Some(b), Some(slug)) = (self.track(), self.exercise()) else {
            self.message = Some("Open an exercise to edit its file.".into());
            return Effect::None;
        };
        match self.ctx.spec_on(b.id(), slug) {
            Ok(spec) => {
                let rel = backends::learner_files(b, spec).pop().map(|(rel, _)| rel).unwrap_or_else(|| backends::main_file(b, spec));
                Effect::Edit(self.ctx.workspace.join(b.id()).join(rel))
            }
            Err(e) => {
                self.message = Some(e.0);
                Effect::None
            }
        }
    }

    fn set_track(&mut self, b: &'static dyn Backend) -> Result<()> {
        backends::generate(b, &self.ctx.track_specs(b.id()), &self.ctx.workspace.join(b.id()))?;
        self.ctx.config.track = Some(b.id().to_string());
        self.ctx.save_config()?;
        self.tests = None;
        self.open = None;
        self.reload();
        self.open_current();
        Ok(())
    }

    // ---- keys --------------------------------------------------------------------

    pub fn key(&mut self, key: KeyEvent) -> Effect {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return Effect::Quit;
        }
        if self.overlay.is_some() {
            return self.overlay_key(key);
        }
        self.message = None;
        match key.code {
            KeyCode::Char('q') => return Effect::Quit,
            KeyCode::Char('?') => self.overlay = Some(Overlay::Help),
            KeyCode::Char('t') => {
                let cursor = self.track().and_then(|b| backends::all().iter().position(|x| x.id() == b.id())).unwrap_or(0);
                self.overlay = Some(Overlay::Tracks { cursor, first_run: false });
            }
            KeyCode::Char('/') => self.overlay = Some(Overlay::Find { query: String::new(), cursor: 0 }),
            KeyCode::Char('j') | KeyCode::Down => self.cursor = (self.cursor + 1).min(self.items().len().saturating_sub(1)),
            KeyCode::Char('k') | KeyCode::Up => self.cursor = self.cursor.saturating_sub(1),
            KeyCode::Enter => {
                if let Some((_, item)) = self.items().get(self.cursor) {
                    let target = Self::item_open(item);
                    self.open(target);
                    self.show_path = false;
                }
            }
            KeyCode::Tab | KeyCode::BackTab if self.exercise().is_some() => {
                let tabs = self.tabs();
                let i = tabs.iter().position(|t| *t == self.tab).unwrap_or(0);
                let n = tabs.len();
                self.tab = tabs[if key.code == KeyCode::Tab { (i + 1) % n } else { (i + n - 1) % n }];
                self.scroll = 0;
                self.version += 1;
            }
            KeyCode::Char('1') => self.set_tab(Tab::Lesson),
            KeyCode::Char('2') => self.set_tab(Tab::Task),
            KeyCode::Char('3') => self.set_tab(Tab::Compare),
            KeyCode::PageDown | KeyCode::Char(' ') => self.scroll = self.scroll.saturating_add(10),
            KeyCode::PageUp => self.scroll = self.scroll.saturating_sub(10),
            KeyCode::Home => self.scroll = 0,
            KeyCode::Left if self.tab == Tab::Compare => self.compare_col = self.compare_col.saturating_sub(1),
            KeyCode::Right if self.tab == Tab::Compare => self.compare_col += 1,
            KeyCode::Char('a') if self.tab == Tab::Compare => {
                self.compare_all = !self.compare_all;
                self.compare_col = 0;
                self.version += 1;
            }
            KeyCode::Char('p') => self.show_path = !self.show_path,
            KeyCode::Char('n') => self.next(),
            KeyCode::Char('r') => self.read(),
            KeyCode::Char('h') => self.hint(),
            KeyCode::Char('c') => return self.check(),
            KeyCode::Char('e') => return self.edit(),
            _ => {}
        }
        Effect::None
    }

    /// The tabs the open item has: readings have no Task, track-only exercises no Compare.
    pub fn tabs(&self) -> Vec<Tab> {
        let native = match (self.track(), self.exercise()) {
            (Some(b), Some(slug)) => self.ctx.spec_on(b.id(), slug).is_ok_and(|s| s.native.is_some()),
            _ => return vec![Tab::Lesson],
        };
        if native { vec![Tab::Lesson, Tab::Task] } else { vec![Tab::Lesson, Tab::Task, Tab::Compare] }
    }

    fn set_tab(&mut self, tab: Tab) {
        if self.tabs().contains(&tab) {
            self.tab = tab;
            self.scroll = 0;
            self.version += 1;
        }
    }

    /// Path items matching the find box: title, slug or concept.
    pub fn found(&self, query: &str) -> Vec<(usize, &PathItem)> {
        let q = query.to_lowercase();
        self.items()
            .into_iter()
            .filter(|(_, i)| {
                q.is_empty()
                    || i.title.to_lowercase().contains(&q)
                    || i.id.contains(&q)
                    || i.concept.as_deref().is_some_and(|c| c.contains(&q))
            })
            .collect()
    }

    fn overlay_key(&mut self, key: KeyEvent) -> Effect {
        let Some(overlay) = self.overlay.take() else { return Effect::None };
        // First run needs a track: closing the picker there quits instead.
        let picking_first = matches!(overlay, Overlay::Tracks { first_run: true, .. });
        if key.code == KeyCode::Esc || (picking_first && key.code == KeyCode::Char('q')) {
            return if picking_first { Effect::Quit } else { Effect::None };
        }
        let up = matches!(key.code, KeyCode::Up | KeyCode::Char('k'));
        let down = matches!(key.code, KeyCode::Down | KeyCode::Char('j'));
        let step = |cursor: usize, len: usize| {
            if up {
                cursor.saturating_sub(1)
            } else if down {
                (cursor + 1).min(len.saturating_sub(1))
            } else {
                cursor
            }
        };
        self.overlay = match overlay {
            Overlay::Help => None,
            Overlay::Tracks { cursor, first_run } => {
                let all = backends::all();
                if key.code == KeyCode::Enter {
                    match self.set_track(all[cursor]) {
                        Err(e) => {
                            self.message = Some(e.0);
                            Some(Overlay::Tracks { cursor, first_run })
                        }
                        Ok(()) if first_run => {
                            let cursor = BACKGROUNDS.iter().position(|(id, _)| Some(*id) == self.ctx.background()).unwrap_or(0);
                            Some(Overlay::Background { cursor, first_run })
                        }
                        Ok(()) => {
                            self.message = Some(format!("Now on the {} track.", all[cursor].name()));
                            None
                        }
                    }
                } else {
                    Some(Overlay::Tracks { cursor: step(cursor, all.len()), first_run })
                }
            }
            Overlay::Background { cursor, first_run } => {
                if key.code == KeyCode::Enter {
                    self.ctx.config.from = Some(BACKGROUNDS[cursor].0.to_string());
                    if let Err(e) = self.ctx.save_config() {
                        self.message = Some(e.0);
                    }
                    self.version += 1;
                    if first_run {
                        let cursor = GOALS.iter().position(|(id, _)| *id == self.ctx.goal()).unwrap_or(0);
                        Some(Overlay::Goal { cursor })
                    } else {
                        None
                    }
                } else {
                    Some(Overlay::Background { cursor: step(cursor, BACKGROUNDS.len()), first_run })
                }
            }
            Overlay::Goal { cursor } => {
                if key.code == KeyCode::Enter {
                    let goal = GOALS[cursor].0;
                    if goal == "compare" {
                        let mine = self.track().map(|b| b.id()).into_iter().collect();
                        Some(Overlay::CompareTracks { cursor: 0, chosen: mine })
                    } else {
                        self.finish_goal(goal, &[]);
                        None
                    }
                } else {
                    Some(Overlay::Goal { cursor: step(cursor, GOALS.len()) })
                }
            }
            Overlay::CompareTracks { cursor, mut chosen } => {
                let all = backends::all();
                match key.code {
                    KeyCode::Char(' ') => {
                        let id = all[cursor].id();
                        if let Some(i) = chosen.iter().position(|c| *c == id) {
                            chosen.remove(i);
                        } else {
                            chosen.push(id);
                        }
                        Some(Overlay::CompareTracks { cursor, chosen })
                    }
                    KeyCode::Enter if chosen.len() >= 2 => {
                        let list: Vec<String> = chosen.iter().map(|s| s.to_string()).collect();
                        self.finish_goal("compare", &list);
                        None
                    }
                    KeyCode::Enter => {
                        self.message = Some("Pick two or more tracks with Space.".into());
                        Some(Overlay::CompareTracks { cursor, chosen })
                    }
                    _ => Some(Overlay::CompareTracks { cursor: step(cursor, all.len()), chosen }),
                }
            }
            Overlay::Find { mut query, cursor } => match key.code {
                KeyCode::Enter => {
                    if let Some((_, item)) = self.found(&query).get(cursor) {
                        let target = Self::item_open(item);
                        self.open(target);
                    }
                    None
                }
                KeyCode::Backspace => {
                    query.pop();
                    Some(Overlay::Find { query, cursor: 0 })
                }
                KeyCode::Up | KeyCode::Down => {
                    let len = self.found(&query).len();
                    Some(Overlay::Find { cursor: step(cursor, len), query })
                }
                KeyCode::Char(c) => {
                    query.push(c);
                    Some(Overlay::Find { query, cursor: 0 })
                }
                _ => Some(Overlay::Find { query, cursor }),
            },
        };
        Effect::None
    }

    fn finish_goal(&mut self, goal: &str, compare: &[String]) {
        match self.ctx.set_goal(goal, compare) {
            Ok(()) => self.message = Some("All set. ? lists the keys.".into()),
            Err(e) => self.message = Some(e.0),
        }
        self.version += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The repo's course in a fresh workspace under target/.
    fn app(name: &str, track: Option<&str>) -> App {
        let ws = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target").join(format!("tui-{name}"));
        let _ = std::fs::remove_dir_all(&ws);
        let mut ctx = Ctx::discover(Some(ws.clone())).unwrap();
        if let Some(t) = track {
            ctx.config.track = Some(t.to_string());
            ctx.save_config().unwrap();
        }
        App::new(Some(ws), ctx)
    }

    fn press(app: &mut App, code: KeyCode) -> Effect {
        app.key(KeyEvent::new(code, KeyModifiers::NONE))
    }

    #[test]
    fn opens_on_the_current_exercise() {
        let a = app("current", Some("go"));
        assert_eq!(a.open, Some(Open::Exercise("hello".into())));
        assert!(a.items()[a.cursor].1.id == "hello");
        assert!(a.ctx.workspace.join("go").join("hello").is_dir(), "files are generated");
    }

    #[test]
    fn first_run_asks_track_background_goal() {
        let mut a = app("first-run", None);
        assert_eq!(a.overlay, Some(Overlay::Tracks { cursor: 0, first_run: true }));
        let go = backends::all().iter().position(|b| b.id() == "go").unwrap();
        for _ in 0..go {
            press(&mut a, KeyCode::Down);
        }
        press(&mut a, KeyCode::Enter);
        assert_eq!(a.ctx.config.track.as_deref(), Some("go"));
        assert!(matches!(a.overlay, Some(Overlay::Background { first_run: true, .. })));
        press(&mut a, KeyCode::Down);
        press(&mut a, KeyCode::Enter);
        assert_eq!(a.ctx.config.from.as_deref(), Some(BACKGROUNDS[1].0));
        assert!(matches!(a.overlay, Some(Overlay::Goal { .. })));
        // Compare: needs a second track before Enter works.
        press(&mut a, KeyCode::Down);
        press(&mut a, KeyCode::Down);
        press(&mut a, KeyCode::Enter);
        assert_eq!(a.overlay, Some(Overlay::CompareTracks { cursor: 0, chosen: vec!["go"] }));
        press(&mut a, KeyCode::Enter);
        assert!(a.overlay.is_some(), "one track isn't enough");
        press(&mut a, KeyCode::Char(' '));
        press(&mut a, KeyCode::Enter);
        assert_eq!(a.overlay, None);
        assert_eq!(a.ctx.goal(), "compare");
        assert_eq!(a.ctx.config.compare, vec!["go".to_string(), backends::all()[0].id().to_string()]);
    }

    #[test]
    fn esc_on_first_run_quits() {
        let mut a = app("first-run-esc", None);
        assert_eq!(press(&mut a, KeyCode::Esc), Effect::Quit);
    }

    #[test]
    fn moving_and_opening_along_the_path() {
        let mut a = app("moving", Some("go"));
        let start = a.cursor;
        press(&mut a, KeyCode::Char('j'));
        assert_eq!(a.cursor, start + 1);
        press(&mut a, KeyCode::Enter);
        let (_, item) = a.items()[start + 1];
        assert_eq!(a.open, Some(Open::Exercise(item.id.clone())));
        press(&mut a, KeyCode::Char('n'));
        assert_eq!(a.cursor, start + 2);
    }

    #[test]
    fn reading_opens_and_marks_the_first_unread_page() {
        let mut a = app("reading", Some("go"));
        press(&mut a, KeyCode::Char('r'));
        let Some(Open::Reading(id)) = a.open.clone() else { panic!("no reading open") };
        assert!(a.ctx.is_read(&id));
        assert_eq!(a.tabs(), vec![Tab::Lesson]);
        press(&mut a, KeyCode::Char('r'));
        assert_ne!(a.open, Some(Open::Reading(id)), "the next unread page");
    }

    #[test]
    fn tabs_cycle_and_check_starts_tests() {
        let mut a = app("tabs", Some("go"));
        press(&mut a, KeyCode::Tab);
        assert_eq!(a.tab, Tab::Task);
        press(&mut a, KeyCode::Tab);
        assert_eq!(a.tab, Tab::Compare);
        press(&mut a, KeyCode::Tab);
        assert_eq!(a.tab, Tab::Lesson);
        press(&mut a, KeyCode::BackTab);
        assert_eq!(a.tab, Tab::Compare);
        assert_eq!(press(&mut a, KeyCode::Char('c')), Effect::Check { track: "go", slug: "hello".into() });
        assert!(a.tests.as_ref().unwrap().running);
        assert_eq!(press(&mut a, KeyCode::Char('c')), Effect::None, "one run at a time");
        let first = a.tests.as_ref().unwrap().live[0].0;
        a.on_msg(Msg::Case { index: first, passed: true });
        assert_eq!(a.tests.as_ref().unwrap().live[0].2, Some(true));
    }

    #[test]
    fn hints_are_counted_and_saved() {
        let mut a = app("hints", Some("go"));
        press(&mut a, KeyCode::Char('h'));
        assert_eq!(a.ctx.progress.hints.get("hello"), Some(&1));
        let saved = Ctx::discover(a.workspace.clone()).unwrap();
        assert_eq!(saved.progress.hints.get("hello"), Some(&1));
    }

    #[test]
    fn edit_names_the_exercise_file_and_saves_trigger_a_check() {
        let mut a = app("edit", Some("go"));
        let Effect::Edit(file) = press(&mut a, KeyCode::Char('e')) else { panic!("no edit") };
        assert!(file.ends_with("hello/hello.go"), "{}", file.display());
        assert!(matches!(a.on_msg(Msg::Saved), Effect::Check { .. }));
    }

    #[test]
    fn find_filters_and_opens() {
        let mut a = app("find", Some("go"));
        press(&mut a, KeyCode::Char('/'));
        for c in "leap".chars() {
            press(&mut a, KeyCode::Char(c));
        }
        assert!(a.found("leap").iter().all(|(_, i)| i.title.to_lowercase().contains("leap") || i.id.contains("leap")));
        press(&mut a, KeyCode::Enter);
        assert_eq!(a.open, Some(Open::Exercise("leap-year".into())));
        assert_eq!(a.overlay, None);
    }
}
