use crate::output::{LogContext, log_output};
use crate::utils::{Messages, build_exclude_matcher, clear_terminal, copy_incremental};
use crossterm::terminal;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub fn start_copy_message(
    msg: &Messages,
    logger: &Option<Arc<Mutex<BufWriter<File>>>>,
    quiet: bool,
    timestamp: bool,
    source: &Path,
    destination: &Path,
) {
    clear_terminal();

    let ctx = LogContext {
        logger: logger.clone(),
        quiet,
        with_timestamp: timestamp,
        row: None,
        on_log: true,
        exclude_matcher: None,
    };

    log_output(&msg.backup_init, &ctx);
    log_output(
        &format!(
            "{} {} {} {}\n\n\n\n\n",
            msg.starting_backup,
            source.display(),
            msg.to,
            destination.display()
        ),
        &ctx,
    );
}

pub fn execute_copy(
    msg: &Messages,
    logger: &Option<Arc<Mutex<BufWriter<File>>>>,
    quiet: bool,
    timestamp: bool,
    exclude_patterns: &[String],
    source: &Path,
    destination: &Path,
) {
    let mut ctx = LogContext {
        logger: logger.clone(),
        quiet,
        with_timestamp: timestamp,
        row: None,
        on_log: true,
        exclude_matcher: None,
    };

    let (_cols, rows) = terminal::size().unwrap_or((80, 24));
    let progress_row = rows.saturating_sub(1);
    let exclude_matcher = if !exclude_patterns.is_empty() {
        match build_exclude_matcher(exclude_patterns) {
            Ok(matcher) => Some(matcher),
            Err(e) => {
                ctx.with_timestamp = false;
                ctx.on_log = false;
                log_output(
                    format!("❌ {}: {}", msg.error_exclude_parsing, e).as_str(),
                    &ctx,
                );
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    ctx.row = Some(progress_row);
    ctx.exclude_matcher = exclude_matcher;

    match copy_incremental(source, destination, msg, &ctx) {
        Ok((copied, skipped)) => {
            ctx.row = None;
            ctx.on_log = true;
            let done_msg = format!(
                "\n\n\n{} ({}, {})",
                msg.backup_ended,
                &msg.files_copied.replace("{}", &copied.to_string()),
                &msg.files_skipped.replace("{}", &skipped.to_string())
            );
            log_output(&done_msg, &ctx);
        }
        Err(e) => {
            ctx.quiet = false;
            ctx.row = None;
            ctx.on_log = true;
            let error_msg = format!("{}: {}", msg.generic_error, e);
            log_output(&error_msg, &ctx);
            std::process::exit(1);
        }
    }
}
