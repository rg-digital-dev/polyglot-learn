//! Terminal output: the one place that knows about colour, glyphs, width,
//! clickable links and the pager. Commands ask for meaning (`heading`,
//! `failure`, a path) and never write escape codes themselves.
//!
//! Colour uses the 16 basic ANSI colours plus bold/dim, so the terminal's own
//! theme decides the exact shade and it reads on dark and light backgrounds.

use std::io::{IsTerminal, Write};
use std::path::Path;
use std::sync::OnceLock;

use anstyle::{AnsiColor, Effects, Style};
use unicode_width::UnicodeWidthStr;

/// `--color`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum ColorWhen {
    Auto,
    Always,
    Never,
}

/// Text is never wider than this, even on a wide terminal.
pub const MAX_WIDTH: usize = 88;
/// Left margin for lesson text and cards.
pub const MARGIN: &str = "  ";

#[derive(Debug, Clone)]
pub struct Ui {
    pub color: bool,
    /// Colour for stderr (errors), decided separately: it may be a terminal when stdout isn't.
    pub err_color: bool,
    pub unicode: bool,
    /// stdout is a terminal.
    pub tty: bool,
    /// OSC 8 hyperlinks.
    pub links: bool,
    /// Usable width, at most MAX_WIDTH.
    pub width: usize,
    pub height: usize,
    pub pager: bool,
}

static UI: OnceLock<Ui> = OnceLock::new();

/// Decide everything once, from the flags, the environment and the terminal.
pub fn init(when: ColorWhen, ascii: bool, no_pager: bool) {
    let stdout_tty = std::io::stdout().is_terminal();
    let stderr_tty = std::io::stderr().is_terminal();
    let env = |k: &str| std::env::var(k).ok().filter(|v| !v.is_empty());
    let auto = |tty: bool| {
        if env("NO_COLOR").is_some() {
            false
        } else if env("CLICOLOR_FORCE").is_some_and(|v| v != "0") {
            true
        } else {
            tty && env("TERM").as_deref() != Some("dumb")
        }
    };
    let (color, err_color) = match when {
        ColorWhen::Always => (true, true),
        ColorWhen::Never => (false, false),
        ColorWhen::Auto => (auto(stdout_tty), auto(stderr_tty)),
    };
    // anstream prints what we give it (and converts for old Windows consoles).
    anstream::ColorChoice::write_global(if color { anstream::ColorChoice::Always } else { anstream::ColorChoice::Never });

    let size = terminal_size::terminal_size();
    let columns = env("COLUMNS").and_then(|c| c.parse().ok()).or(size.map(|(w, _)| w.0 as usize));
    let lines = env("LINES").and_then(|c| c.parse().ok()).or(size.map(|(_, h)| h.0 as usize));
    let ui = Ui {
        color,
        err_color,
        unicode: !ascii && unicode_terminal(),
        tty: stdout_tty,
        links: stdout_tty && color && hyperlinks_supported(),
        width: columns.unwrap_or(MAX_WIDTH).clamp(40, MAX_WIDTH),
        height: lines.unwrap_or(24).max(5),
        pager: !no_pager && stdout_tty,
    };
    let _ = UI.set(ui);
}

/// The settings `init` chose; plain output at 80 columns if it never ran (tests).
pub fn get() -> &'static Ui {
    UI.get_or_init(|| Ui::plain(80))
}

/// Is the terminal's encoding UTF-8? Windows consoles take Unicode directly.
fn unicode_terminal() -> bool {
    if cfg!(windows) {
        return true;
    }
    ["LC_ALL", "LC_CTYPE", "LANG"]
        .iter()
        .find_map(|k| std::env::var(k).ok().filter(|v| !v.is_empty()))
        .is_some_and(|v| {
            let v = v.to_ascii_lowercase();
            v.contains("utf-8") || v.contains("utf8")
        })
}

/// Terminals known to support OSC 8 links; elsewhere links print as `text (url)`.
fn hyperlinks_supported() -> bool {
    let env = |k: &str| std::env::var(k).ok().filter(|v| !v.is_empty());
    if env("LEARN_NO_HYPERLINKS").is_some() {
        return false;
    }
    if env("WT_SESSION").is_some() // Windows Terminal
        || env("KITTY_WINDOW_ID").is_some()
        || env("WEZTERM_EXECUTABLE").is_some()
        || env("KONSOLE_VERSION").is_some()
    {
        return true;
    }
    if let Some(p) = env("TERM_PROGRAM") {
        if ["WezTerm", "iTerm.app", "vscode", "ghostty", "Hyper"].contains(&p.as_str()) {
            return true;
        }
    }
    if env("VTE_VERSION").and_then(|v| v.parse::<u32>().ok()).is_some_and(|v| v >= 5000) {
        return true; // GNOME Terminal, Tilix and other VTE terminals
    }
    env("TERM").is_some_and(|t| t.starts_with("foot") || t == "xterm-kitty" || t == "xterm-ghostty")
}

