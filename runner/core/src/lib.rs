//! The `learn` engine: the course, per-track specs, the learner's progress,
//! lesson notes, generating stubs and tests, and running checks. It does no
//! printing and serves no HTTP; the `learn` binary draws its results as the
//! CLI and the web UI.

pub mod backends;
pub mod check;
pub mod course;
pub mod ctx;
pub mod error;
pub mod lessons;
pub mod naming;
pub mod notes;
pub mod path;
pub mod pathfile;
pub mod spec;
pub mod toolchain;
pub mod views;
pub mod zed;
