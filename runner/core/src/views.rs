//! What the interfaces show, as plain data. Each function reads the course
//! state and returns a serialisable view; the web UI sends it as JSON, and the
//! CLI (later the TUI and the language server) draw the same data. Field names
//! are the web API's, so they're camelCase where the API is.

use serde::Serialize;

use crate::backends::{self, Backend};
use crate::course::{Families, TrackMeta};
use crate::ctx::{Ctx, Reading};
use crate::error::Result;
use crate::notes;
use crate::spec::{CaseKind, Outcome, Spec};

// ---- the course overview -----------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct CourseView {
    pub title: String,
    pub track: Option<String>,
    pub from: Option<String>,
    /// learn, switch or compare.
    pub goal: String,
    /// For the `compare` goal: the tracks compare views start with.
    pub compare: Vec<String>,
    pub backgrounds: Vec<Background>,
    /// What `learn init` and the first-run screen offer as goals.
    pub goals: Vec<Background>,
    pub tracks: Vec<TrackView>,
    pub modules: Vec<ModuleView>,
    pub families: Vec<FamilyView>,
    /// Concepts that have a shared exercise, in course order (the compare chips).
    pub concepts: Vec<String>,
    /// Nothing done on the active track and nothing read: point at the primer.
    pub new: bool,
    pub workspace: String,
}

#[derive(Debug, Serialize)]
pub struct Background {
    pub id: &'static str,
    pub label: &'static str,
}

