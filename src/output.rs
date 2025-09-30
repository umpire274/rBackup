use crate::config::Config;
use crate::utils::Logger;
use crossterm::style::ResetColor;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use globset::GlobSet;
use std::io::{Write, stdout};

#[derive(Debug, Clone)]
pub struct LogContext {
    pub logger: Option<Logger>,
    pub quiet: bool,
    pub with_timestamp: bool,
    pub row: Option<u16>,
    pub on_log: bool,
    pub exclude_matcher: Option<GlobSet>,
}

pub fn now() -> String {
    let custom_format = Config::load().unwrap().timestamp_format;
    chrono::Local::now()
        .format(custom_format.as_str())
        .to_string()
}

pub fn log_output(msg: &str, ctx: &LogContext) {
    let full_msg = if ctx.with_timestamp && !msg.trim().is_empty() {
        format!("[{}] {}", now(), msg)
    } else {
        msg.to_string()
    };

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
        let mut file = file.lock().unwrap();
        let _ = writeln!(file, "{}", full_msg);
    }
}