// ---- styles ------------------------------------------------------------------

fn fg(c: AnsiColor) -> Style {
    Style::new().fg_color(Some(c.into()))
}

pub fn heading_style() -> Style {
    Style::new().effects(Effects::BOLD)
}
pub fn accent_style() -> Style {
    fg(AnsiColor::Cyan)
}
pub fn muted_style() -> Style {
    Style::new().effects(Effects::DIMMED)
}
pub fn code_style() -> Style {
    fg(AnsiColor::Magenta)
}
pub fn path_style() -> Style {
    Style::new().effects(Effects::UNDERLINE)
}
pub fn success_style() -> Style {
    fg(AnsiColor::Green)
}
pub fn failure_style() -> Style {
    fg(AnsiColor::Red)
}
/// Bold: plain yellow is faint on light backgrounds.
pub fn warning_style() -> Style {
    fg(AnsiColor::Yellow).effects(Effects::BOLD)
}

impl Ui {
    /// No colour, no links, no pager: what tests and pipes get.
    pub fn plain(width: usize) -> Ui {
        Ui { color: false, err_color: false, unicode: true, tty: false, links: false, width, height: 24, pager: false }
    }

    pub fn paint(&self, style: Style, text: &str) -> String {
        if self.color && !text.is_empty() {
            format!("{style}{text}{style:#}")
        } else {
            text.to_string()
        }
    }

    pub fn heading(&self, t: &str) -> String {
        self.paint(heading_style(), t)
    }
    pub fn accent(&self, t: &str) -> String {
        self.paint(accent_style(), t)
    }
    pub fn muted(&self, t: &str) -> String {
        self.paint(muted_style(), t)
    }
    pub fn code(&self, t: &str) -> String {
        self.paint(code_style(), t)
    }
    pub fn success(&self, t: &str) -> String {
        self.paint(success_style(), t)
    }
    pub fn failure(&self, t: &str) -> String {
        self.paint(failure_style(), t)
    }
    pub fn warning(&self, t: &str) -> String {
        self.paint(warning_style(), t)
    }

    /// An error for stderr: red `error:`, the first line bold, the rest (usually
    /// what to do about it) indented underneath.
    pub fn error(&self, msg: &str) -> String {
        let paint = |style: Style, t: &str| if self.err_color { format!("{style}{t}{style:#}") } else { t.to_string() };
        let mut lines = msg.lines();
        let first = lines.next().unwrap_or("");
        let mut out = format!("{} {}", paint(failure_style().effects(Effects::BOLD), "error:"), paint(heading_style(), first));
        // The rest (usually what to do) wraps under the message.
        let pad = "       ";
        for l in lines {
            for w in self.wrap(l, pad, pad) {
                out.push('\n');
                out.push_str(&w);
            }
        }
        out
    }

    // ---- glyphs --------------------------------------------------------------

    pub fn pass_mark(&self) -> String {
        self.success(if self.unicode { "✓" } else { "ok" })
    }
    pub fn fail_mark(&self) -> String {
        self.failure(if self.unicode { "✗" } else { "FAIL" })
    }
    pub fn current_mark(&self) -> String {
        self.accent(if self.unicode { "●" } else { ">" })
    }
    pub fn todo_mark(&self) -> String {
        self.muted(if self.unicode { "○" } else { "-" })
    }
    pub fn dot(&self) -> &'static str {
        if self.unicode { "·" } else { "|" }
    }
    pub fn bullet(&self) -> &'static str {
        if self.unicode { "•" } else { "-" }
    }

    /// `███████░░░` for done/total, `width` cells wide.
    pub fn bar(&self, done: usize, total: usize, width: usize) -> String {
        let filled = (done * width + total / 2).checked_div(total).unwrap_or(0).min(width);
        let (full, empty) = if self.unicode { ("█", "░") } else { ("#", ".") };
        format!("{}{}", self.success(&full.repeat(filled)), self.muted(&empty.repeat(width - filled)))
    }

    // ---- links and paths -------------------------------------------------------

    /// Clickable `text` (OSC 8) where supported, else `text (url)`.
    pub fn link(&self, text: &str, url: &str) -> String {
        if self.links {
            format!("\x1b]8;;{url}\x1b\\{text}\x1b]8;;\x1b\\")
        } else if text == url {
            url.to_string()
        } else {
            format!("{text} {}", self.muted(&format!("({url})")))
        }
    }

    /// A path for display: relative to the current folder when that's shorter,
    /// underlined, and clickable (file://) where the terminal supports it.
    pub fn path(&self, p: &Path) -> String {
        let shown = short_path(p);
        let styled = self.paint(path_style(), &shown);
        match (self.links, std::path::absolute(p)) {
            (true, Ok(abs)) => format!("\x1b]8;;{}\x1b\\{styled}\x1b]8;;\x1b\\", file_url(&abs)),
            _ => styled,
        }
    }

    // ---- layout --------------------------------------------------------------

    /// Wrap plain text to the width, every line starting with `indent` (the
    /// first line with `first` instead). Words never break mid-word.
    pub fn wrap(&self, text: &str, first: &str, indent: &str) -> Vec<String> {
        let words: Vec<Word> = text.split_whitespace().map(|w| Word::plain(w.to_string())).collect();
        wrap_words(&words, self.width, first, indent, self)
    }

    /// Print through the pager when the text is taller than the terminal.
    pub fn page(&self, text: &str) {
        let lines = text.lines().count();
        if self.pager && lines + 1 > self.height && spawn_pager(text) {
            return;
        }
        anstream::println!("{text}");
    }

    /// Clear the screen (watch mode), only on a terminal.
    pub fn clear_screen(&self) {
        if self.tty {
            anstream::print!("\x1b[2J\x1b[H");
            let _ = std::io::stdout().flush();
        }
    }
}

