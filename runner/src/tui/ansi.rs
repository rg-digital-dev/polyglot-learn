//! The CLI's renderers (Markdown, task card, hints) produce strings with ANSI
//! colour codes. The TUI reuses them and turns those codes into ratatui text, so
//! there is one renderer for both. Only what `ui.rs` emits needs handling: SGR
//! colours and effects, and OSC 8 links (dropped; the TUI shows link text).

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};

/// Rendered terminal text as ratatui text, one line per `\n`.
pub fn to_text(s: &str) -> Text<'static> {
    let mut style = Style::default();
    Text::from(s.split('\n').map(|line| to_line(line, &mut style)).collect::<Vec<_>>())
}

/// One line; `style` carries over, as it does on a terminal.
fn to_line(line: &str, style: &mut Style) -> Line<'static> {
    let mut spans = Vec::new();
    let mut text = String::new();
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\x1b' {
            text.push(c);
            continue;
        }
        match chars.next() {
            // CSI: parameters, then a final byte. Only `m` (SGR) changes the style.
            Some('[') => {
                let mut params = String::new();
                let mut fin = None;
                for p in chars.by_ref() {
                    if ('\x40'..='\x7e').contains(&p) {
                        fin = Some(p);
                        break;
                    }
                    params.push(p);
                }
                if fin == Some('m') {
                    if !text.is_empty() {
                        spans.push(Span::styled(std::mem::take(&mut text), *style));
                    }
                    *style = apply_sgr(*style, &params);
                }
            }
            // OSC (links, titles): skip to BEL or ESC \.
            Some(']') => {
                while let Some(p) = chars.next() {
                    if p == '\x07' {
                        break;
                    }
                    if p == '\x1b' && chars.peek() == Some(&'\\') {
                        chars.next();
                        break;
                    }
                }
            }
            _ => {}
        }
    }
    if !text.is_empty() {
        spans.push(Span::styled(text, *style));
    }
    Line::from(spans)
}

fn basic(n: u16) -> Color {
    match n {
        0 => Color::Black,
        1 => Color::Red,
        2 => Color::Green,
        3 => Color::Yellow,
        4 => Color::Blue,
        5 => Color::Magenta,
        6 => Color::Cyan,
        7 => Color::Gray,
        8 => Color::DarkGray,
        9 => Color::LightRed,
        10 => Color::LightGreen,
        11 => Color::LightYellow,
        12 => Color::LightBlue,
        13 => Color::LightMagenta,
        14 => Color::LightCyan,
        _ => Color::White,
    }
}

fn apply_sgr(mut style: Style, params: &str) -> Style {
    let nums: Vec<u16> = if params.is_empty() { vec![0] } else { params.split(';').map(|p| p.parse().unwrap_or(0)).collect() };
    let mut i = 0;
    while i < nums.len() {
        match nums[i] {
            0 => style = Style::default(),
            1 => style = style.add_modifier(Modifier::BOLD),
            2 => style = style.add_modifier(Modifier::DIM),
            3 => style = style.add_modifier(Modifier::ITALIC),
            4 => style = style.add_modifier(Modifier::UNDERLINED),
            9 => style = style.add_modifier(Modifier::CROSSED_OUT),
            22 => style = style.remove_modifier(Modifier::BOLD | Modifier::DIM),
            23 => style = style.remove_modifier(Modifier::ITALIC),
            24 => style = style.remove_modifier(Modifier::UNDERLINED),
            29 => style = style.remove_modifier(Modifier::CROSSED_OUT),
            n @ 30..=37 => style = style.fg(basic(n - 30)),
            n @ 90..=97 => style = style.fg(basic(n - 90 + 8)),
            39 => style.fg = None,
            n @ 40..=47 => style = style.bg(basic(n - 40)),
            n @ 100..=107 => style = style.bg(basic(n - 100 + 8)),
            49 => style.bg = None,
            n @ (38 | 48) => {
                let color = match nums.get(i + 1) {
                    Some(5) => {
                        i += 2;
                        nums.get(i).map(|&c| Color::Indexed(c as u8))
                    }
                    Some(2) => {
                        i += 4;
                        match (nums.get(i - 2), nums.get(i - 1), nums.get(i)) {
                            (Some(&r), Some(&g), Some(&b)) => Some(Color::Rgb(r as u8, g as u8, b as u8)),
                            _ => None,
                        }
                    }
                    _ => None,
                };
                if n == 38 {
                    style.fg = color;
                } else {
                    style.bg = color;
                }
            }
            _ => {}
        }
        i += 1;
    }
    style
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::{self, Ui};

    fn colour_ui() -> Ui {
        Ui { color: true, ..Ui::plain(80) }
    }

    #[test]
    fn plain_text_is_one_span_per_line() {
        let t = to_text("one\ntwo");
        assert_eq!(t.lines.len(), 2);
        assert_eq!(t.lines[1].spans[0].content, "two");
        assert_eq!(t.lines[1].spans[0].style, Style::default());
    }

    #[test]
    fn ui_styles_become_ratatui_styles() {
        let ui = colour_ui();
        let s = format!("{} {} {} {}", ui.heading("H"), ui.accent("a"), ui.muted("m"), ui.paint(ui::warning_style(), "w"));
        let line = &to_text(&s).lines[0];
        let styled: Vec<(&str, Style)> = line.spans.iter().map(|s| (s.content.as_ref(), s.style)).collect();
        assert_eq!(
            styled,
            vec![
                ("H", Style::default().add_modifier(Modifier::BOLD)),
                (" ", Style::default()),
                ("a", Style::default().fg(Color::Cyan)),
                (" ", Style::default()),
                ("m", Style::default().add_modifier(Modifier::DIM)),
                (" ", Style::default()),
                ("w", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]
        );
    }

    #[test]
    fn styles_carry_across_lines_and_links_keep_their_text() {
        let t = to_text("\x1b[31mred\nstill red\x1b[0m \x1b]8;;https://x.dev\x1b\\site\x1b]8;;\x1b\\");
        assert_eq!(t.lines[1].spans[0].style.fg, Some(Color::Red));
        let last: String = t.lines[1].spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(last, "still red site");
    }

    #[test]
    fn indexed_and_rgb_colours() {
        let t = to_text("\x1b[38;5;208mo\x1b[48;2;1;2;3mb");
        assert_eq!(t.lines[0].spans[0].style.fg, Some(Color::Indexed(208)));
        assert_eq!(t.lines[0].spans[1].style.bg, Some(Color::Rgb(1, 2, 3)));
    }
}
