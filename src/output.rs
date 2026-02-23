//! Output and logging utilities.
//!
//! This module centralizes terminal and file logging logic. The `LogContext`
//! struct carries per-run options such as timestamp formatting, quiet mode and
//! an optional file logger. The `log_output` function performs the combined
//! terminal and file write.

use crate::utils::Logger;
use crossterm::style::ResetColor;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{Write, stdout};

/// Context used for logging and terminal output.
///
/// The `LogContext` carries information about whether to print timestamps,
/// whether to suppress terminal output, the optional file logger, and other
/// runtime flags. It is intentionally clonable to simplify passing it across
/// threads or components.
#[derive(Debug, Clone)]
pub struct LogContext {
    pub logger: Option<Logger>,
    pub quiet: bool,
    pub with_timestamp: bool,
    pub timestamp_format: Option<String>,
    pub row: Option<u16>,
    pub on_log: bool,
    /// If true, the `exclude_matcher` will be matched against the absolute
    /// source path; when false (default), the matcher is applied to the path
    /// relative to the source directory.
    pub exclude_match_absolute: bool,
    /// If true, the copy operation will be a dry-run: files won't be
    /// actually copied, but the same output and counters will be produced.
    pub dry_run: bool,
    /// Optional list of the original exclude patterns (in the same order used to build the matcher).
    /// Useful to log which pattern caused a skip.
    pub exclude_patterns: Option<Vec<String>>,
    /// Optional exclude matcher that supports identifying which pattern matched.
    /// See `utils::ExcludeMatcher` for details.
    pub exclude_matcher: Option<crate::utils::ExcludeMatcher>,

    /// Control whether skipped items are printed to the scroll area.
    ///
    /// - Never: do not print skipped items.
    /// - Summary: do not print skipped items during the run (only show the final summary).
    /// - All: print both copied and skipped items.
    pub show_skipped: ShowSkipped,
}

/// Policy for displaying skipped items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShowSkipped {
    Never,
    Summary,
    #[default]
    All,
}
impl Default for LogContext {
    fn default() -> Self {
        Self {
            logger: None,
            quiet: false,
            with_timestamp: false,
            timestamp_format: None,
            row: None,
            on_log: true,
            exclude_match_absolute: false,
            dry_run: false,
            exclude_patterns: None,
            exclude_matcher: None,
            show_skipped: ShowSkipped::default(),
        }
    }
}

const DEFAULT_TIMESTAMP_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// Return current local time formatted with the given `strftime`-style format.
///
/// This is a small helper used by the logger to prepend timestamps to lines.
///
/// # Example
///
/// ```rust
/// let s = rbackup::output::now("%Y-%m-%d");
/// println!("today = {}", s);
/// ```
pub fn now(fmt: &str) -> String {
    chrono::Local::now().format(fmt).to_string()
}

/// Write a message to stdout and optionally to the configured file logger.
///
/// This function respects the `LogContext` flags:
/// - `quiet` disables terminal output
/// - `with_timestamp` (and `timestamp_format`) control the prepended timestamp
/// - `row` controls whether the message is printed at a specific terminal row
/// - `on_log` enables writing to the file logger (if present)
///
/// The function attempts to handle poisoned mutexes on the logger and will
/// still proceed to write to the terminal when possible.
///
/// # Parameters
/// - `msg`: message text to write (may contain newlines)
/// - `ctx`: logging context describing where/how to write
pub fn log_output(msg: &str, ctx: &LogContext) {
    let ts = if ctx.with_timestamp && !msg.trim().is_empty() {
        let fmt = ctx
            .timestamp_format
            .as_deref()
            .unwrap_or(DEFAULT_TIMESTAMP_FORMAT);
        format!("[{}] ", now(fmt))
    } else {
        String::new()
    };

    let full_msg = format!("{}{}", ts, msg);

    // Terminal output
    if !ctx.quiet {
        if let Some(row) = ctx.row {
            let _ = execute!(
                stdout(),
                MoveTo(0, row),
                Clear(ClearType::CurrentLine),
                Print(&full_msg),
                ResetColor
            );
        } else {
            println!("{}", full_msg);
        }
    }

    // File logger output (if present and enabled)
    if let Some(file) = &ctx.logger
        && ctx.on_log
    {
        match file.lock() {
            Ok(mut guard) => {
                let _ = writeln!(guard, "{}", full_msg);
            }
            Err(poisoned) => {
                // If poisoned, extract the inner guard and try to write
                let mut guard = poisoned.into_inner();
                let _ = writeln!(guard, "{}", full_msg);
            }
        }
    }
}
