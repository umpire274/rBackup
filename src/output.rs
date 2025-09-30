use crate::utils::Logger;
use crossterm::style::ResetColor;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{Write, stdout};

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
}

const DEFAULT_TIMESTAMP_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn now(fmt: &str) -> String {
    chrono::Local::now().format(fmt).to_string()
}

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

    // Output su terminale
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

    // Output su file di log (solo se presente)
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