/// The local time of day as `HH:MM:SS` (watch mode's "last run").
pub fn clock() -> String {
    #[cfg(windows)]
    {
        // SAFETY: GetLocalTime only fills in the struct we pass.
        let t = unsafe {
            let mut t = std::mem::zeroed::<windows_sys::Win32::Foundation::SYSTEMTIME>();
            windows_sys::Win32::System::SystemInformation::GetLocalTime(&mut t);
            t
        };
        format!("{:02}:{:02}:{:02}", t.wHour, t.wMinute, t.wSecond)
    }
    #[cfg(unix)]
    {
        // SAFETY: localtime_r writes only into the struct we pass.
        unsafe {
            let now = libc::time(std::ptr::null_mut());
            let mut tm = std::mem::zeroed::<libc::tm>();
            libc::localtime_r(&now, &mut tm);
            format!("{:02}:{:02}:{:02}", tm.tm_hour, tm.tm_min, tm.tm_sec)
        }
    }
    #[cfg(not(any(windows, unix)))]
    {
        String::new()
    }
}

/// Relative to the current folder when that's shorter, with `/` separators
/// inside the course for readability.
pub fn short_path(p: &Path) -> String {
    let abs = std::path::absolute(p).unwrap_or_else(|_| p.to_path_buf());
    let cwd = std::env::current_dir().unwrap_or_default();
    let full = abs.display().to_string();
    match abs.strip_prefix(&cwd) {
        Ok(rel) if !rel.as_os_str().is_empty() => {
            let rel = rel.components().map(|c| c.as_os_str().to_string_lossy()).collect::<Vec<_>>().join("/");
            if rel.len() < full.len() { rel } else { full }
        }
        _ => full,
    }
}

fn file_url(abs: &Path) -> String {
    let s = abs.display().to_string().replace('\\', "/");
    let s = s.strip_prefix("//?/").unwrap_or(&s).to_string();
    let encoded: String = s
        .chars()
        .map(|c| match c {
            ' ' => "%20".to_string(),
            '#' => "%23".to_string(),
            '?' => "%3F".to_string(),
            c => c.to_string(),
        })
        .collect();
    if encoded.starts_with('/') { format!("file://{encoded}") } else { format!("file:///{encoded}") }
}

fn spawn_pager(text: &str) -> bool {
    use std::process::{Command, Stdio};
    let (prog, args): (String, Vec<String>) = match std::env::var("PAGER").ok().filter(|p| !p.trim().is_empty()) {
        Some(p) => {
            let mut parts = p.split_whitespace().map(String::from);
            let prog = parts.next().unwrap_or_default();
            if prog == "cat" {
                return false;
            }
            (prog, parts.collect())
        }
        None => match crate::toolchain::which("less") {
            Some(less) => (less.display().to_string(), vec!["-R".into(), "-F".into(), "-X".into()]),
            None => return false,
        },
    };
    let Ok(mut child) = Command::new(&prog).args(&args).stdin(Stdio::piped()).spawn() else {
        return false;
    };
    if let Some(mut stdin) = child.stdin.take() {
        // The reader may quit early (q in less): a broken pipe is fine.
        let _ = stdin.write_all(text.as_bytes());
        let _ = stdin.write_all(b"\n");
    }
    child.wait().is_ok()
}

