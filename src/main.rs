mod commands;
mod config;
mod copy;
mod output;
mod ui;
mod utils;

use crate::config::Config;
use crate::utils::load_translations;
use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "rbackup",
    author = "Alessandro Maestri",
    version,
    about = "rBackup - New incremental directory backup",
    long_about = "rbackup: a Rust-based backup tool that copies only new or modified files from a source to a destination directory. Supports multithreading, language localization, logging, and progress display."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Perform an incremental backup
    Copy {
        /// Source directory to backup
        source: PathBuf,

        /// Destination directory
        destination: PathBuf,

        /// Suppress all output to stdout
        #[arg(short, long, action = ArgAction::SetTrue)]
        quiet: bool,

        /// Print timestamps in logs
        #[arg(short, long, action = ArgAction::SetTrue)]
        timestamp: bool,

        /// File path to write logs
        #[arg(long, value_name = "FILE")]
        log: Option<PathBuf>,

        /// Exclude files matching the given glob pattern (can be used multiple times)
        #[arg(short = 'x', long = "exclude", value_name = "PATTERN", action = ArgAction::Append)]
        exclude: Vec<String>,
    },

    /// Manage the configuration file (view or edit)
    Config {
        /// Initialize a default config file
        #[arg(long = "init", help = "Initiate a default config file")]
        init_config: bool,

        /// Print the current configuration file to stdout
        #[arg(long = "print", help = "Print the current configuration file")]
        print_config: bool,

        /// Edit the configuration file with your preferred editor
        #[arg(
            long = "edit",
            help = "Edit the configuration file (default editor: $EDITOR, or nano/vim/notepad)"
        )]
        edit_config: bool,

        /// Specify the editor to use (overrides $EDITOR/$VISUAL).
        /// Common choices: vim, nano.
        #[arg(
            long = "editor",
            help = "Specify the editor to use (vim, nano, or custom path)"
        )]
        editor: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let translations = load_translations()?;
    let lang_code = if Config::load().unwrap().language == "auto" {
        sys_locale::get_locale()
            .and_then(|val| val.split(&['_', '-']).next().map(str::to_lowercase))
            .unwrap_or_else(|| "en".to_string())
    } else {
        Config::load().unwrap().language.to_lowercase()
    };
    let msg = match translations.get(&lang_code) {
        Some(messages) => messages,
        None => {
            let fallback = translations
                .get("en")
                .expect("English translations missing");
            eprintln!(
                "{}",
                fallback.language_not_supported.replace("{}", &lang_code)
            );
            fallback
        }
    };

    match &cli.command {
        Commands::Config { .. } => commands::handle_conf(&cli.command, msg),
        Commands::Copy { .. } => commands::handle_copy(&cli.command, msg),
    }
}
