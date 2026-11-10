//! Lessons and notes for the terminal: markdown rendered with styles instead
//! of markup, wrapped to the width with a margin, links clickable where the
//! terminal allows. All text is kept; only the markup goes.

use anstyle::{Effects, Style};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use crate::ui::{self, Ui, Word};

pub fn render(md: &str, ui: &Ui) -> String {
    let mut r = Renderer { ui, out: Vec::new(), words: Vec::new(), ..Renderer::default_for(ui) };
    for event in Parser::new_ext(md, Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH) {
        r.event(event);
    }
    r.flush();
    while r.out.last().is_some_and(|l| l.is_empty()) {
        r.out.pop();
    }
    r.out.join("\n")
}

struct ListState {
    /// Next number for an ordered list.
    next: Option<u64>,
}

struct Renderer<'a> {
    ui: &'a Ui,
    out: Vec<String>,
    /// The paragraph (or list item, heading, cell) being collected.
    words: Vec<Word>,
    /// A space separates the next word from the previous one.
    space: bool,
    strong: usize,
    emphasis: usize,
    heading: Option<HeadingLevel>,
    links: Vec<String>,
    lists: Vec<ListState>,
    /// Bullet waiting to start the current item's first line.
    item_prefix: Option<String>,
    quote: usize,
    code: Option<(String, String)>,
    table: Option<Vec<Vec<String>>>,
    cell: Option<String>,
}

