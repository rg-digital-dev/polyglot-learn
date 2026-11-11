//! A `.zed/tasks.json` in the folder you open, so Zed's `task: spawn` offers
//! `learn check` and `learn watch`. Zed extensions can't add tasks themselves,
//! so `learn` writes the file once, and never over one that's already there.

use std::path::PathBuf;

use crate::backends;
use crate::ctx::Ctx;
use crate::error::Result;

/// The tasks, as Zed reads them. Each runs in the folder Zed has open.
pub const TASKS: &str = r#"[
  {
    "label": "learn check",
    "command": "learn",
    "args": ["check"],
    "save": "all",
    "reveal": "always"
  },
  {
    "label": "learn watch",
    "command": "learn",
    "args": ["watch"],
    "reveal": "always"
  }
]
"#;

/// Where the file goes: the checkout, or your own workspace when it's outside
/// the checkout. That's the folder with `course.toml` or `.learn/config.toml`,
/// which is also where the Zed extension starts `learn lsp`.
pub fn tasks_path(ctx: &Ctx) -> PathBuf {
    let folder = if ctx.external_workspace { &ctx.workspace } else { &ctx.root };
    folder.join(".zed").join("tasks.json")
}

/// Write the tasks unless the folder already has a tasks file (yours wins).
pub fn write_tasks(ctx: &Ctx) -> Result<()> {
    let path = tasks_path(ctx);
    if path.exists() {
        return Ok(());
    }
    backends::write(&path, TASKS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_once_and_leaves_yours_alone() {
        let ws = std::env::temp_dir().join(format!("learn-zed-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&ws);
        let ctx = Ctx::discover(Some(ws.clone())).unwrap();
        let path = tasks_path(&ctx);
        assert_eq!(path, ws.join(".zed").join("tasks.json"), "an outside workspace gets its own");

        write_tasks(&ctx).unwrap();
        let tasks: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        let labels: Vec<_> = tasks.as_array().unwrap().iter().map(|t| t["label"].as_str().unwrap()).collect();
        assert_eq!(labels, ["learn check", "learn watch"]);

        std::fs::write(&path, "[]").unwrap();
        write_tasks(&ctx).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "[]");
    }
}
