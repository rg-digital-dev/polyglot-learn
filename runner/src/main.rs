//! `learn`: local, multi-language exercises generated from language-neutral specs.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

mod commands;
mod markdown;
mod serve;
mod tui;
mod ui;
mod view;

// The engine lives in `learn-core`; these keep `crate::ctx` and friends working
// in the interface modules.
use learn_core::{backends, bail, check, course, ctx, error, notes, spec, toolchain, views};

#[derive(Parser)]
#[command(name = "learn", version, about = "Learn languages locally with small, test-driven exercises.")]
struct Cli {
    /// Keep your solutions and progress in this folder instead of `<repo>/workspace`
    /// (also: LEARN_WORKSPACE).
    #[arg(long, global = true, value_name = "DIR")]
    workspace: Option<PathBuf>,

    /// Colour output: auto (on a terminal, unless NO_COLOR is set), always or never.
    #[arg(long, global = true, value_enum, default_value_t = ui::ColorWhen::Auto, value_name = "WHEN")]
    color: ui::ColorWhen,

    /// Plain ASCII symbols (ok/FAIL/>/-) instead of ✓ ✗ ● ○.
    #[arg(long, global = true)]
    ascii: bool,

    /// Print long text directly instead of through a pager.
    #[arg(long, global = true)]
    no_pager: bool,

    /// Plain `learn` on a terminal opens the full-screen view; this prints
    /// `learn show` instead (also: NO_TUI=1).
    #[arg(long)]
    no_tui: bool,

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
        /// learn (one language), switch (from one you know) or compare (side by side)
        #[arg(long)]
        goal: Option<String>,
        /// For `--goal compare`: the tracks to compare, e.g. rust,go
        #[arg(long, value_delimiter = ',')]
        compare: Vec<String>,
    },
    /// Show which toolchains are installed for each track.
    Doctor,
    /// Pick a track and generate its workspace.
    Start { lang: String },
    /// Show an exercise: what to do and which file to edit (default: the current one).
    Show {
        slug: Option<String>,
        /// Also print the lesson, the track note and the links.
        #[arg(long)]
        full: bool,
    },
    /// Read the lesson for an exercise: the concept, your track's note and further reading.
    Lesson { slug: Option<String> },
    /// Read a page of the Syntax Primer (default: the next unread one).
    Read {
        /// A page, e.g. syntax-variables (or just variables)
        topic: Option<String>,
        /// Show this track's note instead of the active track's
        #[arg(long)]
        lang: Option<String>,
    },
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
    /// Show one concept's note from each track, one after another (your track's family first).
    Compare {
        concept: String,
        /// Comma-separated tracks, e.g. rust,go,lua
        #[arg(long, value_delimiter = ',')]
        lang: Vec<String>,
        /// Every track the concept is on, not only your track's family.
        #[arg(long, conflicts_with = "lang")]
        all: bool,
    },
    /// Regenerate tests (and missing stubs) after a spec changes.
    Generate {
        /// A track id, or `all` (default: the active track)
        #[arg(long)]
        lang: Option<String>,
    },
    /// Your progress on the active track.
    Status {
        /// Every track side by side.
        #[arg(long)]
        all: bool,
    },
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
    ui::init(cli.color, cli.ascii, cli.no_pager);
    let outcome = if cli.command.is_none() && tui::wanted(cli.no_tui) {
        tui::run(cli.workspace)
    } else {
        commands::dispatch(cli.workspace, cli.command)
    };
    match outcome {
        Ok(code) => code,
        Err(e) => {
            anstream::eprintln!("{}", ui::get().error(&e.0));
            ExitCode::FAILURE
        }
    }
}
