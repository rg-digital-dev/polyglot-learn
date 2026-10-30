//! `course.toml` (module order) and `tracks/<lang>/track.toml` (links, install hints).

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

use crate::bail;
use crate::error::{Context, Result};

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // title/id are shown by the web UI
pub struct Course {
    #[serde(default)]
    pub title: String,
    #[serde(rename = "module")]
    pub modules: Vec<Module>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Module {
    pub id: String,
    pub title: String,
    pub exercises: Vec<String>,
}

impl Course {
    pub fn load(root: &Path) -> Result<Course> {
        let path = root.join("course.toml");
        let text = std::fs::read_to_string(&path).context(|| format!("can't read {}", path.display()))?;
        let course: Course = toml::from_str(&text).context(|| format!("invalid {}", path.display()))?;
        let mut seen = std::collections::HashSet::new();
        for slug in course.slugs() {
            if !seen.insert(slug) {
                bail!("course.toml lists `{slug}` twice");
            }
        }
        Ok(course)
    }

    pub fn slugs(&self) -> impl Iterator<Item = &str> {
        self.modules.iter().flat_map(|m| m.exercises.iter().map(String::as_str))
    }
}

#[derive(Debug, Default, Deserialize)]
#[allow(dead_code)]
pub struct TrackMeta {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub lings: Option<Lings>,
    #[serde(default)]
    pub install: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub links: BTreeMap<String, Vec<Link>>,
}

#[derive(Debug, Deserialize)]
pub struct Lings {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Link {
    pub kind: String,
    pub title: String,
    pub url: String,
}

impl TrackMeta {
    /// Missing track.toml is not an error: the track just has no links yet.
    pub fn load(root: &Path, track: &str) -> Result<TrackMeta> {
        let path = root.join("tracks").join(track).join("track.toml");
        if !path.exists() {
            return Ok(TrackMeta::default());
        }
        let text = std::fs::read_to_string(&path).context(|| format!("can't read {}", path.display()))?;
        toml::from_str(&text).context(|| format!("invalid {}", path.display()))
    }

    pub fn links_for(&self, concept: &str) -> &[Link] {
        self.links.get(concept).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Label for a link kind, using the track's *lings name when there is one.
    pub fn kind_label(&self, kind: &str) -> String {
        match kind {
            "docs" => "Docs".into(),
            "lings" => self.lings.as_ref().map_or("Exercises".into(), |l| l.name.clone()),
            "exercism" => "Exercism".into(),
            other => other.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repo_course_references_existing_specs() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let course = Course::load(&root).unwrap();
        for slug in course.slugs() {
            assert!(root.join("specs").join(format!("{slug}.json")).exists(), "missing spec {slug}");
        }
    }

    #[test]
    fn repo_track_tomls_parse() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        for track in ["rust", "typescript", "php", "lua", "go", "qml"] {
            let meta = TrackMeta::load(&root, track).unwrap();
            for links in meta.links.values() {
                for l in links {
                    assert!(["docs", "lings", "exercism"].contains(&l.kind.as_str()), "{track}: kind {}", l.kind);
                    assert!(l.url.starts_with("https://"), "{track}: {}", l.url);
                }
            }
        }
    }
}
