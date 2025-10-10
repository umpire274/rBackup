//! Command handlers invoked by the CLI dispatcher.
//!
//! This module contains the high-level functions that implement the behavior
//! of the `config` and `copy` subcommands. They adapt CLI arguments and the
//! loaded configuration into the lower-level utilities responsible for I/O,
//! logging and copying.

use crate::cli::Commands;
use crate::config::Config;
use crate::copy::{execute_copy, start_copy_message};
use crate::output::{LogContext, log_output};
use crate::utils::{Messages, build_exclude_matcher, create_logger};

/// Handle the `config` subcommand.
///
/// This function implements the logic for printing, creating or editing the
/// configuration file. It receives the parsed CLI command, the localized
/// messages and the effective configuration.
///
/// # Parameters
/// - `cmd`: the parsed CLI command (expected to be `Commands::Config`).
/// - `msg`: localized messages used for output.
/// - `config`: currently loaded configuration values.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(...)` on unexpected I/O errors (for example when creating the config file).
pub fn handle_conf(
    cmd: &Commands,
    msg: &Messages,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Commands::Config {
        print_config,
        init_config,
        edit_config,
        editor,
    } = cmd
    {
        println!();
        let ctx = LogContext {
            logger: None,
            quiet: false,
            with_timestamp: false,
            timestamp_format: Some(config.timestamp_format.clone()),
            row: None,
            on_log: false,
            exclude_matcher: None,
            exclude_match_absolute: false,
            dry_run: false,
            exclude_patterns: None,
        };

        if *print_config {
            match Config::load() {
                Ok(conf) => {
                    log_output(
                        format!(
                            "\u{1F4C4} {}:\n{}",
                            msg.cur_conf,
                            serde_yaml::to_string(&conf).unwrap()
                        )
                        .as_str(),
                        &ctx,
                    );
                }
                Err(_) => log_output(
                    format!("\u{26A0}\u{FE0F} {}.", msg.conf_file_not_found).as_str(),
                    &ctx,
                ),
            }
        }

        if *init_config {
            let _ = Config::default_config();
            log_output(
                format!(
                    "\u{2705} {} {:?}",
                    msg.conf_initialized,
                    Config::config_file()
                )
                .as_str(),
                &ctx,
            );
        }

        if *edit_config {
            Config::edit(editor.clone()).unwrap();
        }
    }
    Ok(())
}

/// Handle the `copy` subcommand.
///
/// This function prepares logging, exclude matchers and the `LogContext`,
/// then starts the copy operation and prints the start/finish messages.
///
/// # Parameters
/// - `cmd`: the parsed CLI command (expected to be `Commands::Copy`).
/// - `msg`: localized messages used for output and log lines.
/// - `config`: loaded configuration values, used for default timestamp formatting.
///
/// # Behavior and errors
/// - Attempts to create an optional log file if requested. If log creation
///   fails, an error message is printed and logging is disabled.
/// - If exclude patterns are provided and parsing fails, the function will
///   log an error and return a failure.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(...)` if exclude pattern parsing fails or other I/O errors occur.
pub fn handle_copy(
    cmd: &Commands,
    msg: &Messages,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Commands::Copy {
        source,
        destination,
        quiet,
        timestamp,
        log,
        exclude,
        absolute_exclude,
        ignore_case,
        dry_run,
    } = cmd
    {
        // create_logger now returns io::Result<Option<Logger>>
        let logger = match create_logger(log.as_deref()) {
            Ok(l) => l,
            Err(e) => {
                // Create a temporary ctx to report the error
                let ctx = LogContext {
                    logger: None,
                    quiet: false,
                    with_timestamp: true,
                    timestamp_format: Some(config.timestamp_format.clone()),
                    row: None,
                    on_log: false,
                    exclude_matcher: None,
                    exclude_match_absolute: false,
                    dry_run: false,
                    exclude_patterns: None,
                };
                log_output(&format!("Failed to create log file: {}", e), &ctx);
                None
            }
        };

        // Build a mutable LogContext and pass it to copy functions to reduce arg count
        let mut ctx = LogContext {
            logger: logger.clone(),
            quiet: *quiet,
            with_timestamp: *timestamp,
            timestamp_format: Some(config.timestamp_format.clone()),
            row: None,
            on_log: true,
            exclude_matcher: None,
            exclude_match_absolute: *absolute_exclude,
            dry_run: *dry_run,
            exclude_patterns: None,
        };

        // Build exclude matcher here (avoid duplication)
        if !exclude.is_empty() {
            match build_exclude_matcher(exclude, *ignore_case) {
                Ok(matcher) => {
                    ctx.exclude_matcher = Some(matcher);
                    ctx.exclude_patterns = Some(exclude.clone());
                }
                Err(e) => {
                    // report and return error
                    ctx.with_timestamp = false;
                    ctx.on_log = false;
                    log_output(
                        format!("\u{274C} {}: {}", msg.error_exclude_parsing, e).as_str(),
                        &ctx,
                    );
                    return Err(Box::new(e));
                }
            }
        }

        start_copy_message(msg, &ctx, source, destination);

        execute_copy(msg, &mut ctx, source, destination);
    }
    Ok(())
}
