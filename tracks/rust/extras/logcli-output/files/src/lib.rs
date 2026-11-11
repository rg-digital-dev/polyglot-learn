//! logq: read log files, filter them and summarise them.
//! `learn` writes this file as the project grows, one module per stage;
//! don't edit it.

pub mod args;
pub mod entry;
pub mod error;
pub mod filter;
pub mod input;
pub mod output;
pub mod run;
