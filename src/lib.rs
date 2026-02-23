//! rBackup library crate
//!
//! This crate exposes the internal modules and a small public surface used by
//! the binary `rbackup`. The documentation here provides a short overview and
//! re-exports for commonly used types.
//!
//! # Re-exports
//!
//! - `LogContext` — context for logging and output operations.
//! - `Logger`, `Messages` — utility types used across the crate.
//! - `build_exclude_matcher`, `copy_incremental`, `is_newer` — commonly used helper
//!   functions for building exclude matchers and performing incremental copies.

pub mod cli;
pub mod commands;
pub mod config;
pub mod copy;
pub mod output;
pub mod ui;
pub mod utils;

// Re-exports
pub use output::{LogContext, ShowSkipped};

/// Thread-safe file logger type: `Arc<Mutex<BufWriter<File>>>`.
pub use utils::{Logger, Messages, build_exclude_matcher, copy_incremental, is_newer};
