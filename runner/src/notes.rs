//! Lesson notes and their "coming from X" sections.
//!
//! Notes mark background-specific blocks like this:
//!
//! ```text
//! <!-- from:typescript -->
//! ## Coming from TypeScript
//! ...
//! <!-- /from -->
//! <!-- from:default -->
//! ## Key points
//! ...
//! <!-- /from -->
//! ```
//!
//! The block matching the learner's background is kept; if none matches, the
//! `default` block is kept instead. Text outside blocks is always shown.

use std::path::Path;

use crate::course::TrackMeta;

/// Backgrounds offered by `learn init`. Notes may add sections for any of them.
pub const BACKGROUNDS: &[(&str, &str)] = &[
    ("typescript", "TypeScript"),
    ("javascript", "JavaScript"),
    ("python", "Python"),
    ("php", "PHP"),
    ("go", "Go"),
    ("rust", "Rust"),
    ("java", "Java"),
    ("csharp", "C#"),
    ("lua", "Lua"),
    ("none", "None of these / prefer neutral notes"),
];

/// Background sections to try, most specific first.
fn candidates(from: &str) -> Vec<&str> {
    match from {
        "javascript" => vec!["javascript", "typescript"],
        other => vec![other],
    }
}

struct Block {
    key: String,
    start: usize,
    end: usize,
}

pub fn render(md: &str, from: Option<&str>) -> String {
    let lines: Vec<&str> = md.lines().collect();
    let mut blocks = Vec::new();
    let mut open: Option<(String, usize)> = None;
    for (i, line) in lines.iter().enumerate() {
        let t = line.trim();
        if let Some(key) = t.strip_prefix("<!-- from:").and_then(|r| r.strip_suffix("-->")) {
            open = Some((key.trim().to_ascii_lowercase(), i));
        } else if t == "<!-- /from -->" {
            if let Some((key, start)) = open.take() {
                blocks.push(Block { key, start, end: i });
            }
        }
    }

    let wanted = from.map(candidates).unwrap_or_default();
    let chosen = wanted
        .iter()
        .find_map(|w| blocks.iter().position(|b| b.key == *w).map(|_| w.to_string()))
        .unwrap_or_else(|| "default".to_string());

    let mut out = Vec::new();
    let mut skip_until = None;
    for (i, line) in lines.iter().enumerate() {
        if let Some(end) = skip_until {
            if i <= end {
                continue;
            }
            skip_until = None;
        }
        if let Some(b) = blocks.iter().find(|b| b.start == i) {
            if b.key != chosen {
                skip_until = Some(b.end);
            }
            continue;
        }
        if blocks.iter().any(|b| b.end == i) {
            continue;
        }
        out.push(*line);
    }
    // Collapse the blank lines left behind by removed blocks.
    let mut text = String::new();
    let mut blank = 0;
    for line in out {
        if line.trim().is_empty() {
            blank += 1;
            if blank > 1 {
                continue;
            }
        } else {
            blank = 0;
        }
        text.push_str(line);
        text.push('\n');
    }
    text.trim().to_string()
}

pub fn read_concept(root: &Path, concept: &str, from: Option<&str>) -> Option<String> {
    let text = std::fs::read_to_string(root.join("concepts").join(format!("{concept}.md"))).ok()?;
    Some(render(&text, from))
}

pub fn read_track_note(root: &Path, track: &str, concept: &str, from: Option<&str>) -> Option<String> {
    let text = std::fs::read_to_string(root.join("tracks").join(track).join(format!("{concept}.md"))).ok()?;
    Some(render(&text, from))
}

/// "Go deeper" section rendered from the track's link data.
pub fn go_deeper(meta: &TrackMeta, concept: &str) -> Option<String> {
    let links = meta.links_for(concept);
    if links.is_empty() {
        return None;
    }
    let mut out = String::from("## Go deeper\n");
    for l in links {
        out.push_str(&format!("- {}: {} <{}>\n", meta.kind_label(&l.kind), l.title, l.url));
    }
    Some(out.trim_end().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOTE: &str = "# Title\n\nIntro.\n\n<!-- from:typescript -->\n## Coming from TypeScript\nTS bit.\n<!-- /from -->\n\n<!-- from:python -->\n## Coming from Python\nPy bit.\n<!-- /from -->\n\n<!-- from:default -->\n## Key points\nNeutral bit.\n<!-- /from -->\n\n## Common mistake\nOops.\n";

    #[test]
    fn keeps_the_matching_section() {
        let out = render(NOTE, Some("typescript"));
        assert!(out.contains("TS bit."));
        assert!(!out.contains("Py bit."));
        assert!(!out.contains("Neutral bit."));
        assert!(out.contains("Intro.") && out.contains("Oops."));
        assert!(!out.contains("<!--"));
        assert!(!out.contains("\n\n\n"));
    }

    #[test]
    fn falls_back_to_default() {
        let out = render(NOTE, Some("java"));
        assert!(out.contains("Neutral bit."));
        assert!(!out.contains("TS bit.") && !out.contains("Py bit."));
        assert!(render(NOTE, None).contains("Neutral bit."));
    }

    #[test]
    fn javascript_uses_typescript_section() {
        assert!(render(NOTE, Some("javascript")).contains("TS bit."));
        let js = NOTE.replace("from:python", "from:javascript");
        let out = render(&js, Some("javascript"));
        assert!(out.contains("Py bit.") && !out.contains("TS bit."));
    }

    #[test]
    fn notes_without_markers_are_unchanged() {
        assert_eq!(render("# A\n\nB\n", Some("python")), "# A\n\nB");
    }
}
