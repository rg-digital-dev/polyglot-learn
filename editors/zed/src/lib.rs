//! Starts `learn lsp` for polysyntax exercises: failing test cases show as
//! diagnostics on save, hovering a signature shows the task and the hints you
//! revealed, and code actions run the check, reveal a hint or open LESSON.md.
//! All the work happens in `learn`; this only finds it and starts it.

use zed_extension_api::{self as zed, LanguageServerId, Result};

/// Files at the project root that mark a polysyntax checkout or workspace.
const MARKERS: [&str; 2] = ["course.toml", ".learn/config.toml"];

struct Polysyntax;

impl zed::Extension for Polysyntax {
    fn new() -> Self {
        Polysyntax
    }

    fn language_server_command(&mut self, _id: &LanguageServerId, worktree: &zed::Worktree) -> Result<zed::Command> {
        // Other Go or Python projects get no server: `learn lsp` has nothing to say there.
        if !is_course(|f| worktree.read_text_file(f).is_ok()) {
            return Err(format!("not a polysyntax folder: open your polysyntax checkout or your `learn` workspace (no {} at the project root)", MARKERS.join(" or ")));
        }
        // A `lsp.polysyntax.binary.path` setting never gets here: Zed then
        // starts that binary itself, with the setting's `arguments`.
        let command = worktree.which("learn").ok_or_else(|| {
            "`learn` isn't on your PATH. Install polysyntax (see its README), or set lsp.polysyntax.binary.path (with \"arguments\": [\"lsp\"]) in Zed's settings".to_string()
        })?;
        Ok(zed::Command { command, args: vec!["lsp".into()], env: worktree.shell_env() })
    }
}

/// True when one of the markers exists at the project root.
fn is_course(exists: impl Fn(&str) -> bool) -> bool {
    MARKERS.iter().any(|m| exists(m))
}

zed::register_extension!(Polysyntax);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_checkout_or_a_workspace_counts() {
        assert!(is_course(|f| f == "course.toml"));
        assert!(is_course(|f| f == ".learn/config.toml"));
        assert!(!is_course(|f| f == "go.mod"));
    }
}
