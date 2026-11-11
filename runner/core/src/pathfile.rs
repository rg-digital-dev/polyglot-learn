//! `PATH.md` in each track's workspace folder: the path in order, stage by
//! stage, with what's done and where you are, linking every exercise's file and
//! lesson. Editors sort folders by name; this is the order to work in. It's
//! rewritten when a track is set up and whenever an exercise passes.

use std::path::{Path, PathBuf};

use crate::backends::{self, Backend};
use crate::ctx::Ctx;
use crate::error::Result;
use crate::lessons;

/// The file name, in each track's workspace folder.
pub const FILE: &str = "PATH.md";

/// Where a track's `PATH.md` goes.
pub fn path(ctx: &Ctx, track: &str) -> PathBuf {
    ctx.workspace.join(track).join(FILE)
}

/// Rewrite a track's `PATH.md` (unchanged files aren't touched).
pub fn write(ctx: &Ctx, b: &dyn Backend) -> Result<()> {
    backends::write(&path(ctx, b.id()), &markdown(ctx, b))
}

/// A relative link inside the track folder, `/`-separated for Markdown.
fn link(p: &Path) -> String {
    p.components().map(|c| c.as_os_str().to_string_lossy()).collect::<Vec<_>>().join("/")
}

pub fn markdown(ctx: &Ctx, b: &dyn Backend) -> String {
    let track = b.id();
    let specs = ctx.path_specs(track);
    let done = specs.iter().filter(|s| ctx.is_done(track, &s.slug)).count();
    let current = ctx.current(track).map(|s| s.slug.clone());

    let mut out = vec![
        "<!-- Written by `learn`, and rewritten as you go: keep your own notes elsewhere. -->".to_string(),
        format!("# Your {} path", b.name()),
    ];
    let mut status = format!("{done} of {} done.", specs.len());
    match specs.iter().find(|s| Some(&s.slug) == current.as_ref()) {
        Some(next) => status.push_str(&format!(" Next: **[{}]({})**.", next.title, link(&backends::main_file(b, next)))),
        None => status.push_str(" All done!"),
    }
    out.push(status);
    out.push("The folders are numbered in this order. Open an exercise's file, save it, and `learn` runs its tests.".into());

    for step in ctx.path(track) {
        let module = &ctx.course.modules[step.module];
        let on: Vec<_> = ctx.module_exercises(step).filter_map(|slug| specs.iter().find(|s| s.slug == *slug)).collect();
        let Some(first) = on.first() else { continue };
        let stage = first.stages.get(track).cloned().unwrap_or_default();
        out.push(format!("## {} · `{stage}/`", module.title));
        let lines: Vec<String> = on
            .iter()
            .map(|s| {
                let file = link(&backends::main_file(b, s));
                let lesson = link(&lessons::path(track, s));
                let (mark, name, here) = if ctx.is_done(track, &s.slug) {
                    ("✓", format!("[{}]({file})", s.title), "")
                } else if Some(&s.slug) == current.as_ref() {
                    ("→", format!("**[{}]({file})**", s.title), " ← you are here")
                } else {
                    ("○", format!("[{}]({file})", s.title), "")
                };
                format!("- {mark} {name} · {} · [lesson]({lesson}){here}", s.concept)
            })
            .collect();
        out.push(lines.join("\n"));
    }
    let mut text = out.join("\n\n");
    text.push('\n');
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_the_path_in_order_with_progress() {
        let ws = std::env::temp_dir().join(format!("learn-pathfile-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&ws);
        let mut ctx = Ctx::discover(Some(ws)).unwrap();
        let go = backends::get("go").unwrap();
        ctx.progress.done.entry("go".into()).or_default().insert("hello".into());
        let md = markdown(&ctx, go);
        assert!(md.contains("# Your Go path"));
        assert!(md.contains("## Foundations · `01-foundations/`"), "{md}");
        assert!(md.contains("- ✓ [Hello](01-foundations/hello/hello.go) · functions · [lesson](01-foundations/hello/LESSON.md)"), "{md}");
        assert_eq!(md.matches("← you are here").count(), 1);
        let stages: Vec<usize> = md.match_indices("\n## ").map(|(i, _)| i).collect();
        assert!(stages.len() > 2 && stages.windows(2).all(|w| w[0] < w[1]));
        let numbers: Vec<&str> = md.lines().filter(|l| l.starts_with("## ")).map(|l| l.rsplit('`').nth(1).unwrap()).collect();
        let mut sorted = numbers.clone();
        sorted.sort();
        assert_eq!(numbers, sorted, "stage folders sort in path order");
    }
}
