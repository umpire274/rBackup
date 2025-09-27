use crate::output::log_output;
use crate::utils::{
	CopyOptions, Messages, build_exclude_matcher, clear_terminal, copy_incremental,
};
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

	log_output(&msg.backup_init, logger, quiet, timestamp, None, true);
	log_output(
		&format!(
			"{} {} {} {}\n\n\n\n\n",
			msg.starting_backup,
			source.display(),
			msg.to,
			destination.display()
		),
		logger,
		quiet,
		timestamp,
		None,
		true,
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
	let (_cols, rows) = terminal::size().unwrap_or((80, 24));
	let progress_row = rows.saturating_sub(1);
	let exclude_matcher = if !exclude_patterns.is_empty() {
		match build_exclude_matcher(exclude_patterns) {
			Ok(matcher) => Some(matcher),
			Err(e) => {
				eprintln!("❌ {}: {}", msg.error_exclude_parsing, e);
				std::process::exit(1);
			}
		}
	} else {
		None
	};

	let options = CopyOptions {
		logger,
		quiet,
		timestamp,
		exclude_matcher,
		progress_row,
	};

	match copy_incremental(source, destination, msg, &options) {
		Ok((copied, skipped)) => {
			let done_msg = format!(
				"\n\n\n{} ({}, {})",
				msg.backup_ended,
				&msg.files_copied.replace("{}", &copied.to_string()),
				&msg.files_skipped.replace("{}", &skipped.to_string())
			);
			log_output(
				&done_msg,
				logger,
				options.quiet,
				options.timestamp,
				None,
				true,
			);
		}
		Err(e) => {
			let error_msg = format!("{}: {}", msg.generic_error, e);
			log_output(&error_msg, logger, false, options.timestamp, None, true);
			std::process::exit(1);
		}
	}
}
