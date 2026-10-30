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
    /// Set in workspaces that live outside the course repo, so `learn` can be
    /// run from inside them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub course_root: Option<PathBuf>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Progress {
    /// track -> finished exercise slugs
    #[serde(default)]
    pub done: BTreeMap<String, BTreeSet<String>>,
    /// slug -> number of hints revealed
    #[serde(default)]
    pub hints: BTreeMap<String, usize>,
}

pub struct Ctx {
    pub root: PathBuf,
    pub workspace: PathBuf,
    pub state_dir: PathBuf,
    pub external_workspace: bool,
    pub course: Course,
    /// Specs in course order.
    pub specs: Vec<Spec>,
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
                 Run `learn` from inside your polyglot-learn checkout, or from a workspace \
                 that was created with `learn --workspace <dir> start <lang>`."
            );
        };
        if !root.join("course.toml").is_file() {
            bail!(
                "this workspace points at {} but there is no course.toml there.\n\
                 Edit course_root in .learn/config.toml to point at your polyglot-learn checkout.",
                root.display()
            );
        }

        let default_ws = root.join("workspace");
        let workspace = requested.or(found_workspace).unwrap_or_else(|| default_ws.clone());
        let external_workspace = workspace != default_ws;
        let state_dir = if external_workspace { workspace.join(".learn") } else { root.join(".learn") };

        let course = Course::load(&root)?;
        let mut specs = Vec::new();
        for slug in course.slugs() {
            let path = root.join("specs").join(format!("{slug}.json"));
            if !path.exists() {
                bail!("course.toml lists `{slug}` but {} doesn't exist", path.display());
            }
            let spec = Spec::load(&path)?;
            if spec.slug != slug {
                bail!("{} has slug `{}`; it must match the file name", path.display(), spec.slug);
            }
            specs.push(spec);
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

        Ok(Ctx { root, workspace, state_dir, external_workspace, course, specs, config, progress })
    }

    pub fn spec(&self, slug: &str) -> Result<&Spec> {
        self.specs.iter().find(|s| s.slug == slug).ok_or_else(|| {
            let all: Vec<&str> = self.specs.iter().map(|s| s.slug.as_str()).collect();
            Error(format!("no exercise called `{slug}`. Exercises: {}", all.join(", ")))
        })
    }

    pub fn is_done(&self, track: &str, slug: &str) -> bool {
        self.progress.done.get(track).is_some_and(|d| d.contains(slug))
    }

    /// The first unfinished exercise on a track, in course order.
    pub fn current(&self, track: &str) -> Option<&Spec> {
        self.specs.iter().find(|s| !self.is_done(track, &s.slug))
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
                "no track selected yet. Run `learn start <lang>` with one of: {}",
                crate::backends::ids().join(", ")
            ),
        }
    }
}
