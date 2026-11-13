//! Locating the course repo, the learner's workspace and their saved state.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::bail;
use crate::course::Course;
use crate::error::{Context, Error, Result};
use crate::spec::Spec;

pub const WORKSPACE_ENV: &str = "LEARN_WORKSPACE";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// The language the learner already knows ("typescript", "python", ...).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// The active track.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,
    /// What the learner is here for: one of `GOALS` (`learn` when unset).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    /// For the `compare` goal: the tracks compare views start with.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compare: Vec<String>,
    /// Set in workspaces that live outside the course repo, so `learn` can be
    /// run from inside them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub course_root: Option<PathBuf>,
}

/// What `learn init` offers as a goal, and what each changes:
/// - `learn`: nothing extra.
/// - `switch`: notes lead with their "coming from X" section, and the task card says so.
/// - `compare`: compare views start with the tracks in `compare`, and `learn status`
///   shows every track side by side.
pub const GOALS: &[(&str, &str)] = &[
    ("learn", "Learn one language well"),
    ("switch", "Move from a language I know to a new one"),
    ("compare", "Compare languages side by side"),
];

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Progress {
    /// track -> finished exercise slugs
    #[serde(default)]
    pub done: BTreeMap<String, BTreeSet<String>>,
    /// slug -> number of hints revealed
    #[serde(default)]
    pub hints: BTreeMap<String, usize>,
    /// Readings that have been shown (for the whole workspace, not per track:
    /// the page and its table are the same on every track).
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub read: BTreeSet<String>,
}

/// A reading-only page: `concepts/<id>.md` plus `tracks/<lang>/<id>.md`.
#[derive(Debug, Clone)]
pub struct Reading {
    pub id: String,
    pub title: String,
    /// Index of its module in `course.modules`.
    pub module: usize,
}

/// Tracks whose backend runs a project's tests as one codebase.
pub const PROJECT_TRACKS: &[&str] = &["go", "rust", "python", "typescript", "php", "lua"];

pub struct Ctx {
    pub root: PathBuf,
    pub workspace: PathBuf,
    pub state_dir: PathBuf,
    pub external_workspace: bool,
    /// Shared modules from course.toml, then each track's own stages from its path.toml.
    pub course: Course,
    /// Each track's path: which modules it uses, in its order.
    pub paths: BTreeMap<String, Vec<crate::path::Step>>,
    /// Specs in module order (use `track_specs` for one track's order).
    pub specs: Vec<Spec>,
    /// Readings in course order.
    pub readings: Vec<Reading>,
    pub config: Config,
    pub progress: Progress,
}

fn absolute(p: &Path) -> Result<PathBuf> {
    std::path::absolute(p).context(|| format!("can't resolve {}", p.display()))
}

fn read_config(state_dir: &Path) -> Result<Config> {
    let path = state_dir.join("config.toml");
    if !path.exists() {
        return Ok(Config::default());
    }
    let text = std::fs::read_to_string(&path).context(|| format!("can't read {}", path.display()))?;
    toml::from_str(&text).context(|| format!("{} is not valid TOML; fix or delete it", path.display()))
}

impl Ctx {
    /// Find the course root by walking up from the current directory to
    /// `course.toml` (or to an external workspace that records its course).
    pub fn discover(workspace_flag: Option<PathBuf>) -> Result<Ctx> {
        let cwd = std::env::current_dir().context(|| "can't read the current directory".into())?;
        Ctx::discover_from(&cwd, workspace_flag)
    }

