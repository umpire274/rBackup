use crate::cli::Commands;
use crate::config::Config;
use crate::copy::{execute_copy, start_copy_message};
use crate::output::{LogContext, log_output};
use crate::utils::{Messages, build_exclude_matcher, create_logger};

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
