//! `learn`: local, multi-language exercises generated from language-neutral specs.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

mod backends;
mod check;
mod commands;
mod course;
mod ctx;
mod error;
mod naming;
mod notes;
mod serve;
mod spec;
mod toolchain;

#[derive(Parser)]
#[command(name = "learn", version, about = "Learn languages locally with small, test-driven exercises.")]
struct Cli {
    /// Keep your solutions and progress in this folder instead of `<repo>/workspace`
    /// (also: LEARN_WORKSPACE).
    #[arg(long, global = true, value_name = "DIR")]
    workspace: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Tell learn which language you already know, so notes compare against it.
    Init {
        /// typescript, javascript, python, php, go, rust, java, csharp, lua or none
        #[arg(long)]
        from: Option<String>,
    },
    /// Show which toolchains are installed for each track.
    Doctor,
    /// Pick a track and generate its workspace.
    Start { lang: String },
    /// Show an exercise: lesson, track note and the file to edit (default: current).
    Show { slug: Option<String> },
    /// Run the tests; mark the exercise done when they pass.
    Check {
        slug: Option<String>,
        /// Print the full test output.
        #[arg(short, long)]
        verbose: bool,
    },
    /// Re-run the current exercise on every save and move on when it passes.
    Watch,
    /// Reveal the next hint.
    Hint { slug: Option<String> },
    /// Show one concept's note from every track, one after another.
    Compare {
        concept: String,
        /// Comma-separated tracks, e.g. rust,go,lua
        #[arg(long, value_delimiter = ',')]
        lang: Vec<String>,
    },
    /// Regenerate tests (and missing stubs) after a spec changes.
    Generate {
        /// A track id, or `all` (default: the active track)
        #[arg(long)]
        lang: Option<String>,
    },
    /// Progress table: exercises by track.
    Status,
    /// List (or open) further-reading links for a concept.
    Docs {
        concept: Option<String>,
        #[arg(long)]
        lang: Option<String>,
        /// Open the links in your browser.
        #[arg(long)]
        open: bool,
    },
    /// Check every link in tracks/*/track.toml and report dead ones.
    CheckLinks,
    /// Open the web UI at http://learn.localhost:4321 (stops with Ctrl+C or Quit).
    Serve {
        #[arg(long, default_value_t = 4321)]
        port: u16,
        /// Open it in your default browser.
        #[arg(long)]
        open: bool,
    },
    /// For maintainers: verify untouched stubs fail and reference solutions pass.
    Selftest {
        /// Comma-separated tracks (default: all)
        #[arg(long, value_delimiter = ',')]
        lang: Vec<String>,
        /// Folder with reference solutions laid out as <lang>/<path of the stub>
        #[arg(long, value_name = "DIR")]
        solutions: Option<PathBuf>,
        /// Fail when a toolchain is missing instead of skipping that track.
        #[arg(long)]
        require_all: bool,
    },
}

/// No raw panics for learners: a closed pipe (`learn status | head`) exits
/// quietly, and anything else becomes a short message that asks for a report.
fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let msg = info
            .payload()
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_default();
        if msg.contains("failed printing to stdout") || msg.contains("failed printing to stderr") {
            std::process::exit(0);
        }
        let place = info.location().map(|l| format!(" ({}:{})", l.file(), l.line())).unwrap_or_default();
        eprintln!(
            "error: learn hit an internal problem: {msg}{place}\n\
             This is a bug. Please report it with the command you ran and the output of `learn doctor`:\n\
             https://github.com/rg-digital-dev/polyglot-learn/issues"
        );
        std::process::exit(101);
    }));
}

fn main() -> ExitCode {
    install_panic_hook();
    let cli = Cli::parse();
    match commands::dispatch(cli.workspace, cli.command) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
