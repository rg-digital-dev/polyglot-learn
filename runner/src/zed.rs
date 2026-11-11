//! The Zed extension (`editors/zed`), built into release binaries so learners
//! get it from `learn` instead of compiling it or finding it in a registry.
//! Zed loads any folder in `extensions/installed` that holds an
//! `extension.toml` and a built `extension.wasm`.

use std::path::{Path, PathBuf};

use crate::error::{Context, Result};
use crate::toolchain;

/// The extension's id in its extension.toml: its folder name in Zed.
pub const ID: &str = "polysyntax-lsp";
const WASM: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/zed.wasm"));
const MANIFEST: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/zed-extension.toml"));

/// Whether this build carries the extension (release builds do).
pub fn bundled() -> bool {
    !WASM.is_empty() && !MANIFEST.is_empty()
}

/// Zed's data folder, where `extensions/installed` lives.
pub fn data_dir() -> Option<PathBuf> {
    let env = |k: &str| std::env::var_os(k).filter(|v| !v.is_empty()).map(PathBuf::from);
    if cfg!(windows) {
        env("LOCALAPPDATA").map(|d| d.join("Zed"))
    } else if cfg!(target_os = "macos") {
        env("HOME").map(|h| h.join("Library").join("Application Support").join("Zed"))
    } else {
        env("XDG_DATA_HOME").or_else(|| env("HOME").map(|h| h.join(".local").join("share"))).map(|d| d.join("zed"))
    }
}

/// Zed is installed: on PATH, or it has made its data folder.
pub fn found() -> bool {
    toolchain::which("zed").is_some() || data_dir().is_some_and(|d| d.is_dir())
}

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Missing,
    /// A dev extension (a link to a source folder): yours, left alone.
    Dev,
    Current,
    Outdated,
}

/// The extension's folder under a Zed data folder.
pub fn folder(data: &Path) -> PathBuf {
    data.join("extensions").join("installed").join(ID)
}

pub fn state(data: &Path) -> State {
    state_of(&folder(data), WASM, MANIFEST)
}

fn state_of(dir: &Path, wasm: &[u8], manifest: &[u8]) -> State {
    match std::fs::symlink_metadata(dir) {
        Err(_) => State::Missing,
        Ok(m) if m.file_type().is_symlink() => State::Dev,
        Ok(_) => {
            let same = |name: &str, want: &[u8]| std::fs::read(dir.join(name)).is_ok_and(|b| b == want);
            if same("extension.wasm", wasm) && same("extension.toml", manifest) {
                State::Current
            } else {
                State::Outdated
            }
        }
    }
}

/// Install or update the extension. Zed loads it on its next start.
pub fn install(data: &Path) -> Result<()> {
    install_to(&folder(data), WASM, MANIFEST)
}

fn install_to(dir: &Path, wasm: &[u8], manifest: &[u8]) -> Result<()> {
    std::fs::create_dir_all(dir).context(|| format!("can't create {}", dir.display()))?;
    for (name, bytes) in [("extension.toml", manifest), ("extension.wasm", wasm)] {
        let path = dir.join(name);
        if std::fs::read(&path).is_ok_and(|b| b == bytes) {
            continue;
        }
        std::fs::write(&path, bytes).context(|| format!("can't write {}", path.display()))?;
    }
    Ok(())
}

/// Remove an installed (not a dev) extension.
pub fn remove(data: &Path) -> Result<bool> {
    let dir = folder(data);
    match state_of(&dir, WASM, MANIFEST) {
        State::Missing | State::Dev => Ok(false),
        _ => std::fs::remove_dir_all(&dir).map(|()| true).context(|| format!("can't remove {}", dir.display())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("learn-zed-ext-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        d.join(ID)
    }

    #[test]
    fn installs_then_sees_it_current_then_outdated() {
        let d = dir("cycle");
        assert_eq!(state_of(&d, b"wasm", b"toml"), State::Missing);
        install_to(&d, b"wasm", b"toml").unwrap();
        assert_eq!(std::fs::read(d.join("extension.wasm")).unwrap(), b"wasm");
        assert_eq!(state_of(&d, b"wasm", b"toml"), State::Current);
        assert_eq!(state_of(&d, b"wasm2", b"toml"), State::Outdated, "a newer learn carries a newer build");
        install_to(&d, b"wasm2", b"toml").unwrap();
        assert_eq!(state_of(&d, b"wasm2", b"toml"), State::Current);
    }

    #[test]
    fn the_data_folder_is_zeds() {
        let d = data_dir().expect("a home folder");
        let name = d.file_name().unwrap().to_string_lossy().to_lowercase();
        assert_eq!(name, "zed");
    }
}
