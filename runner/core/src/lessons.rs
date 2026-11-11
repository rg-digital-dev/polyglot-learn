//! `LESSON.md` beside each exercise: the task, the concept lesson, the track's
//! note and links, written for the learner's background. Editors show it next
//! to the code (Zed's Markdown preview, VS Code's), so the lesson is there
//! without a terminal or browser. It's regenerated like the tests, never edited.

use std::path::{Path, PathBuf};

use crate::backends::{self, Backend, Generated};
use crate::course::TrackMeta;
use crate::ctx::Ctx;
use crate::error::Result;
use crate::notes::{self, BACKGROUNDS};
use crate::spec::Spec;

/// The file name, in each exercise's folder.
pub const FILE: &str = "LESSON.md";

/// Where an exercise's `LESSON.md` goes, relative to the track folder.
pub fn path(spec: &Spec) -> PathBuf {
    PathBuf::from(&spec.slug).join(FILE)
}

/// Stubs, tests and every `LESSON.md` for a track, plus Zed's tasks when
/// missing: what setting up a track (`learn start`, `learn generate`, the web
/// UI, the TUI) writes.
pub fn generate_track(ctx: &Ctx, b: &dyn Backend) -> Result<Generated> {
    let dir = ctx.workspace.join(b.id());
    let specs = ctx.track_specs(b.id());
    let report = backends::generate(b, &specs, &dir)?;
    write_all(ctx, b, &specs, &dir)?;
    crate::zed::write_tasks(ctx)?;
    Ok(report)
}

/// Rewrite every `LESSON.md` for a track, e.g. after the background changed.
/// Unchanged files aren't touched, so watchers see nothing.
pub fn write_all(ctx: &Ctx, b: &dyn Backend, specs: &[Spec], track_dir: &Path) -> Result<()> {
    let meta = TrackMeta::load(&ctx.root, b.id())?;
    for spec in specs {
        backends::write(&track_dir.join(path(spec)), &markdown(ctx, b, &meta, spec))?;
    }
    Ok(())
}

/// After the background or goal changed: rewrite the active track's lessons.
pub fn refresh(ctx: &Ctx) -> Result<()> {
    let Some(track) = ctx.config.track.as_deref() else { return Ok(()) };
    let b = backends::get(track)?;
    write_all(ctx, b, &ctx.track_specs(b.id()), &ctx.workspace.join(b.id()))
}

/// One exercise's `LESSON.md`.
pub fn markdown(ctx: &Ctx, b: &dyn Backend, meta: &TrackMeta, spec: &Spec) -> String {
    let style = ctx.note_style();
    let from = BACKGROUNDS.iter().find(|(id, _)| Some(*id) == style.from).map(|(_, l)| format!(", coming from {l}"));
    let mut out = vec![
        format!(
            "<!-- Written by `learn` for the {} track{}. It's rewritten, so keep your own notes elsewhere. -->",
            b.name(),
            from.unwrap_or_default()
        ),
        format!("# {}", spec.title),
        format!("**{}** · {}", spec.concept, b.name()),
        spec.main.summary.clone(),
    ];

    let mut edit = Vec::new();
    for (rel, sigs) in backends::learner_files(b, spec) {
        let rel = rel.strip_prefix(&spec.slug).unwrap_or(&rel).components().map(|c| c.as_os_str().to_string_lossy()).collect::<Vec<_>>().join("/");
        edit.push(format!("**Edit** `{rel}`"));
        if !sigs.is_empty() {
            edit.push(format!("```{}\n{}\n```", b.fence(), sigs.join("\n")));
        }
    }
    out.push(edit.join("\n\n"));
    out.push("Save, and `learn` (or `learn watch`) runs the tests; `learn check` runs them once. Stuck? `learn hint`.".into());

    let mut parts = Vec::new();
    match spec.native {
        Some(_) => parts.extend(notes::read_native_lesson(spec, style)),
        None => {
            parts.extend(notes::read_concept(&ctx.root, &spec.concept, style));
            parts.extend(notes::read_track_note(&ctx.root, b.id(), &spec.concept, style));
        }
    }
    for p in parts {
        out.push("---".into());
        out.push(p.trim().to_string());
    }

    let links = meta.links_for(&spec.concept);
    if !links.is_empty() {
        out.push("---".into());
        out.push("## Go deeper".into());
        out.push(links.iter().map(|l| format!("- {}: [{}]({})", meta.kind_label(&l.kind), l.title, l.url)).collect::<Vec<_>>().join("\n"));
    }
    let mut text = out.join("\n\n");
    text.push('\n');
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(name: &str) -> Ctx {
        let ws = std::env::temp_dir().join(format!("learn-lessons-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&ws);
        Ctx::discover(Some(ws)).unwrap()
    }

    #[test]
    fn every_exercise_on_the_track_gets_a_lesson() {
        let ctx = ctx("all");
        let go = backends::get("go").unwrap();
        generate_track(&ctx, go).unwrap();
        let dir = ctx.workspace.join("go");
        for spec in ctx.track_specs("go") {
            assert!(dir.join(path(&spec)).is_file(), "{} has no LESSON.md", spec.slug);
        }
        let hello = std::fs::read_to_string(dir.join("hello").join(FILE)).unwrap();
        assert!(hello.starts_with("<!-- Written by `learn` for the Go track."), "{hello}");
        assert!(hello.contains("# Hello\n"));
        assert!(hello.contains("**Edit** `hello.go`\n\n```go\nfunc Greet(name string) string\n```"), "{hello}");
        assert!(hello.contains("\n---\n"), "the lesson and the note follow");
    }

    #[test]
    fn the_background_shapes_the_lesson() {
        let mut ctx = ctx("from");
        let go = backends::get("go").unwrap();
        let spec = ctx.spec_on("go", "hello").unwrap().clone();
        let meta = TrackMeta::load(&ctx.root, "go").unwrap();
        let plain = markdown(&ctx, go, &meta, &spec);
        ctx.config.from = Some("python".into());
        let python = markdown(&ctx, go, &meta, &spec);
        assert!(python.contains("coming from Python"));
        assert_ne!(plain, python);
    }

    #[test]
    fn a_track_only_exercise_gets_its_own_lesson() {
        let ctx = ctx("native");
        let rust = backends::get("rust").unwrap();
        let spec = ctx.track_specs("rust").into_iter().find(|s| s.native.is_some()).expect("rust has a track-only exercise");
        let meta = TrackMeta::load(&ctx.root, "rust").unwrap();
        let md = markdown(&ctx, rust, &meta, &spec);
        assert!(md.contains(&format!("# {}\n", spec.title)));
        assert!(md.contains("\n---\n"), "the native lesson is included: {md}");
    }
}