    /// `discover`, walking up from `start` instead of the current directory
    /// (`learn lsp` starts from the file being edited).
    pub fn discover_from(start: &Path, workspace_flag: Option<PathBuf>) -> Result<Ctx> {
        let cwd = start;
        let requested = match workspace_flag.or_else(|| std::env::var_os(WORKSPACE_ENV).map(PathBuf::from)) {
            Some(p) => Some(absolute(&p)?),
            None => None,
        };

        let mut root = None;
        let mut found_workspace = None;
        for dir in cwd.ancestors() {
            if dir.join("course.toml").is_file() {
                root = Some(dir.to_path_buf());
                break;
            }
            if dir.join(".learn").join("config.toml").is_file() {
                if let Some(course_root) = read_config(&dir.join(".learn"))?.course_root {
                    root = Some(course_root);
                    found_workspace = Some(dir.to_path_buf());
                    break;
                }
            }
        }
        if root.is_none() {
            if let Some(ws) = &requested {
                root = read_config(&ws.join(".learn"))?.course_root;
            }
        }
        let Some(root) = root else {
            bail!(
                "couldn't find course.toml in this folder or any parent.\n\
                 Run `learn` from inside your polysyntax checkout, or from a workspace \
                 that was created with `learn --workspace <dir> start <lang>`."
            );
        };
        if !root.join("course.toml").is_file() {
            bail!(
                "this workspace points at {} but there is no course.toml there.\n\
                 Edit course_root in .learn/config.toml to point at your polysyntax checkout.",
                root.display()
            );
        }

        let default_ws = root.join("workspace");
        let workspace = requested.or(found_workspace).unwrap_or_else(|| default_ws.clone());
        let external_workspace = workspace != default_ws;
        let state_dir = if external_workspace { workspace.join(".learn") } else { root.join(".learn") };

        let mut course = Course::load(&root)?;
        let paths = crate::path::load_all(&root, &mut course, &crate::backends::ids())?;
        // Own stages joined the module list: no exercise may now appear twice.
        course.validate()?;
        let mut specs = Vec::new();
        for (mi, m) in course.modules.iter().enumerate() {
            if let Some(track) = &m.track {
                if crate::backends::get(track).ok().map(|b| b.id()) != Some(track.as_str()) {
                    bail!("course.toml: module `{}` has unknown track `{track}`", m.id);
                }
            }
            if let Some(tracks) = &m.tracks {
                if m.track.is_some() {
                    bail!("course.toml: module `{}` has both `track` and `tracks`; a track-only module needs just `track`", m.id);
                }
                if tracks.is_empty() {
                    bail!("course.toml: module `{}` has an empty `tracks` list; leave it out for every track", m.id);
                }
                for track in tracks {
                    if crate::backends::get(track).ok().map(|b| b.id()) != Some(track.as_str()) {
                        bail!("course.toml: module `{}` lists unknown track `{track}`", m.id);
                    }
                }
            }
            for slug in &m.exercises {
                let spec = match &m.track {
                    Some(track) => {
                        let dir = root.join("tracks").join(track).join("extras").join(slug);
                        if !dir.join("exercise.toml").is_file() {
                            bail!("course.toml lists `{slug}` but {} doesn't exist", dir.join("exercise.toml").display());
                        }
                        Spec::load_native(&dir, track, slug)?
                    }
                    None => {
                        let path = root.join("specs").join(format!("{slug}.json"));
                        if !path.exists() {
                            bail!("course.toml lists `{slug}` but {} doesn't exist", path.display());
                        }
                        let mut spec = Spec::load(&path)?;
                        if spec.slug != *slug {
                            bail!("{} has slug `{}`; it must match the file name", path.display(), spec.slug);
                        }
                        // On exactly the tracks whose path includes it.
                        spec.tracks = Some(
                            paths
                                .iter()
                                .filter(|(_, steps)| steps.iter().any(|st| st.module == mi && !st.skip.contains(slug)))
                                .map(|(t, _)| t.clone())
                                .collect(),
                        );
                        // Every track the exercise is on must be able to run it.
                        for b in crate::backends::all().iter().filter(|b| spec.on_track(b.id())) {
                            if spec.resources && b.resource_module(&spec).is_none() {
                                bail!(
                                    "{slug} uses the resource module, which the {} track doesn't have; leave module `{}` out of that track (its path.toml, or `tracks = [...]` in course.toml)",
                                    b.name(),
                                    m.id
                                );
                            }
                            if spec.uses_paths() && b.path_literal("x").is_none() {
                                bail!(
                                    "{slug} reads files, which the {} track can't do; leave module `{}` out of that track (its path.toml, or `tracks = [...]` in course.toml)",
                                    b.name(),
                                    m.id
                                );
                            }
                        }
                        spec
                    }
                };
                specs.push(spec);
            }
        }

        // Stage folders: the path's stages that have exercises on the track,
        // numbered in order, so a file tree reads like the path.
        for (track, steps) in &paths {
            let mut n = 0;
            for step in steps {
                let m = &course.modules[step.module];
                let slugs: Vec<&String> = m.exercises.iter().filter(|s| !step.skip.contains(*s)).collect();
                if slugs.is_empty() {
                    continue;
                }
                n += 1;
                let stage = format!("{n:02}-{}", m.id);
                for slug in slugs {
                    if let Some(spec) = specs.iter_mut().find(|s| s.slug == *slug) {
                        spec.stages.insert(track.clone(), stage.clone());
                    }
                }
            }
        }

        // Projects: each stage knows its place, and its cases are every stage's
        // so far, numbered across the project (tests are named `cNN_…` that way).
        for m in course.modules.iter().filter(|m| m.project) {
            let track = m.track.as_deref().unwrap_or_default();
            if !PROJECT_TRACKS.contains(&track) {
                bail!("project `{}`: the {track} track can't run projects yet (only {})", m.id, PROJECT_TRACKS.join(", "));
            }
            let mut so_far: Vec<crate::spec::Case> = Vec::new();
            for (k, slug) in m.exercises.iter().enumerate() {
                let Some(spec) = specs.iter_mut().find(|s| s.slug == *slug) else { continue };
                if spec.native.is_none() {
                    bail!("project `{}`: stage `{slug}` must be the track's own exercise (tracks/<lang>/extras/{slug}/)", m.id);
                }
                for mut c in std::mem::take(&mut spec.main.cases) {
                    c.index = so_far.len();
                    so_far.push(c);
                }
                if so_far.len() > 99 {
                    bail!("project `{}` has more than 99 cases in all; test names only number two digits", m.id);
                }
                spec.main.cases = so_far.clone();
                spec.project = Some(crate::spec::ProjectStage { id: m.id.clone(), title: m.title.clone(), number: k + 1, stages: m.exercises.clone() });
            }
        }

        let mut readings = Vec::new();
        for (i, m) in course.modules.iter().enumerate() {
            for id in &m.readings {
                if specs.iter().any(|s| s.concept == *id) {
                    bail!("course.toml: reading `{id}` has the same name as an exercise concept; give it another id");
                }
                let path = root.join("concepts").join(format!("{id}.md"));
                let text = std::fs::read_to_string(&path).context(|| format!("can't read {} (reading `{id}`)", path.display()))?;
                let title = crate::notes::title_of(&text).unwrap_or_else(|| id.clone());
                readings.push(Reading { id: id.clone(), title, module: i });
            }
        }

        let config = read_config(&state_dir)?;
        let progress_path = state_dir.join("progress.json");
        let progress = if progress_path.exists() {
            let text = std::fs::read_to_string(&progress_path)
                .context(|| format!("can't read {}", progress_path.display()))?;
            serde_json::from_str(&text)
                .context(|| format!("{} is damaged; fix or delete it", progress_path.display()))?
        } else {
            Progress::default()
        };

        Ok(Ctx { root, workspace, state_dir, external_workspace, course, paths, specs, readings, config, progress })
    }

