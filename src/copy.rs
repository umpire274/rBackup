use crate::output::{LogContext, log_output};
use crate::utils::{Messages, clear_terminal, copy_incremental};
use crossterm::terminal;
use std::io::Write;
use std::path::Path;

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

pub fn execute_copy(msg: &Messages, ctx: &mut LogContext, source: &Path, destination: &Path) {
    let (_cols, rows) = terminal::size().unwrap_or((80, 24));
    let progress_row = rows.saturating_sub(1);

    // ctx.exclude_matcher is expected to be prepared by the caller (commands::handle_copy)

    ctx.row = Some(progress_row);

    match copy_incremental(source, destination, msg, ctx) {
        Ok((copied, skipped)) => {
            ctx.row = None;
            ctx.on_log = true;
            let done_msg = format!(
                "\n\n\n{} ({}, {})",
                msg.backup_ended,
                &msg.files_copied.replace("{}", &copied.to_string()),
                &msg.files_skipped.replace("{}", &skipped.to_string())
            );
            log_output(&done_msg, ctx);

            // Flush logger if present
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
        Err(e) => {
            ctx.quiet = false;
            ctx.row = None;
            ctx.on_log = true;
            let error_msg = format!("{}: {}", msg.generic_error, e);
            log_output(&error_msg, ctx);

            // Try to flush logger before exiting
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

            std::process::exit(1);
        }
    }
}