impl<'a> Renderer<'a> {
    fn default_for(ui: &'a Ui) -> Renderer<'a> {
        Renderer {
            ui,
            out: Vec::new(),
            words: Vec::new(),
            space: false,
            strong: 0,
            emphasis: 0,
            heading: None,
            links: Vec::new(),
            lists: Vec::new(),
            item_prefix: None,
            quote: 0,
            code: None,
            table: None,
            cell: None,
        }
    }

    fn blank(&mut self) {
        if self.out.last().is_some_and(|l| !l.is_empty()) {
            self.out.push(String::new());
        }
    }

    /// Indentation for lists and quotes, after the margin.
    fn indent(&self) -> String {
        let mut s = ui::MARGIN.to_string();
        s.push_str(&"  ".repeat(self.lists.len().saturating_sub(1)));
        if self.quote > 0 {
            s.push_str(&self.ui.muted(if self.ui.unicode { "│ " } else { "| " }));
        }
        s
    }

    fn style(&self) -> Option<Style> {
        let mut s = Style::new();
        let mut any = false;
        if self.strong > 0 || self.heading.is_some() {
            s = s.effects(s.get_effects() | Effects::BOLD);
            any = true;
        }
        if matches!(self.heading, Some(HeadingLevel::H1 | HeadingLevel::H2)) {
            s = s.fg_color(ui::accent_style().get_fg_color());
        }
        if self.emphasis > 0 {
            s = s.effects(s.get_effects() | Effects::ITALIC);
            any = true;
        }
        if !self.links.is_empty() {
            s = s.fg_color(ui::accent_style().get_fg_color());
            any = true;
        }
        any.then_some(s)
    }

    fn push_word(&mut self, text: String, style: Option<Style>) {
        let glue = !self.space && !self.words.is_empty();
        self.words.push(Word { text, style, link: self.links.last().cloned(), glue });
        self.space = false;
    }

    fn text(&mut self, t: &str) {
        if let Some(cell) = &mut self.cell {
            cell.push_str(t);
            return;
        }
        if t.starts_with(char::is_whitespace) {
            self.space = true;
        }
        let style = self.style();
        let mut parts = t.split_whitespace().peekable();
        while let Some(w) = parts.next() {
            self.push_word(w.to_string(), style);
            if parts.peek().is_some() {
                self.space = true;
            }
        }
        if t.ends_with(char::is_whitespace) {
            self.space = true;
        }
    }

    fn inline_code(&mut self, c: &str) {
        if let Some(cell) = &mut self.cell {
            cell.push_str(c);
            return;
        }
        let style = Some(ui::code_style());
        // Short spans stay together; long ones may wrap at their spaces.
        if c.chars().count() <= self.ui.width / 3 {
            self.push_word(c.to_string(), style);
        } else {
            for (i, part) in c.split(' ').enumerate() {
                if i > 0 {
                    self.space = true;
                }
                self.push_word(part.to_string(), style);
            }
        }
    }

    /// Lay out the collected words as one block.
    fn flush(&mut self) {
        if self.words.is_empty() && self.item_prefix.is_none() {
            return;
        }
        let words = std::mem::take(&mut self.words);
        let indent = self.indent();
        let (first, rest) = match self.item_prefix.take() {
            Some(prefix) => {
                let pad = " ".repeat(ui::visible_width(&prefix));
                (format!("{indent}{prefix}"), format!("{indent}{pad}"))
            }
            None if !self.lists.is_empty() => {
                // A later paragraph of a list item lines up with its text.
                let pad = "  ".to_string();
                (format!("{indent}{pad}"), format!("{indent}{pad}"))
            }
            None => (indent.clone(), indent),
        };
        self.out.extend(ui::wrap_words(&words, self.ui.width, &first, &rest, self.ui));
        self.space = false;
    }

    fn event(&mut self, event: Event) {
        match event {
            Event::Start(tag) => self.start(tag),
            Event::End(tag) => self.end(tag),
            Event::Text(t) => match &mut self.code {
                Some((_, body)) => body.push_str(&t),
                None => self.text(&t),
            },
            Event::Code(c) => self.inline_code(&c),
            Event::SoftBreak => self.space = true,
            Event::HardBreak => self.flush(),
            Event::Rule => {
                self.flush();
                self.blank();
                let rule = if self.ui.unicode { "─" } else { "-" }.repeat(self.ui.width.saturating_sub(ui::MARGIN.len()));
                self.out.push(format!("{}{}", ui::MARGIN, self.ui.muted(&rule)));
                self.out.push(String::new());
            }
            // HTML (comments marking background sections) isn't shown.
            Event::Html(_) | Event::InlineHtml(_) => {}
            _ => {}
        }
    }

    fn start(&mut self, tag: Tag) {
        match tag {
            Tag::Paragraph => {
                if self.lists.is_empty() || self.item_prefix.is_none() && self.words.is_empty() {
                    // Paragraph spacing; inside a list only between paragraphs.
                    if self.item_prefix.is_none() {
                        self.blank_unless_tight();
                    }
                }
            }
            Tag::Heading { level, .. } => {
                self.flush();
                self.blank();
                self.heading = Some(level);
            }
            Tag::BlockQuote(_) => {
                self.flush();
                self.blank();
                self.quote += 1;
            }
            Tag::CodeBlock(kind) => {
                self.flush();
                self.blank();
                let lang = match kind {
                    CodeBlockKind::Fenced(l) => l.split(|c: char| c == ',' || c.is_whitespace()).next().unwrap_or("").to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                self.code = Some((lang, String::new()));
            }
            Tag::List(start) => {
                self.flush();
                if self.lists.is_empty() {
                    self.blank();
                }
                self.lists.push(ListState { next: start });
            }
            Tag::Item => {
                self.flush();
                let bullet = match self.lists.last_mut().and_then(|l| l.next.as_mut()) {
                    Some(n) => {
                        let s = format!("{n}. ");
                        *n += 1;
                        s
                    }
                    None => format!("{} ", self.ui.bullet()),
                };
                self.item_prefix = Some(self.ui.muted(&bullet));
            }
            Tag::Emphasis => self.emphasis += 1,
            Tag::Strong => self.strong += 1,
            Tag::Link { dest_url, .. } => self.links.push(dest_url.to_string()),
            Tag::Table(_) => {
                self.flush();
                self.blank();
                self.table = Some(Vec::new());
            }
            Tag::TableHead | Tag::TableRow => {
                if let Some(t) = &mut self.table {
                    t.push(Vec::new());
                }
            }
            Tag::TableCell => self.cell = Some(String::new()),
            _ => {}
        }
    }

    fn blank_unless_tight(&mut self) {
        self.blank();
    }

    fn end(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Paragraph => {
                self.flush();
                if self.lists.is_empty() {
                    self.out.push(String::new());
                }
            }
            TagEnd::Heading(_) => {
                self.flush();
                self.heading = None;
                self.out.push(String::new());
            }
            TagEnd::BlockQuote(_) => {
                self.flush();
                self.quote -= 1;
                self.out.push(String::new());
            }
            TagEnd::CodeBlock => {
                if let Some((lang, body)) = self.code.take() {
                    self.code_block(&lang, &body);
                }
            }
            TagEnd::List(_) => {
                self.flush();
                self.lists.pop();
                if self.lists.is_empty() {
                    self.out.push(String::new());
                }
            }
            TagEnd::Item => self.flush(),
            TagEnd::Emphasis => self.emphasis -= 1,
            TagEnd::Strong => self.strong -= 1,
            TagEnd::Link => {
                if let Some(url) = self.links.pop() {
                    let text: String = self.words.iter().rev().map(|w| w.text.as_str()).take(1).collect();
                    if !self.ui.links && text != url {
                        self.space = true;
                        self.push_word(format!("({url})"), Some(ui::muted_style()));
                    }
                }
            }
            TagEnd::TableCell => {
                if let (Some(cell), Some(t)) = (self.cell.take(), &mut self.table) {
                    if let Some(row) = t.last_mut() {
                        row.push(cell.trim().to_string());
                    }
                }
            }
            TagEnd::Table => {
                if let Some(rows) = self.table.take() {
                    self.table_block(&rows);
                }
            }
            _ => {}
        }
    }

    fn code_block(&mut self, lang: &str, body: &str) {
        let indent = self.indent();
        if !lang.is_empty() {
            self.out.push(format!("{indent}{}", self.ui.muted(lang)));
        }
        let gutter = self.ui.muted(if self.ui.unicode { "│ " } else { "| " });
        // A line too wide for the terminal continues after a dotted gutter,
        // broken at a space, instead of being cut mid-word by the terminal.
        let cont = self.ui.muted(if self.ui.unicode { "┆   " } else { ":   " });
        let room = self.ui.width.saturating_sub(ui::visible_width(&indent) + 2).max(20);
        for line in body.trim_end_matches('\n').lines() {
            let mut first = true;
            for piece in split_code_line(line, room) {
                let g = if first { &gutter } else { &cont };
                self.out.push(format!("{indent}{g}{}", highlight(&piece, lang, self.ui)).trim_end().to_string());
                first = false;
            }
        }
        self.out.push(String::new());
    }

    fn table_block(&mut self, rows: &[Vec<String>]) {
        let cols = rows.iter().map(Vec::len).max().unwrap_or(0);
        let widths: Vec<usize> = (0..cols)
            .map(|c| rows.iter().filter_map(|r| r.get(c)).map(|s| ui::visible_width(s)).max().unwrap_or(0))
            .collect();
        let indent = self.indent();
        let total: usize = widths.iter().sum::<usize>() + 3 * cols.saturating_sub(1) + ui::visible_width(&indent);
        if total <= self.ui.width {
            for (i, row) in rows.iter().enumerate() {
                let cells: Vec<String> = (0..cols)
                    .map(|c| {
                        let text = row.get(c).map(String::as_str).unwrap_or("");
                        let pad = " ".repeat(widths[c] - ui::visible_width(text));
                        if i == 0 { format!("{}{pad}", self.ui.heading(text)) } else { format!("{text}{pad}") }
                    })
                    .collect();
                self.out.push(format!("{indent}{}", cells.join(&self.ui.muted(" │ "))).trim_end().to_string());
            }
        } else {
            // Too wide to line up: one wrapped entry per row, cells named by the header.
            let header = rows.first().cloned().unwrap_or_default();
            for row in rows.iter().skip(1) {
                let text = row
                    .iter()
                    .enumerate()
                    .map(|(c, cell)| match header.get(c) {
                        Some(h) if !h.is_empty() => format!("{h}: {cell}"),
                        _ => cell.clone(),
                    })
                    .collect::<Vec<_>>()
                    .join("; ");
                let bullet = format!("{} ", self.ui.bullet());
                let pad = " ".repeat(bullet.chars().count());
                self.out.extend(self.ui.wrap(&text, &format!("{indent}{}", self.ui.muted(&bullet)), &format!("{indent}{pad}")));
            }
        }
        self.out.push(String::new());
    }
}

/// Split a code line into pieces of at most `room` columns, at spaces where
/// possible. Continuation pieces get two fewer columns (their gutter is wider).
fn split_code_line(line: &str, room: usize) -> Vec<String> {
    let mut pieces = Vec::new();
    let mut rest = line.to_string();
    let mut limit = room;
    while ui::visible_width(&rest) > limit {
        let chars: Vec<char> = rest.chars().collect();
        let mut cut = limit.min(chars.len());
        // Back up to a space, but not into the line's indentation.
        let indent = chars.iter().take_while(|c| c.is_whitespace()).count();
        if let Some(sp) = chars[..cut].iter().rposition(|c| *c == ' ').filter(|&p| p > indent) {
            cut = sp;
        }
        pieces.push(chars[..cut].iter().collect::<String>().trim_end().to_string());
        rest = chars[cut..].iter().collect::<String>().trim_start().to_string();
        limit = room.saturating_sub(2).max(10);
    }
    pieces.push(rest);
    pieces
}

/// Light highlighting with no grammar: comments dimmed, strings and numbers
/// coloured. Enough to make code blocks easy to scan.
pub fn highlight(line: &str, lang: &str, ui: &Ui) -> String {
    if !ui.color {
        return line.to_string();
    }
    let comment: &[&str] = match lang {
        "lua" => &["--"],
        "sh" | "bash" | "toml" | "python" | "py" | "powershell" | "ps1" | "yaml" => &["#"],
        "php" => &["//", "#"],
        "text" | "" => &[],
        _ => &["//"],
    };
    let mut out = String::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    let rest = |i: usize| chars[i..].iter().collect::<String>();
    while i < chars.len() {
        let tail = rest(i);
        if comment.iter().any(|c| tail.starts_with(c)) || tail.starts_with("/*") && !comment.is_empty() {
            out.push_str(&ui.muted(&tail));
            break;
        }
        let c = chars[i];
        if c == '"' || c == '\'' || c == '`' {
            let mut j = i + 1;
            while j < chars.len() && chars[j] != c {
                if chars[j] == '\\' {
                    j += 1;
                }
                j += 1;
            }
            let end = (j + 1).min(chars.len());
            out.push_str(&ui.paint(ui::success_style(), &chars[i..end].iter().collect::<String>()));
            i = end;
            continue;
        }
        let boundary = i == 0 || !(chars[i - 1].is_alphanumeric() || chars[i - 1] == '_');
        if c.is_ascii_digit() && boundary {
            let mut j = i;
            while j < chars.len() && (chars[j].is_ascii_alphanumeric() || chars[j] == '.' || chars[j] == '_') {
                j += 1;
            }
            out.push_str(&ui.paint(ui::code_style(), &chars[i..j].iter().collect::<String>()));
            i = j;
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plain(md: &str, width: usize) -> String {
        render(md, &Ui::plain(width))
    }

    #[test]
    fn drops_markup_keeps_text() {
        let out = plain("# Title\n\nSome **bold**, *emphasis* and `code`.\n", 80);
        assert_eq!(out, "  Title\n\n  Some bold, emphasis and code.");
    }

    #[test]
    fn wraps_paragraphs_with_a_margin() {
        let out = plain("one two three four five six seven eight nine ten eleven twelve", 30);
        assert_eq!(out, "  one two three four five six\n  seven eight nine ten eleven\n  twelve");
        assert!(out.lines().all(|l| l.chars().count() <= 30));
    }

    #[test]
    fn punctuation_stays_glued_to_code_and_links() {
        assert_eq!(plain("Use `x`, then [docs](https://e.org).", 80), "  Use x, then docs (https://e.org).");
        assert_eq!(plain("See <https://e.org>.", 80), "  See https://e.org.");
    }

    #[test]
    fn lists_hang_their_wrapped_lines() {
        let out = plain("- alpha beta gamma delta epsilon zeta\n- two\n\n1. first\n2. second\n", 24);
        assert_eq!(out, "  • alpha beta gamma\n    delta epsilon zeta\n  • two\n\n  1. first\n  2. second");
    }

    #[test]
    fn code_blocks_get_a_label_and_gutter() {
        let out = plain("Text.\n\n```rust\nfn main() {\n    let x = 1;\n}\n```\n\nAfter.\n", 80);
        assert_eq!(out, "  Text.\n\n  rust\n  │ fn main() {\n  │     let x = 1;\n  │ }\n\n  After.");
    }

    #[test]
    fn tables_line_up_or_fall_back_to_a_list() {
        let md = "| A | B |\n|---|---|\n| one | two |\n";
        assert_eq!(plain(md, 80), "  A   │ B\n  one │ two");
        let wide = "| Name | Description |\n|---|---|\n| x | a long description that cannot fit next to its name |\n";
        assert_eq!(plain(wide, 40), "  • Name: x; Description: a long\n    description that cannot fit next to\n    its name");
    }

    #[test]
    fn every_word_of_a_real_note_survives() {
        let note = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../tracks/rust/interfaces.md")).unwrap();
        let out = plain(&note, 60);
        let words = |s: &str| -> Vec<String> {
            s.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_lowercase).collect()
        };
        // Markup (#, *, `, link targets, HTML comments) aside, the words are the
        // same, in order, plus each code block's language label.
        let source = pulldown_cmark::Parser::new(&note)
            .filter_map(|e| match e {
                Event::Text(t) | Event::Code(t) => Some(t.to_string()),
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => Some(lang.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(words(&out), words(&source));
        assert!(out.lines().all(|l| ui::visible_width(l) <= 60 || l.contains('│')), "{out}");
    }

    /// The Syntax Primer is mostly tables: each page and note fits a narrow
    /// terminal (tables line up or become lists; only code may run past).
    #[test]
    fn primer_pages_fit_the_terminal() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let mut pages = Vec::new();
        for dir in std::iter::once(root.join("concepts")).chain(["typescript", "python", "rust", "go", "php", "lua", "qml"].map(|t| root.join("tracks").join(t))) {
            for e in std::fs::read_dir(&dir).unwrap().flatten() {
                let name = e.file_name().to_string_lossy().to_string();
                if name.starts_with("syntax-") && name.ends_with(".md") {
                    pages.push(e.path());
                }
            }
        }
        for path in pages {
            let md = crate::notes::render(&std::fs::read_to_string(&path).unwrap(), None);
            for width in [60, 100] {
                let out = plain(&md, width);
                let long: Vec<&str> = out.lines().filter(|l| ui::visible_width(l) > width && !l.contains('│')).collect();
                assert!(long.is_empty(), "{} at width {width}:\n{}", path.display(), long.join("\n"));
            }
        }
    }

    #[test]
    fn colour_marks_headings_code_and_comments() {
        let ui = Ui { color: true, ..Ui::plain(80) };
        let out = render("## Key\n\nUse `x`.\n\n```go\nx := 1 // one\n```\n", &ui);
        assert!(out.contains("\x1b[1m\x1b[36mKey") || out.contains("\x1b[1;36mKey") || out.contains("Key\x1b[0m"), "{out:?}");
        assert!(out.contains("\x1b[35mx\x1b[0m"), "{out:?}");
        assert!(out.contains("\x1b[2m// one\x1b[0m"), "{out:?}");
    }
}