    pub fn spec(&self, slug: &str) -> Result<&Spec> {
        self.specs.iter().find(|s| s.slug == slug).ok_or_else(|| {
            let all: Vec<&str> = self.specs.iter().map(|s| s.slug.as_str()).collect();
            Error(format!("no exercise called `{slug}`.
Exercises: {}", all.join(", ")))
        })
    }

    /// An exercise on a given track (shared, or that track's own).
    pub fn spec_on(&self, track: &str, slug: &str) -> Result<&Spec> {
        let Ok(spec) = self.spec(slug) else {
            let all: Vec<&str> = self.path_specs(track).iter().map(|s| s.slug.as_str()).collect();
            bail!("no exercise called `{slug}` on the {track} track.
Exercises: {}", all.join(", "));
        };
        match &spec.native {
            Some(n) if n.track != track => bail!("`{slug}` is part of the {} track only.
Switch with `learn start {}`, or pick another exercise.", n.track, n.track),
            _ if !spec.on_track(track) => bail!(
                "`{slug}` isn't on the {track} track; it's on {}.",
                spec.tracks.as_deref().unwrap_or_default().join(", ")
            ),
            _ => Ok(spec),
        }
    }

    /// Every exercise on a track, in its path's order (owned: backends take a slice).
    pub fn track_specs(&self, track: &str) -> Vec<Spec> {
        self.path_specs(track).into_iter().cloned().collect()
    }

    /// The exercises whose files belong in the workspace now: every one, except
    /// a project's stages that the learner hasn't reached yet. Pass these to
    /// `backends::generate` and `check::run`, which write every spec they get.
    pub fn workspace_specs(&self, track: &str) -> Vec<Spec> {
        self.path_specs(track).into_iter().filter(|s| self.reached(track, s)).cloned().collect()
    }

    /// The modules on a track's path, in order, with what it skips of each.
    pub fn path(&self, track: &str) -> &[crate::path::Step] {
        self.paths.get(track).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Whether a track's path uses module `i` of `course.modules`.
    pub fn module_on_track(&self, i: usize, track: &str) -> bool {
        self.path(track).iter().any(|s| s.module == i)
    }

    /// A module's exercises on a track (its skips left out).
    pub fn module_exercises<'a>(&'a self, step: &'a crate::path::Step) -> impl Iterator<Item = &'a String> + 'a {
        self.course.modules[step.module].exercises.iter().filter(|s| !step.skip.contains(*s))
    }

    /// The exercises on a track, in its path's order.
    pub fn path_specs(&self, track: &str) -> Vec<&Spec> {
        self.path(track)
            .iter()
            .flat_map(|step| self.module_exercises(step))
            .filter_map(|slug| self.specs.iter().find(|s| s.slug == *slug))
            .collect()
    }

    /// An exercise's folder in the workspace: `<track>/NN-stage/<slug>`.
    pub fn exercise_dir(&self, track: &str, slug: &str) -> PathBuf {
        let rel = self.specs.iter().find(|s| s.slug == slug).map(|s| s.dir(track)).unwrap_or_else(|| PathBuf::from(slug));
        self.workspace.join(track).join(rel)
    }

    /// The exercise on a track that a file belongs to; `rel` is relative to
    /// the track's folder. The folder itself isn't a file of it. A project's
    /// files belong to its current stage.
    pub fn exercise_at(&self, track: &str, rel: &Path) -> Option<&Spec> {
        let spec = self.path_specs(track).into_iter().find(|s| {
            let dir = s.dir(track);
            rel.starts_with(&dir) && rel != dir
        })?;
        match &spec.project {
            Some(p) => self.project_stage(track, &p.id),
            None => Some(spec),
        }
    }

    /// A project's current stage: the first one not done, or the last.
    pub fn project_stage(&self, track: &str, project: &str) -> Option<&Spec> {
        let stages: Vec<&Spec> = self.path_specs(track).into_iter().filter(|s| s.project.as_ref().is_some_and(|p| p.id == project)).collect();
        stages.iter().find(|s| !self.is_done(track, &s.slug)).or(stages.last()).copied()
    }

    /// Whether an exercise's files belong in the workspace yet: always, except
    /// for a project's later stages, which arrive one by one as the stage
    /// before passes.
    pub fn reached(&self, track: &str, spec: &Spec) -> bool {
        match &spec.project {
            Some(p) if p.number > 1 => self.is_done(track, &p.stages[p.number - 2]),
            _ => true,
        }
    }

    pub fn is_done(&self, track: &str, slug: &str) -> bool {
        self.progress.done.get(track).is_some_and(|d| d.contains(slug))
    }

    /// The first unfinished exercise on a track, in its path's order.
    pub fn current(&self, track: &str) -> Option<&Spec> {
        self.path_specs(track).into_iter().find(|s| !self.is_done(track, &s.slug))
    }

    /// The learner's background for "coming from X" notes; `none` means neutral notes.
    pub fn background(&self) -> Option<&str> {
        self.config.from.as_deref().filter(|f| *f != "none")
    }

    /// The learner's goal, `learn` when none was chosen.
    pub fn goal(&self) -> &str {
        self.config.goal.as_deref().unwrap_or("learn")
    }

    /// Check and save a goal; `compare` (tracks for the compare goal) needs two or
    /// more known tracks, and is dropped for the other goals.
    pub fn set_goal(&mut self, goal: &str, compare: &[String]) -> Result<()> {
        if !GOALS.iter().any(|(id, _)| *id == goal) {
            let ids: Vec<&str> = GOALS.iter().map(|(id, _)| *id).collect();
            bail!("unknown goal `{goal}`.\nChoose one of: {}", ids.join(", "));
        }
        let mut tracks = Vec::new();
        for t in compare {
            tracks.push(crate::backends::get(t)?.id().to_string());
        }
        if goal == "compare" && tracks.len() == 1 {
            bail!("name at least two tracks to compare, e.g. --compare rust,go");
        }
        self.config.goal = Some(goal.to_string());
        self.config.compare = if goal == "compare" { tracks } else { Vec::new() };
        self.save_config()
    }

    /// How to render notes: the background, leading with it for the `switch` goal.
    pub fn note_style(&self) -> crate::notes::Style<'_> {
        crate::notes::Style { from: self.background(), lead: self.goal() == "switch" && self.background().is_some() }
    }

    /// The readings a learner on `track` (or on no track yet) works through.
    pub fn readings_for(&self, track: Option<&str>) -> Vec<&Reading> {
        match track {
            Some(t) => self.readings_on(t),
            None => self.readings.iter().collect(),
        }
    }

    /// Readings on a track, in course order.
    pub fn readings_on(&self, track: &str) -> Vec<&Reading> {
        self.readings.iter().filter(|r| self.module_on_track(r.module, track)).collect()
    }

    /// A reading by id, or by its id without the `syntax-` style prefix
    /// (`variables` for `syntax-variables`).
    pub fn reading(&self, name: &str) -> Option<&Reading> {
        self.readings
            .iter()
            .find(|r| r.id == name)
            .or_else(|| self.readings.iter().find(|r| r.id.split_once('-').is_some_and(|(_, rest)| rest == name)))
    }

    pub fn is_read(&self, id: &str) -> bool {
        self.progress.read.contains(id)
    }

    /// Mark a reading as read and save, if it wasn't already.
    pub fn mark_read(&mut self, id: &str) -> Result<()> {
        if self.progress.read.insert(id.to_string()) {
            self.save_progress()?;
        }
        Ok(())
    }

    /// Nothing done on the track and nothing read: show the primer hint.
    pub fn is_new(&self, track: &str) -> bool {
        self.progress.read.is_empty()
            && self.progress.done.get(track).is_none_or(|d| d.is_empty())
            && !self.readings_on(track).is_empty()
    }

    pub fn save_config(&mut self) -> Result<()> {
        if self.external_workspace {
            self.config.course_root = Some(self.root.clone());
        }
        let text = toml::to_string(&self.config).map_err(|e| Error(e.to_string()))?;
        self.write_state("config.toml", &text)
    }

    pub fn save_progress(&self) -> Result<()> {
        let text = serde_json::to_string_pretty(&self.progress).map_err(|e| Error(e.to_string()))?;
        self.write_state("progress.json", &text)
    }

    fn write_state(&self, name: &str, text: &str) -> Result<()> {
        std::fs::create_dir_all(&self.state_dir).context(|| format!("can't create {}", self.state_dir.display()))?;
        let path = self.state_dir.join(name);
        std::fs::write(&path, text).context(|| format!("can't write {}", path.display()))
    }

    /// The active track, or an error that tells the learner how to pick one.
    pub fn active_track(&self) -> Result<String> {
        match &self.config.track {
            Some(t) => Ok(t.clone()),
            None => bail!(
                "no track selected yet.
Run `learn start <lang>` with one of: {}",
                crate::backends::ids().join(", ")
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn old_progress_files_load_without_read_marks() {
        let p: Progress = serde_json::from_str(r#"{"done":{"rust":["hello"]},"hints":{}}"#).unwrap();
        assert!(p.read.is_empty());
        let mut p = p;
        p.read.insert("syntax-types".into());
        let back: Progress = serde_json::from_str(&serde_json::to_string(&p).unwrap()).unwrap();
        assert!(back.read.contains("syntax-types"));
    }
}
