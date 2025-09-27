use std::env;
use std::fs;
use std::io::{self};
use std::path::PathBuf;
use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub language: String,
	pub timestamp_format: String,
}

pub const DEFAULT_CONFIG_TEMPLATE: &str = r#"
# RBackup configuration file
# --------------------------

# Language for messages.
# Supported values:
# - auto   → uses system locale
# - en     → English
# - it     → Italian
language: auto

# Timestamp format for log entries (uses `strftime` syntax)
# Common placeholders:
# - %Y → year (e.g. 2025)
# - %m → month (01-12)
# - %d → day (01-31)
# - %H → hour (00-23)
# - %M → minute (00-59)
# - %S → second (00-59)
# Full reference: https://docs.rs/chrono/latest/chrono/format/strftime/index.html
timestamp_format: '%Y-%m-%d %H:%M:%S'
"#;

impl Config {
	pub fn default_config() -> io::Result<()> {
		let path = Config::config_file();
		if let Some(dir) = path.parent() {
			fs::create_dir_all(dir)?;
		}
		let content = DEFAULT_CONFIG_TEMPLATE;
		fs::write(&path, content)?;

		Ok(())
	}

	pub fn config_file() -> PathBuf {
		if cfg!(target_os = "windows") {
			let appdata = env::var("APPDATA").unwrap_or_else(|_| ".".into());
			PathBuf::from(format!("{appdata}\\rbackup\\rbackup.conf"))
		} else {
			let home = env::var("HOME").unwrap_or_else(|_| ".".into());
			PathBuf::from(format!("{home}/.rbackup/rbackup.conf"))
		}
	}

	pub fn load() -> io::Result<Self> {
		let path = Config::config_file();
		let content = fs::read_to_string(&path)?;
		let conf: Config = serde_yaml::from_str(&content)
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
		Ok(conf)
	}

	pub fn edit(editor: Option<String>) -> io::Result<()> {
		let editor_cmd = editor
			.or_else(|| env::var("EDITOR").ok())
			.or_else(|| env::var("VISUAL").ok())
			.unwrap_or_else(|| {
				if cfg!(target_os = "windows") {
					"notepad".to_string()
				} else {
					"vi".to_string()
				}
			});

		let path = Config::config_file();
		Command::new(editor_cmd)
			.arg(path)
			.status()
			.map(|_| ())
			.map_err(|e| io::Error::other(format!("Failed to open editor: {e}")))
	}
}
