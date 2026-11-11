//! Per-track paths: `tracks/<lang>/path.toml` orders a track's stages. A stage
//! either uses a shared module from `course.toml` (optionally skipping some of
//! its exercises) or is the track's own stage, with exercises written for that
//! language in `tracks/<lang>/extras/`.
//!
//! ```toml
//! [[stage]]
//! use = "foundations"
//! skip = ["word-count"]
//!
//! [[stage]]
//! id = "deeper-go"
//! title = "Deeper Go"
//! exercises = ["embedded-logger", "pipeline"]
//!
//! [[stage]]
//! id = "url-shortener"
//! title = "Project: URL shortener"
//! project = true      # one codebase, one folder; each exercise is a stage
//! exercises = ["shortener-store", "shortener-handlers"]
//! ```
//!
//! A track without a `path.toml` gets every shared module that's on it, in
//! course order, followed by any track-only modules `course.toml` still lists
//! for it (the 0.3 layout).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::Deserialize;

use crate::bail;
use crate::course::{Course, Module};
use crate::error::{Context, Result};

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct PathFile {
    #[serde(default)]
    stage: Vec<StageDef>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StageDef {
    /// A shared module's id.
    #[serde(rename = "use")]
    uses: Option<String>,
    /// Exercises of the shared module this track leaves out.
    #[serde(default)]
    skip: Vec<String>,
    /// For the track's own stage.
    id: Option<String>,
    title: Option<String>,
    #[serde(default)]
    exercises: Vec<String>,
    /// The stage is a project: its exercises build one codebase in one folder.
    #[serde(default)]
    project: bool,
}

/// One stage of a track's path: a module (shared, or the track's own) and the
/// exercises of it the track leaves out.
#[derive(Debug, Clone)]
pub struct Step {
    /// Index into `course.modules`.
    pub module: usize,
    pub skip: BTreeSet<String>,
}

/// Read every track's path. The tracks' own stages are added to
/// `course.modules` (with `track` set), so the rest of the engine sees one list
/// of modules; the returned paths say which of them each track uses, in order.
pub fn load_all(root: &Path, course: &mut Course, tracks: &[&str]) -> Result<BTreeMap<String, Vec<Step>>> {
    let mut paths = BTreeMap::new();
    for &track in tracks {
        let file = root.join("tracks").join(track).join("path.toml");
        let steps = if file.is_file() {
            let text = std::fs::read_to_string(&file).context(|| format!("can't read {}", file.display()))?;
            let def: PathFile = toml::from_str(&text).context(|| format!("invalid {}", file.display()))?;
            resolve(course, track, def, &file.display().to_string())?
        } else {
            default_path(course, track)
        };
        paths.insert(track.to_string(), steps);
    }
    Ok(paths)
}

fn default_path(course: &Course, track: &str) -> Vec<Step> {
    let shared = course.modules.iter().enumerate().filter(|(_, m)| m.track.is_none() && m.on_track(track));
    let own = course.modules.iter().enumerate().filter(|(_, m)| m.track.as_deref() == Some(track));
    shared.chain(own).map(|(module, _)| Step { module, skip: BTreeSet::new() }).collect()
}

fn resolve(course: &mut Course, track: &str, def: PathFile, file: &str) -> Result<Vec<Step>> {
    let mut steps = Vec::new();
    let mut seen = BTreeSet::new();
    for (n, stage) in def.stage.into_iter().enumerate() {
        let at = format!("{file}, stage {}", n + 1);
        let module = match (&stage.uses, &stage.id) {
            (Some(_), Some(_)) => bail!("{at}: has both `use` and `id`; a stage uses a shared module or is the track's own"),
            (None, None) => bail!("{at}: needs `use = \"<module>\"` or an `id` and `title` of its own"),
            (Some(name), None) => {
                if stage.title.is_some() || !stage.exercises.is_empty() || stage.project {
                    bail!("{at}: a stage that uses `{name}` takes its title and exercises from course.toml");
                }
                let Some(i) = course.modules.iter().position(|m| m.id == *name && m.track.is_none()) else {
                    bail!("{at}: uses `{name}`, which isn't a shared module in course.toml");
                };
                if !course.modules[i].on_track(track) {
                    bail!("{at}: uses `{name}`, but course.toml's `tracks` for it leaves out {track}");
                }
                for s in &stage.skip {
                    if !course.modules[i].exercises.contains(s) {
                        bail!("{at}: skips `{s}`, which isn't in module `{name}`");
                    }
                }
                i
            }
            (None, Some(id)) => {
                if !stage.skip.is_empty() {
                    bail!("{at}: `skip` only applies to a shared module");
                }
                let Some(title) = stage.title.clone() else { bail!("{at}: stage `{id}` needs a `title`") };
                if stage.exercises.is_empty() {
                    bail!("{at}: stage `{id}` has no exercises");
                }
                if course.modules.iter().any(|m| m.id == *id) {
                    bail!("{at}: stage id `{id}` is already used by another module or stage");
                }
                course.modules.push(Module {
                    id: id.clone(),
                    title,
                    track: Some(track.to_string()),
                    tracks: None,
                    exercises: stage.exercises.clone(),
                    readings: Vec::new(),
                    project: stage.project,
                });
                course.modules.len() - 1
            }
        };
        for slug in course.modules[module].exercises.iter().filter(|s| !stage.skip.contains(*s)) {
            if !seen.insert(slug.clone()) {
                bail!("{at}: `{slug}` is already on the {track} path");
            }
        }
        steps.push(Step { module, skip: stage.skip.into_iter().collect() });
    }
    Ok(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn course() -> Course {
        toml::from_str(
            "[[module]]\nid = \"basics\"\ntitle = \"Basics\"\nexercises = [\"a\", \"b\"]\n\
             [[module]]\nid = \"files\"\ntitle = \"Files\"\ntracks = [\"rust\"]\nexercises = [\"f\"]\n\
             [[module]]\nid = \"deeper-go\"\ntitle = \"Deeper Go\"\ntrack = \"go\"\nexercises = [\"g\"]\n",
        )
        .unwrap()
    }

    fn resolve_text(course: &mut Course, track: &str, text: &str) -> Result<Vec<Step>> {
        resolve(course, track, toml::from_str(text).unwrap(), "path.toml")
    }

    #[test]
    fn without_a_path_file_a_track_gets_its_modules_in_course_order() {
        let c = course();
        let ids = |steps: Vec<Step>| steps.iter().map(|s| c.modules[s.module].id.clone()).collect::<Vec<_>>();
        assert_eq!(ids(default_path(&c, "go")), ["basics", "deeper-go"]);
        assert_eq!(ids(default_path(&c, "rust")), ["basics", "files"]);
    }

    #[test]
    fn own_stages_become_modules_of_that_track() {
        let mut c = course();
        let steps = resolve_text(
            &mut c,
            "lua",
            "[[stage]]\nid = \"lua-games\"\ntitle = \"Game Scripting\"\nexercises = [\"loop\"]\n[[stage]]\nuse = \"basics\"\nskip = [\"b\"]\n",
        )
        .unwrap();
        assert_eq!(c.modules[steps[0].module].track.as_deref(), Some("lua"));
        assert_eq!(c.modules[steps[1].module].id, "basics");
        assert!(steps[1].skip.contains("b"));
    }

    #[test]
    fn mistakes_are_reported() {
        let bad = [
            ("[[stage]]\nuse = \"nope\"\n", "isn't a shared module"),
            ("[[stage]]\nuse = \"files\"\n", "leaves out lua"),
            ("[[stage]]\nuse = \"basics\"\nskip = [\"z\"]\n", "skips `z`"),
            ("[[stage]]\nuse = \"basics\"\n[[stage]]\nuse = \"basics\"\n", "already on the lua path"),
            ("[[stage]]\nid = \"x\"\nexercises = [\"q\"]\n", "needs a `title`"),
            ("[[stage]]\nid = \"basics\"\ntitle = \"B\"\nexercises = [\"q\"]\n", "already used"),
            ("[[stage]]\ntitle = \"T\"\n", "needs `use"),
        ];
        for (text, want) in bad {
            let err = resolve_text(&mut course(), "lua", text).unwrap_err().0;
            assert!(err.contains(want), "{text:?}: {err}");
        }
    }
}
