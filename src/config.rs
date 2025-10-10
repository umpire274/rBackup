//! Configuration helpers and utilities for rbackup.
//!
//! This module defines the `Config` structure representing user-configurable
//! settings, plus helpers to read, write and edit the configuration file.

use std::env;
use std::fs;
use std::io::{self};
use std::path::PathBuf;
use std::process::Command;

use serde::{Deserialize, Serialize};

/// Runtime configuration for the application.
///
/// This struct is deserialized from a YAML configuration file. Fields are kept
/// minimal: language and timestamp format. Add fields here if you extend the
/// configuration schema.
///
/// # Examples
///
/// ```rust,no_run
/// use rbackup::config::Config;
/// let cfg = Config::load_or_default();
/// println!("Language: {}", cfg.language);
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Language code used for messages ("auto", "en", "it", ...)
    pub language: String,
    /// Timestamp format used when printing timestamps in logs (strftime syntax)
    pub timestamp_format: String,
}

/// Default configuration file template (YAML).
///
/// This string is written when the user asks to initialize a configuration
/// file.
pub const DEFAULT_CONFIG_TEMPLATE: &str = r#"
# RBackup configuration file
# --------------------------

# Language for messages.
# Supported values:
# - auto   -> uses system locale
# - en     -> English
# - it     -> Italian
language: auto

# Timestamp format for log entries (uses `strftime` syntax)
# Common placeholders:
# - %Y -> year (e.g. 2025)
# - %m -> month (01-12)
# - %d -> day (01-31)
# - %H -> hour (00-23)
# - %M -> minute (00-59)
# - %S -> second (00-59)
# Full reference: https://docs.rs/chrono/latest/chrono/format/strftime/index.html
timestamp_format: '%Y-%m-%d %H:%M:%S'
"#;

impl Config {
    /// Create the default configuration file on disk.
    ///
    /// The file path is returned by `Config::config_file()`; any parent
    /// directory is created as needed.
    pub fn default_config() -> io::Result<()> {
        let path = Config::config_file();
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let content = DEFAULT_CONFIG_TEMPLATE;
        fs::write(&path, content)?;

        Ok(())
    }

    /// Compute the canonical location of the configuration file for the
    /// current platform.
    ///
    /// On Windows this returns `%APPDATA%\\rbackup\\rbackup.conf`. On Unix
    /// platforms it returns `$HOME/.rbackup/rbackup.conf`.
    pub fn config_file() -> PathBuf {
        if cfg!(target_os = "windows") {
            let appdata = env::var("APPDATA").unwrap_or_else(|_| ".".into());
            PathBuf::from(format!("{appdata}\\rbackup\\rbackup.conf"))
        } else {
            let home = env::var("HOME").unwrap_or_else(|_| ".".into());
            PathBuf::from(format!("{home}/.rbackup/rbackup.conf"))
        }
    }

    /// Load the configuration from disk.
    ///
    /// Returns `io::Error` if the file does not exist or if its contents are
    /// invalid YAML.
    pub fn load() -> io::Result<Self> {
        let path = Config::config_file();
        let content = fs::read_to_string(&path)?;
        let conf: Config = serde_yaml::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(conf)
    }

    /// Load the configuration, falling back to sensible defaults when the
    /// configuration file is missing or invalid.
    pub fn load_or_default() -> Self {
        Config::load().unwrap_or_else(|_| Config {
            language: "auto".to_string(),
            timestamp_format: "%Y-%m-%d %H:%M:%S".to_string(),
        })
    }

    /// Open the configuration file in the user's preferred editor.
    ///
    /// The `editor` parameter, if provided, overrides the `EDITOR` or
    /// `VISUAL` environment variables. On Windows the default editor is
    /// `notepad`, on other platforms `vi`.
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
