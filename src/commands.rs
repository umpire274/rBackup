use crate::Commands;
use crate::config::Config;
use crate::copy::{execute_copy, start_copy_message};
use crate::utils::Messages;
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};

pub fn handle_conf(cmd: &Commands, msg: &Messages) -> Result<(), Box<dyn std::error::Error>> {
	if let Commands::Config {
		print_config,
		init_config,
		edit_config,
		editor,
	} = cmd
	{
		println!();
		if *print_config {
			match Config::load() {
				Ok(conf) => {
					println!("📄 {}:", msg.cur_conf);
					println!("{}", serde_yaml::to_string(&conf).unwrap());
				}
				Err(_) => println!("⚠️ {}.", msg.conf_file_not_found),
			}
		}

		if *init_config {
			let _ = Config::default_config();
			println!("✅ {} {:?}", msg.conf_initialized, Config::config_file());
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
		let logger = log.as_ref().map(|path| {
			let file = File::create(path).expect("Unable to create log file");
			Arc::new(Mutex::new(BufWriter::new(file)))
		});

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
		/*println!();
		println!("Running backup from {:?} to {:?}", source, destination);
		println!("Quiet: {}", quiet);
		println!("Timestamp: {}", timestamp);
		println!("Log file: {:?}", logger);
		println!("Exclude patterns: {:?}", exclude);
		 */
	}
	Ok(())
}
