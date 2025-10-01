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
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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

        /// Match exclude patterns against absolute paths instead of relative
        #[arg(long = "absolute-exclude", action = ArgAction::SetTrue, help = "Match exclude patterns against absolute source paths")]
        absolute_exclude: bool,

        /// Case-insensitive matching for exclude patterns
        #[arg(long = "ignore-case", action = ArgAction::SetTrue, help = "Perform case-insensitive matching for exclude patterns")]
        ignore_case: bool,

        /// Do a dry-run (don't actually copy files)
        #[arg(long = "dry-run", action = ArgAction::SetTrue, help = "Perform a dry-run without copying files")]
        dry_run: bool,
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