#[derive(Debug, Serialize)]
pub struct FamilyView {
    pub id: String,
    pub title: String,
    pub about: String,
    pub tracks: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
pub struct TrackView {
    pub id: &'static str,
    pub name: &'static str,
    pub family: Option<String>,
    /// What the language is for, in one line.
    #[serde(rename = "for")]
    pub purpose: Option<String>,
    pub tool: ToolView,
    pub done: usize,
    pub current: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ToolView {
    pub label: &'static str,
    pub found: bool,
    pub path: Option<String>,
    pub version: Option<String>,
    pub warning: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ModuleView {
    pub id: String,
    pub title: String,
    pub track: Option<String>,
    /// Every track the module is on: all of them, its own (`track`), or its `tracks` list.
    pub tracks: Vec<&'static str>,
    pub exercises: Vec<ExerciseSummary>,
    pub readings: Vec<ReadingSummary>,
}

#[derive(Debug, Serialize)]
pub struct ExerciseSummary {
    pub slug: String,
    pub title: String,
    pub concept: String,
    /// The tracks it's done on.
    pub done: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
pub struct ReadingSummary {
    pub id: String,
    pub title: String,
    pub read: bool,
}

pub fn track(ctx: &Ctx, b: &dyn Backend) -> TrackView {
    let tool = b.tool().find();
    let meta = TrackMeta::load(&ctx.root, b.id()).unwrap_or_default();
    TrackView {
        id: b.id(),
        name: b.name(),
        family: meta.family,
        purpose: meta.purpose,
        tool: ToolView {
            label: b.tool().label,
            found: tool.is_some(),
            path: tool.as_ref().map(|t| t.path.display().to_string()),
            version: tool.as_ref().map(|t| t.version.clone()),
            warning: tool.as_ref().and_then(|t| b.check_tool(t)),
        },
        done: ctx.progress.done.get(b.id()).map_or(0, |d| d.len()),
        current: ctx.current(b.id()).map(|s| s.slug.clone()),
    }
}

pub fn course(ctx: &Ctx) -> CourseView {
    let modules = ctx
        .course
        .modules
        .iter()
        .enumerate()
        .map(|(i, m)| ModuleView {
            id: m.id.clone(),
            title: m.title.clone(),
            track: m.track.clone(),
            tracks: backends::ids().into_iter().filter(|t| ctx.module_on_track(i, t)).collect(),
            exercises: m
                .exercises
                .iter()
                .filter_map(|slug| ctx.spec(slug).ok())
                .map(|s| ExerciseSummary {
                    slug: s.slug.clone(),
                    title: s.title.clone(),
                    concept: s.concept.clone(),
                    done: backends::ids().into_iter().filter(|t| ctx.is_done(t, &s.slug)).collect(),
                })
                .collect(),
            readings: ctx
                .readings
                .iter()
                .filter(|r| m.readings.contains(&r.id))
                .map(|r| ReadingSummary { id: r.id.clone(), title: r.title.clone(), read: ctx.is_read(&r.id) })
                .collect(),
        })
        .collect();
    CourseView {
        title: ctx.course.title.clone(),
        track: ctx.config.track.clone(),
        from: ctx.config.from.clone(),
        goal: ctx.goal().to_string(),
        compare: ctx.config.compare.clone(),
        backgrounds: notes::BACKGROUNDS.iter().map(|(id, label)| Background { id, label }).collect(),
        goals: crate::ctx::GOALS.iter().map(|(id, label)| Background { id, label }).collect(),
        tracks: backends::all().iter().map(|b| track(ctx, *b)).collect(),
        modules,
        families: Families::load(&ctx.root)
            .unwrap_or_default()
            .family
            .into_iter()
            .map(|(id, f)| FamilyView { tracks: family_members(ctx, &id), id, title: f.title, about: f.about })
            .collect(),
        concepts: shared_concepts(ctx),
        new: ctx.config.track.as_deref().is_some_and(|t| ctx.is_new(t)),
        workspace: ctx.workspace.display().to_string(),
    }
}

/// A track's family, from its track.toml.
pub fn family_of(ctx: &Ctx, track: &str) -> Option<String> {
    TrackMeta::load(&ctx.root, track).ok().and_then(|m| m.family)
}

/// The tracks in a family, in track order.
pub fn family_members(ctx: &Ctx, family: &str) -> Vec<&'static str> {
    backends::ids().into_iter().filter(|t| family_of(ctx, t).as_deref() == Some(family)).collect()
}

/// Concepts with a shared (not track-only) exercise, in course order.
pub fn shared_concepts(ctx: &Ctx) -> Vec<String> {
    let mut v: Vec<String> = ctx.specs.iter().filter(|s| s.native.is_none()).map(|s| s.concept.clone()).collect();
    v.dedup();
    v
}

// ---- one exercise ------------------------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseView {
    pub slug: String,
    pub title: String,
    pub concept: String,
    pub summary: String,
    pub module: Option<String>,
    pub track: &'static str,
    pub track_name: &'static str,
    pub fence: &'static str,
    pub tool_found: bool,
    pub tool_label: &'static str,
    pub done: bool,
    pub files: Vec<FileView>,
    pub native: bool,
    pub tracks: Vec<&'static str>,
    pub lesson: Option<String>,
    pub note: Option<String>,
    pub links: Vec<LinkView>,
    pub hints: Vec<String>,
    pub hints_total: usize,
    pub cases: Vec<CaseView>,
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FileView {
    /// Relative to the track folder, with `/` separators.
    pub path: String,
    pub absolute: String,
    /// The signatures the learner must keep, as written in the stub.
    pub signatures: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct LinkView {
    pub kind: String,
    pub label: String,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct CaseView {
    pub index: usize,
    pub description: String,
    /// Not known for a native exercise, whose tests are code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    /// The call and its expected result, e.g. `greet("Alice") → "Hello, Alice!"`.
    pub lines: Vec<String>,
}

pub fn links(meta: &TrackMeta, concept: &str) -> Vec<LinkView> {
    meta.links_for(concept)
        .iter()
        .map(|l| LinkView { kind: l.kind.clone(), label: meta.kind_label(&l.kind), title: l.title.clone(), url: l.url.clone() })
        .collect()
}

pub fn cases(spec: &Spec) -> Vec<CaseView> {
    if spec.native.is_some() {
        return spec
            .cases()
            .into_iter()
            .map(|(_, c)| CaseView { index: c.index, description: c.description.clone(), function: None, lines: Vec::new() })
            .collect();
    }
    let join = |vals: &[serde_json::Value]| vals.iter().map(backends::json).collect::<Vec<_>>().join(", ");
    spec.cases()
        .into_iter()
        .map(|(f, c)| {
            let lines = match &c.kind {
                CaseKind::Call { input, outcome } => {
                    let expected = match outcome {
                        Outcome::Value(v) => backends::json(v),
                        Outcome::Error(contains) => crate::spec::describe_error(contains),
                    };
                    vec![format!("{}({}) → {expected}", f.name, join(input))]
                }
                CaseKind::Scenario { create, steps } => {
                    let mut l: Vec<String> =
                        create.iter().enumerate().map(|(i, a)| format!("v{i} = {}({})", f.name, join(a))).collect();
                    l.extend(steps.iter().map(|s| format!("v{}({}) → {}", s.on, join(&s.input), backends::json(&s.expected))));
                    l
                }
            };
            CaseView { index: c.index, description: c.description.clone(), function: Some(f.name.clone()), lines }
        })
        .collect()
}

fn rel_string(p: &std::path::Path) -> String {
    p.components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect::<Vec<_>>().join("/")
}

/// An exercise on a track. The caller makes sure its files exist.
pub fn exercise(ctx: &Ctx, b: &dyn Backend, spec: &Spec) -> Result<ExerciseView> {
    let dir = ctx.workspace.join(b.id());
    let meta = TrackMeta::load(&ctx.root, b.id())?;
    let shown = ctx.progress.hints.get(&spec.slug).copied().unwrap_or(0).min(spec.hints.len());
    let order: Vec<&str> = ctx.path_specs(b.id()).iter().map(|s| s.slug.as_str()).collect();
    let pos = order.iter().position(|s| *s == spec.slug).unwrap_or(0);
    let from = ctx.note_style();
    Ok(ExerciseView {
        slug: spec.slug.clone(),
        title: spec.title.clone(),
        concept: spec.concept.clone(),
        summary: spec.main.summary.clone(),
        module: ctx.course.modules.iter().find(|m| m.exercises.contains(&spec.slug)).map(|m| m.title.clone()),
        track: b.id(),
        track_name: b.name(),
        fence: b.fence(),
        tool_found: b.tool().find().is_some(),
        tool_label: b.tool().label,
        done: ctx.is_done(b.id(), &spec.slug),
        files: backends::learner_files(b, spec)
            .into_iter()
            .map(|(rel, signatures)| FileView { path: rel_string(&rel), absolute: dir.join(&rel).display().to_string(), signatures })
            .collect(),
        native: spec.native.is_some(),
        tracks: backends::ids().into_iter().filter(|t| spec.on_track(t)).collect(),
        lesson: match spec.native {
            Some(_) => notes::read_native_lesson(spec, from),
            None => notes::read_concept(&ctx.root, &spec.concept, from),
        },
        note: match spec.native {
            Some(_) => None,
            None => notes::read_track_note(&ctx.root, b.id(), &spec.concept, from),
        },
        links: links(&meta, &spec.concept),
        hints: spec.hints.iter().take(shown).cloned().collect(),
        hints_total: spec.hints.len(),
        cases: cases(spec),
        prev: pos.checked_sub(1).and_then(|i| order.get(i)).map(|s| s.to_string()),
        next: order.get(pos + 1).map(|s| s.to_string()),
    })
}

// ---- comparing one concept across tracks ----------------------------------------------

#[derive(Debug, Serialize)]
pub struct CompareView {
    pub concept: String,
    pub lesson: Option<String>,
    /// The first shared exercise for the concept, if it has one.
    pub exercise: Option<String>,
    pub columns: Vec<CompareColumn>,
    /// The page's title when the concept is a reading.
    pub reading: Option<String>,
    /// Why the families of the shown tracks differ: one note per pair of families.
    pub why: Vec<WhyView>,
}

#[derive(Debug, Serialize)]
pub struct WhyView {
    pub between: [String; 2],
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct CompareColumn {
    pub track: &'static str,
    pub name: &'static str,
    pub note: Option<String>,
    pub links: Vec<LinkView>,
}

/// Whether `concept` can be compared: a reading, or a concept with a shared exercise.
pub fn is_comparable(ctx: &Ctx, concept: &str) -> bool {
    ctx.readings.iter().any(|r| r.id == concept) || ctx.specs.iter().any(|s| s.native.is_none() && s.concept == concept)
}

/// The tracks a concept is on: for a reading, the tracks its module is on; for a
/// shared exercise concept, the tracks with an exercise for it (`files` isn't on QML).
pub fn tracks_for(ctx: &Ctx, concept: &str) -> Vec<&'static dyn Backend> {
    let reading = ctx.readings.iter().find(|r| r.id == concept);
    backends::all()
        .iter()
        .copied()
        .filter(|b| match reading {
            Some(r) => ctx.module_on_track(r.module, b.id()),
            None => ctx.specs.iter().any(|s| s.native.is_none() && s.concept == concept && s.on_track(b.id())),
        })
        .collect()
}

/// Where comparing a concept starts for a learner on `track`: the concept's
/// tracks in that track's family. Every track the concept is on when there's no
/// track, no family, or fewer than two family members have it.
pub fn default_compare_tracks(ctx: &Ctx, concept: &str, track: Option<&str>) -> Vec<&'static dyn Backend> {
    let all = tracks_for(ctx, concept);
    // The `compare` goal: the tracks the learner chose, where the concept has them.
    if ctx.goal() == "compare" && !ctx.config.compare.is_empty() {
        let chosen: Vec<&'static dyn Backend> = all.iter().copied().filter(|b| ctx.config.compare.iter().any(|c| c == b.id())).collect();
        if chosen.len() >= 2 {
            return chosen;
        }
    }
    let Some(family) = track.and_then(|t| family_of(ctx, t)) else { return all };
    let members = family_members(ctx, &family);
    let mine: Vec<&'static dyn Backend> = all.iter().copied().filter(|b| members.contains(&b.id())).collect();
    if mine.len() < 2 { all } else { mine }
}

/// Why-notes for every pair of families among `tracks`.
pub fn why_notes(ctx: &Ctx, tracks: &[&str]) -> Vec<WhyView> {
    let families = Families::load(&ctx.root).unwrap_or_default();
    let mut present: Vec<String> = tracks.iter().filter_map(|t| family_of(ctx, t)).collect();
    present.sort();
    present.dedup();
    let mut out = Vec::new();
    for (i, a) in present.iter().enumerate() {
        for b in &present[i + 1..] {
            if let Some(w) = families.why(a, b) {
                out.push(WhyView { between: w.between.clone(), text: w.text.clone() });
            }
        }
    }
    out
}

/// `concept` in each of `wanted` (all of its tracks when empty). The caller checks
/// `is_comparable` first.
pub fn compare(ctx: &Ctx, concept: &str, wanted: &[String]) -> Result<CompareView> {
    let from = ctx.note_style();
    let mut columns = Vec::new();
    for b in tracks_for(ctx, concept) {
        if !wanted.is_empty() && !wanted.iter().any(|w| w == b.id()) {
            continue;
        }
        let meta = TrackMeta::load(&ctx.root, b.id())?;
        columns.push(CompareColumn {
            track: b.id(),
            name: b.name(),
            note: notes::read_track_note(&ctx.root, b.id(), concept, from),
            links: links(&meta, concept),
        });
    }
    let columns_tracks: Vec<&str> = columns.iter().map(|c| c.track).collect();
    Ok(CompareView {
        concept: concept.to_string(),
        lesson: notes::read_concept(&ctx.root, concept, from),
        exercise: ctx.specs.iter().find(|s| s.native.is_none() && s.concept == concept).map(|s| s.slug.clone()),
        reading: ctx.readings.iter().find(|r| r.id == concept).map(|r| r.title.clone()),
        why: why_notes(ctx, &columns_tracks),
        columns,
    })
}

// ---- one track's path -----------------------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PathView {
    pub track: &'static str,
    pub track_name: &'static str,
    /// What the language is for, in one line.
    #[serde(rename = "for")]
    pub purpose: Option<String>,
    pub stages: Vec<StageView>,
    /// The first unfinished exercise on the path.
    pub current: Option<String>,
    pub done: usize,
    pub total: usize,
    pub read: usize,
    pub readings: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StageView {
    pub id: String,
    pub title: String,
    /// The track's own stage (written for this language), not a shared module.
    pub own: bool,
    pub items: Vec<PathItem>,
}

#[derive(Debug, Serialize)]
pub struct PathItem {
    /// `reading` or `exercise`.
    pub kind: &'static str,
    /// The reading id or the exercise slug.
    pub id: String,
    pub title: String,
    pub concept: Option<String>,
    /// Read, or passed on this track.
    pub done: bool,
}

/// A track's path: its stages in order, each reading and exercise with its mark.
pub fn path(ctx: &Ctx, b: &dyn Backend) -> PathView {
    let track = b.id();
    let stages: Vec<StageView> = ctx
        .path(track)
        .iter()
        .map(|step| {
            let m = &ctx.course.modules[step.module];
            let readings = ctx.readings.iter().filter(|r| m.readings.contains(&r.id)).map(|r| PathItem {
                kind: "reading",
                id: r.id.clone(),
                title: r.title.clone(),
                concept: None,
                done: ctx.is_read(&r.id),
            });
            let exercises = ctx.module_exercises(step).filter_map(|slug| ctx.spec(slug).ok()).map(|s| PathItem {
                kind: "exercise",
                id: s.slug.clone(),
                title: s.title.clone(),
                concept: Some(s.concept.clone()),
                done: ctx.is_done(track, &s.slug),
            });
            StageView { id: m.id.clone(), title: m.title.clone(), own: m.track.is_some(), items: readings.chain(exercises).collect() }
        })
        .collect();
    let count = |kind: &str, done: bool| {
        stages.iter().flat_map(|s| &s.items).filter(|i| i.kind == kind && (!done || i.done)).count()
    };
    PathView {
        track,
        track_name: b.name(),
        purpose: TrackMeta::load(&ctx.root, track).ok().and_then(|m| m.purpose),
        current: ctx.current(track).map(|s| s.slug.clone()),
        done: count("exercise", true),
        total: count("exercise", false),
        read: count("reading", true),
        readings: count("reading", false),
        stages,
    }
}

// ---- a reading page ------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct ReadingView {
    pub id: String,
    pub title: String,
    pub module: String,
    /// 1-based position among the readings on the track.
    pub index: usize,
    pub count: usize,
    pub prev: Option<ReadingBrief>,
    pub next: Option<ReadingBrief>,
    pub track: Option<&'static str>,
    pub lesson: Option<String>,
    pub note: Option<String>,
    pub read: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReadingBrief {
    pub id: String,
    pub title: String,
}

pub fn reading(ctx: &Ctx, r: &Reading, backend: Option<&dyn Backend>) -> ReadingView {
    let track = backend.map(|b| b.id());
    let list = ctx.readings_for(track);
    let i = list.iter().position(|x| x.id == r.id);
    let brief = |x: &&Reading| ReadingBrief { id: x.id.clone(), title: x.title.clone() };
    let from = ctx.note_style();
    ReadingView {
        id: r.id.clone(),
        title: r.title.clone(),
        module: ctx.course.modules[r.module].title.clone(),
        index: i.map_or(0, |i| i + 1),
        count: list.len(),
        prev: i.and_then(|i| i.checked_sub(1)).and_then(|i| list.get(i)).map(brief),
        next: i.and_then(|i| list.get(i + 1)).map(brief),
        track,
        lesson: notes::read_concept(&ctx.root, &r.id, from),
        note: track.and_then(|t| notes::read_track_note(&ctx.root, t, &r.id, from)),
        read: ctx.is_read(&r.id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Map, Value};

    /// Keys at every level, with arrays merged into one element shape and every
    /// scalar as "value": what the web UI depends on, without content or
    /// machine-specific values (tool paths, versions).
    fn shape(v: &Value) -> Value {
        match v {
            Value::Object(m) => Value::Object(m.iter().map(|(k, v)| (k.clone(), shape(v))).collect()),
            Value::Array(items) => {
                let merged = items.iter().map(shape).reduce(merge);
                Value::Array(merged.into_iter().collect())
            }
            _ => json!("value"),
        }
    }

    fn merge(a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::Object(mut a), Value::Object(b)) => {
                for (k, v) in b {
                    match a.get_mut(&k) {
                        Some(old) => *old = merge(old.take(), v),
                        None => {
                            a.insert(k, v);
                        }
                    }
                }
                Value::Object(a)
            }
            (Value::Array(a), Value::Array(b)) => Value::Array(a.into_iter().chain(b).reduce(merge).into_iter().collect()),
            (a, _) => a,
        }
    }

    /// The web API's JSON shapes. After an intended change, regenerate the file
    /// with `LEARN_UPDATE_SHAPES=1 cargo test -p learn-core api_shapes` and
    /// update runner/web to match.
    #[test]
    fn api_shapes_are_pinned() {
        let workspace = std::env::temp_dir().join(format!("learn-shape-test-{}", std::process::id()));
        let ctx = Ctx::discover(Some(workspace.clone())).unwrap();
        let go = backends::get("go").unwrap();
        let rust = backends::get("rust").unwrap();
        let lua = backends::get("lua").unwrap();
        let mut got = Map::new();
        let mut put = |name: &str, v: Value| {
            got.insert(name.to_string(), shape(&v));
        };
        put("state", serde_json::to_value(course(&ctx)).unwrap());
        put("exercise", serde_json::to_value(exercise(&ctx, go, ctx.spec("hello").unwrap()).unwrap()).unwrap());
        put("exercise-native", serde_json::to_value(exercise(&ctx, rust, ctx.spec("scoped-guard").unwrap()).unwrap()).unwrap());
        put("compare", serde_json::to_value(compare(&ctx, "functions", &[]).unwrap()).unwrap());
        put("compare-reading", serde_json::to_value(compare(&ctx, "syntax-types", &[]).unwrap()).unwrap());
        put("reading", serde_json::to_value(reading(&ctx, ctx.reading("syntax-types").unwrap(), Some(lua))).unwrap());
        put("path", serde_json::to_value(path(&ctx, go)).unwrap());
        let _ = std::fs::remove_dir_all(&workspace);

        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("api-shapes.json");
        let text = serde_json::to_string_pretty(&Value::Object(got.clone())).unwrap() + "\n";
        if std::env::var_os("LEARN_UPDATE_SHAPES").is_some() {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, &text).unwrap();
        }
        let pinned: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(
            Value::Object(got),
            pinned,
            "the web API's JSON shape changed; if that's intended, regenerate {} (see this test's comment) and update runner/web",
            path.display()
        );
    }

    fn repo_ctx(name: &str) -> Ctx {
        Ctx::discover(Some(std::env::temp_dir().join(format!("learn-views-{name}-{}", std::process::id())))).unwrap()
    }

    fn ids(tracks: Vec<&'static dyn Backend>) -> Vec<&'static str> {
        tracks.iter().map(|b| b.id()).collect()
    }

    #[test]
    fn comparing_starts_with_the_family_or_the_chosen_tracks() {
        let mut ctx = repo_ctx("compare");
        assert_eq!(ids(default_compare_tracks(&ctx, "conditionals", Some("lua"))), ["python", "php", "lua"]);
        assert_eq!(ids(default_compare_tracks(&ctx, "conditionals", Some("rust"))), ["rust", "go"]);
        // No track: every track the concept is on.
        assert_eq!(default_compare_tracks(&ctx, "conditionals", None).len(), 7);
        // QML has no `files` exercise, so TypeScript alone isn't a comparison: all of them.
        assert_eq!(default_compare_tracks(&ctx, "files", Some("typescript")).len(), 6);

        ctx.config.goal = Some("compare".into());
        ctx.config.compare = vec!["rust".into(), "lua".into()];
        assert_eq!(ids(default_compare_tracks(&ctx, "conditionals", Some("go"))), ["rust", "lua"]);
        ctx.config.compare = vec!["qml".into(), "lua".into()];
        // QML has no `files`: one chosen track left, so back to the family.
        assert_eq!(ids(default_compare_tracks(&ctx, "files", Some("rust"))), ["rust", "go"]);
    }

    #[test]
    fn a_path_lists_stages_in_order_with_marks() {
        let mut ctx = repo_ctx("path");
        let go = backends::get("go").unwrap();
        let p = path(&ctx, go);
        assert_eq!(p.stages[0].id, "syntax-primer");
        assert!(p.stages[0].items.iter().all(|i| i.kind == "reading"));
        assert_eq!(p.stages[1].items[0].id, "hello");
        assert!(p.stages.last().unwrap().own, "the track's own stage comes last on Go's path");
        assert_eq!(p.current.as_deref(), Some("hello"));
        assert_eq!((p.done, p.read), (0, 0));
        assert_eq!(p.total, ctx.path_specs("go").len());
        ctx.progress.done.entry("go".into()).or_default().insert("hello".into());
        ctx.progress.read.insert("syntax-types".into());
        let p = path(&ctx, go);
        assert_eq!((p.done, p.read), (1, 1));
        assert_eq!(p.current.as_deref(), Some("leap-year"));
    }

    #[test]
    fn why_notes_cover_each_pair_of_families_shown() {
        let ctx = repo_ctx("why");
        assert!(why_notes(&ctx, &["rust", "go"]).is_empty());
        let one = why_notes(&ctx, &["rust", "lua"]);
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].between, ["compiled".to_string(), "scripting".to_string()]);
        assert_eq!(why_notes(&ctx, &["rust", "typescript", "lua"]).len(), 3);
    }

    #[test]
    fn the_switch_goal_leads_with_the_background() {
        let mut ctx = repo_ctx("switch");
        ctx.config.goal = Some("switch".into());
        assert!(!ctx.note_style().lead, "no background, nothing to lead with");
        ctx.config.from = Some("python".into());
        assert!(ctx.note_style().lead);
        ctx.config.goal = Some("learn".into());
        assert!(!ctx.note_style().lead);
    }

    #[test]
    fn shapes_merge_array_items_and_hide_values() {
        let v = json!({ "a": [{ "x": 1 }, { "y": "s" }], "b": null, "c": [] });
        assert_eq!(shape(&v), json!({ "a": [{ "x": "value", "y": "value" }], "b": "value", "c": [] }));
    }
}