// ---- word wrapping ---------------------------------------------------------------

/// A unit of wrapped text: shown as one piece (styled and linked as a whole),
/// never split across lines. `glue` means no space before it.
#[derive(Debug, Clone)]
pub struct Word {
    pub text: String,
    pub style: Option<Style>,
    pub link: Option<String>,
    pub glue: bool,
}

impl Word {
    pub fn plain(text: String) -> Word {
        Word { text, style: None, link: None, glue: false }
    }
}

/// Display width of text without escape codes.
pub fn visible_width(s: &str) -> usize {
    UnicodeWidthStr::width(strip_escapes(s).as_str())
}

/// Remove SGR (`ESC [ … m`) and OSC (`ESC ] … ESC \`) sequences.
pub fn strip_escapes(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\x1b' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('[') => {
                for c in chars.by_ref() {
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
            Some(']') => {
                while let Some(c) = chars.next() {
                    if c == '\x1b' && chars.peek() == Some(&'\\') {
                        chars.next();
                        break;
                    }
                    if c == '\x07' {
                        break;
                    }
                }
            }
            _ => {}
        }
    }
    out
}

fn render_word(w: &Word, ui: &Ui) -> String {
    let text = match w.style {
        Some(s) => ui.paint(s, &w.text),
        None => w.text.clone(),
    };
    match (&w.link, ui.links) {
        (Some(url), true) => format!("\x1b]8;;{url}\x1b\\{text}\x1b]8;;\x1b\\"),
        _ => text,
    }
}

/// Lay words out in lines of at most `width` columns.
pub fn wrap_words(words: &[Word], width: usize, first: &str, indent: &str, ui: &Ui) -> Vec<String> {
    // Words glued together (`(` + code + `)`, a link + its full stop) move as one.
    let mut units: Vec<Vec<&Word>> = Vec::new();
    for w in words {
        match units.last_mut() {
            Some(unit) if w.glue => unit.push(w),
            _ => units.push(vec![w]),
        }
    }
    let mut lines = Vec::new();
    let mut line = first.to_string();
    let mut used = UnicodeWidthStr::width(first);
    let mut empty = true;
    for unit in units {
        let wlen: usize = unit.iter().map(|w| UnicodeWidthStr::width(w.text.as_str())).sum();
        let gap = if empty { 0 } else { 1 };
        if !empty && used + gap + wlen > width {
            lines.push(std::mem::take(&mut line));
            line = indent.to_string();
            used = UnicodeWidthStr::width(indent);
            empty = true;
        }
        if !empty {
            line.push(' ');
            used += 1;
        }
        for w in unit {
            line.push_str(&render_word(w, ui));
        }
        used += wlen;
        empty = false;
    }
    if !empty || lines.is_empty() {
        lines.push(line);
    }
    lines.iter().map(|l| l.trim_end().to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_without_breaking_words() {
        let ui = Ui::plain(20);
        let lines = ui.wrap("the quick brown fox jumps over the lazy dog", "  ", "  ");
        assert_eq!(lines, ["  the quick brown", "  fox jumps over the", "  lazy dog"]);
        assert!(lines.iter().all(|l| l.chars().count() <= 20));
        // A word longer than the line stays whole on its own line.
        assert_eq!(ui.wrap("a supercalifragilistic b", "", ""), ["a", "supercalifragilistic", "b"]);
    }

    #[test]
    fn plain_ui_has_no_escapes() {
        let ui = Ui::plain(80);
        assert_eq!(ui.heading("x"), "x");
        assert_eq!(ui.link("docs", "https://e.org"), "docs (https://e.org)");
        assert_eq!(ui.bar(1, 2, 4), "██░░");
    }

    #[test]
    fn colour_and_links_use_escape_codes() {
        let ui = Ui { color: true, links: true, ..Ui::plain(80) };
        assert_eq!(ui.failure("no"), "\x1b[31mno\x1b[0m");
        assert_eq!(ui.link("docs", "https://e.org"), "\x1b]8;;https://e.org\x1b\\docs\x1b]8;;\x1b\\");
        assert_eq!(visible_width(&ui.link(&ui.heading("docs"), "https://e.org")), 4);
    }

    #[test]
    fn ascii_glyphs() {
        let ui = Ui { unicode: false, ..Ui::plain(80) };
        assert_eq!([ui.pass_mark(), ui.fail_mark(), ui.current_mark(), ui.todo_mark()], ["ok", "FAIL", ">", "-"]);
        assert_eq!(ui.bar(1, 2, 4), "##..");
    }
}
