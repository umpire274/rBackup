use crate::Commands;
use crate::config::Config;
use crate::copy::{execute_copy, start_copy_message};
use crate::output::{LogContext, log_output};
use crate::utils::{Messages, create_logger};

pub fn handle_conf(cmd: &Commands, msg: &Messages) -> Result<(), Box<dyn std::error::Error>> {
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
            row: None,
            on_log: false,
            exclude_matcher: None,
        };

        if *print_config {
            match Config::load() {
                Ok(conf) => {
                    log_output(
                        format!(
                            "📄 {}:\n{}",
                            msg.cur_conf,
                            serde_yaml::to_string(&conf).unwrap()
                        )
                            .as_str(),
                        &ctx,
                    );
                }
                Err(_) => log_output(format!("⚠️ {}.", msg.conf_file_not_found).as_str(), &ctx),
            }
        }

        if *init_config {
            let _ = Config::default_config();
            log_output(
                format!("✅ {} {:?}", msg.conf_initialized, Config::config_file()).as_str(),
                &ctx,
            );
        }

        if *edit_config {
            Config::edit(editor.clone()).unwrap();
        }
    }
    Ok(())
}

pub fn handle_copy(cmd: &Commands, msg: &Messages) -> Result<(), Box<dyn std::error::Error>> {
    if let Commands::Copy {
        source,
        destination,
        quiet,
        timestamp,
        log,
        exclude,
    } = cmd
    {
        let logger = create_logger(log.as_deref());

        start_copy_message(msg, &logger, *quiet, *timestamp, source, destination);

        execute_copy(
            msg,
            &logger,
            *quiet,
            *timestamp,
            exclude,
            source,
            destination,
        );
    }
    Ok(())
}
