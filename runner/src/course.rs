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
    /// Set for a track-only module: its exercises live in
    /// `tracks/<track>/extras/<slug>/` instead of `specs/`.
    #[serde(default)]
    pub track: Option<String>,
    /// For a shared module that suits only some languages: the tracks it's on.
    /// Not together with `track`; leave both out for every track.
    #[serde(default)]
    pub tracks: Option<Vec<String>>,
    #[serde(default)]
    pub exercises: Vec<String>,
    /// Reading-only pages (`concepts/<id>.md` plus a note per track), shown
    /// before the module's exercises. No specs, tests or progress towards "done".
    #[serde(default)]
    pub readings: Vec<String>,
}

impl Module {
    pub fn on_track(&self, track: &str) -> bool {
        self.track.as_deref().is_none_or(|t| t == track) && self.tracks.as_ref().is_none_or(|ts| ts.iter().any(|t| t == track))
    }
}

impl Course {
    pub fn load(root: &Path) -> Result<Course> {
        let path = root.join("course.toml");
        let text = std::fs::read_to_string(&path).context(|| format!("can't read {}", path.display()))?;
        let course: Course = toml::from_str(&text).context(|| format!("invalid {}", path.display()))?;
        course.validate()?;
        Ok(course)
    }

    fn validate(&self) -> Result<()> {
        let mut seen = std::collections::HashSet::new();
        for id in self.slugs().chain(self.reading_ids()) {
            if !seen.insert(id) {
                bail!("course.toml lists `{id}` twice");
            }
        }
        Ok(())
    }

