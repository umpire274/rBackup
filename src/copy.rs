//! High-level copy helpers and user-facing messages.
//!
//! This module implements the start/finish messages and the main copy loop
//! orchestration. The heavy lifting is performed by utilities in `utils` and
//! `output` modules; this module coordinates UI, logging flush and error
//! reporting.

use crate::output::{LogContext, log_output};
use crate::utils::{Messages, clear_terminal, copy_incremental};
use crossterm::terminal;
use std::io::Write;
use std::path::Path;

/// Print the initial messages shown when a copy operation starts.
///
/// This clears the terminal and prints the localized "backup started" messages
/// including source and destination paths. It uses the provided `LogContext`
/// for formatting decisions (quiet / timestamps) but does not mutate it.
///
/// # Parameters
/// - `msg`: localized messages bundle.
/// - `ctx`: current log/output context.
/// - `source`: source directory path.
/// - `destination`: destination directory path.
pub fn start_copy_message(msg: &Messages, ctx: &LogContext, source: &Path, destination: &Path) {
    clear_terminal();

    // Use provided context directly (don't mutate)
    log_output(&msg.backup_init, ctx);
    log_output(
        &format!(
            "{} {} {} {}\n\n\n\n\n",
            msg.starting_backup,
            source.display(),
            msg.to,
            destination.display()
        ),
        ctx,
    );
}

// Helper function: centralize logger flush to avoid duplication
// (kept private as internal utility)
fn flush_logger(ctx: &mut LogContext) {
    if let Some(log) = &ctx.logger {
        match log.lock() {
            Ok(mut guard) => {
                let _ = guard.flush();
            }
            Err(poisoned) => {
                let mut guard = poisoned.into_inner();
                let _ = guard.flush();
            }
        }
    }
}

/// Run the incremental copy operation and print final messages.
///
/// This function sets up the progress row, calls the core `copy_incremental`
/// helper and prints a summary or a fatal error message. On unrecoverable
/// errors it attempts to flush the logger and exits the process.
///
/// # Parameters
/// - `msg`: localized messages bundle.
/// - `ctx`: mutable logging/context object; some fields (row, on_log) are
///   updated during execution.
/// - `source`: source directory path.
/// - `destination`: destination directory path.
pub fn execute_copy(
    msg: &Messages,
    ctx: &mut LogContext,
    source: &Path,
    destination: &Path,
    delta: bool,
) {
    let (_cols, rows) = terminal::size().unwrap_or((80, 24));
    let progress_row = rows.saturating_sub(1);

    // ctx.exclude_matcher is expected to be prepared by the caller (commands::handle_copy)

    ctx.row = Some(progress_row);

    match copy_incremental(source, destination, msg, ctx, delta) {
        Ok((copied, skipped)) => {
            ctx.row = None;
            ctx.on_log = true;
            let done_msg = format!(
                "\n\n\n{} ({}. {}, {})",
                &msg.backup_ended,
                &msg.files_total
                    .replace("{}", &(copied + skipped).to_string()),
                &msg.files_copied.replace("{}", &copied.to_string()),
                &msg.files_skipped.replace("{}", &skipped.to_string())
            );

            log_output(&done_msg, ctx);

            // Flush logger if present
            flush_logger(ctx);
        }
        Err(e) => {
            ctx.quiet = false;
            ctx.row = None;
            ctx.on_log = true;
            let error_msg = format!("{}: {}", msg.generic_error, e);
            log_output(&error_msg, ctx);

            // Try to flush logger before exiting
            flush_logger(ctx);

            std::process::exit(1);
        }
    }
}