    /// Reading ids in course order.
    pub fn reading_ids(&self) -> impl Iterator<Item = &str> {
        self.modules.iter().flat_map(|m| m.readings.iter().map(String::as_str))
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

    fn parse(text: &str) -> Result<Course> {
        let course: Course = toml::from_str(text).map_err(|e| crate::error::Error(e.to_string()))?;
        course.validate()?;
        Ok(course)
    }

    #[test]
    fn a_module_may_have_only_readings() {
        let c = parse("[[module]]
id = \"p\"
title = \"P\"
readings = [\"syntax-a\", \"syntax-b\"]
").unwrap();
        assert!(c.modules[0].exercises.is_empty());
        assert_eq!(c.reading_ids().collect::<Vec<_>>(), ["syntax-a", "syntax-b"]);
        assert_eq!(c.slugs().count(), 0);
    }

    #[test]
    fn a_reading_may_not_repeat_or_reuse_a_slug() {
        let twice = "[[module]]
id = \"p\"
title = \"P\"
readings = [\"a\", \"a\"]
";
        assert!(parse(twice).is_err());
        let clash = "[[module]]
id = \"p\"
title = \"P\"
readings = [\"hello\"]
exercises = [\"hello\"]
";
        assert!(parse(clash).is_err());
    }

    /// Every reading has a titled concept page and a note on every track its module is on.
    #[test]
    fn repo_course_readings_exist() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let course = Course::load(&root).unwrap();
        for m in &course.modules {
            for id in &m.readings {
                let page = std::fs::read_to_string(root.join("concepts").join(format!("{id}.md")))
                    .unwrap_or_else(|_| panic!("missing concepts/{id}.md"));
                assert!(crate::notes::title_of(&page).is_some(), "concepts/{id}.md has no `# ` title");
                for t in crate::backends::ids().into_iter().filter(|t| m.on_track(t)) {
                    let note = root.join("tracks").join(t).join(format!("{id}.md"));
                    assert!(note.exists(), "missing {}", note.display());
                }
            }
        }
    }

    #[test]
    fn repo_course_references_existing_specs() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let course = Course::load(&root).unwrap();
        for m in &course.modules {
            for slug in &m.exercises {
                let path = match &m.track {
                    Some(t) => root.join("tracks").join(t).join("extras").join(slug).join("exercise.toml"),
                    None => root.join("specs").join(format!("{slug}.json")),
                };
                assert!(path.exists(), "missing {}", path.display());
            }
        }
    }

    /// Every case listed in a native exercise's `exercise.toml` has a test with
    /// its `cNN_` name, and every test file name says which case it checks.
    #[test]
    fn repo_native_exercises_name_their_cases() {
        fn read_all(dir: &Path, skip: &[std::path::PathBuf], rel: &Path, out: &mut String) {
            for e in std::fs::read_dir(dir.join(rel)).unwrap().flatten() {
                let r = rel.join(e.file_name());
                if e.path().is_dir() {
                    read_all(dir, skip, &r, out);
                } else if !skip.contains(&r) {
                    out.push_str(&std::fs::read_to_string(e.path()).unwrap());
                }
            }
        }
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let course = Course::load(&root).unwrap();
        for m in &course.modules {
            let Some(track) = &m.track else { continue };
            for slug in &m.exercises {
                let dir = root.join("tracks").join(track).join("extras").join(slug);
                let spec = crate::spec::Spec::load_native(&dir, track, slug).unwrap();
                let native = spec.native.as_ref().unwrap();
                let mut tests = String::new();
                read_all(&native.files, &native.stubs, Path::new(""), &mut tests);
                for (_, c) in spec.cases() {
                    let name = crate::naming::case_name(c.index, &c.description);
                    if c.index == 0 && !native.typecheck.is_empty() {
                        // Filled in by the type checker, not by a test.
                        assert!(!tests.contains(&name), "{track}/{slug}: {name} comes from the type checker; remove the test");
                        continue;
                    }
                    assert!(tests.contains(&name), "{track}/{slug}: no test named {name}");
                }
                let next = crate::naming::case_name(spec.cases().len(), "");
                assert!(!tests.contains(&format!("{next}_")), "{track}/{slug}: a test {next}_… isn't listed in cases");
            }
        }
    }

    /// `track.toml`'s `[install]` (what `learn doctor` prints) is the source of
    /// truth; each README's Install section must show the same commands.
    #[test]
    fn repo_readme_install_steps_match_track_toml() {
        const INSTALLERS: &[&str] = &[
            "sudo ", "brew ", "winget ", "curl ", "pip ", "aqt ", "fnm ", "rustup ", "rustup-init", "xcode-select", "source ",
            "export ",
        ];
        // A command without its trailing `   # comment`.
        let bare = |c: &str| c.split("   #").next().unwrap().trim().to_string();
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let mut problems = Vec::new();
        for track in ["rust", "typescript", "python", "php", "lua", "go", "qml"] {
            let meta = TrackMeta::load(&root, track).unwrap();
            let readme = std::fs::read_to_string(root.join("tracks").join(track).join("README.md")).unwrap();
            let start = readme.find("## Install").unwrap_or_else(|| panic!("{track}: README has no ## Install"));
            let section = &readme[start..];
            let section = &section[..section[3..].find("\n## ").map_or(section.len(), |i| i + 3)];
            // Commands the README shows: lines of fenced blocks, and inline code
            // spans (split at `&&`).
            let mut shown: Vec<String> = Vec::new();
            for (i, part) in section.split("```").enumerate() {
                if i % 2 == 1 {
                    shown.extend(part.lines().skip(1).map(bare).filter(|l| !l.is_empty()));
                } else {
                    for span in part.split('`').skip(1).step_by(2) {
                        shown.extend(span.split(" && ").map(bare));
                    }
                }
            }
            let lines: Vec<&String> = meta.install.values().flatten().collect();
            let commands: Vec<String> = lines.iter().filter(|c| !c.starts_with('#')).map(|c| bare(c)).collect();
            // Alternatives may be mentioned in the README if a `#` note in track.toml names them.
            let notes: Vec<&String> = lines.iter().copied().filter(|c| c.starts_with('#')).collect();
            for c in &commands {
                if !shown.contains(c) {
                    problems.push(format!("{track}: README doesn't show `{c}`"));
                }
            }
            for s in shown.iter().filter(|s| INSTALLERS.iter().any(|i| s.starts_with(i))) {
                if !commands.contains(s) && !notes.iter().any(|n| n.contains(s.as_str())) {
                    problems.push(format!("{track}: README shows `{s}`, which track.toml doesn't have"));
                }
            }
        }
        assert!(problems.is_empty(), "\n{}", problems.join("\n"));
    }

    #[test]
    fn repo_track_tomls_parse() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        for track in ["rust", "typescript", "python", "php", "lua", "go", "qml"] {
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
