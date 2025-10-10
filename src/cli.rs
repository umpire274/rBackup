//! Command-line interface definitions for the `rbackup` binary.
//!
//! This module defines the clap-powered `Cli` parser and the `Commands` enum
//! describing the supported subcommands and their options.

use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

/// Parsed command-line arguments for the `rbackup` binary.
///
/// Use `Cli::parse()` (provided by clap) to obtain a populated instance.
///
/// If no subcommand is provided the `command` field will be `None` and the
/// caller can decide how to react (for example: print help and exit 0).
///
/// # Examples
///
/// ```rust,no_run
/// use clap::Parser;
/// use rbackup::cli::Cli;
/// // Let clap parse arguments from the current process
/// let cli = Cli::parse();
/// match cli.command {
///     Some(rbackup::cli::Commands::Copy { .. }) => println!("Copy command chosen"),
///     Some(rbackup::cli::Commands::Config { .. }) => println!("Config command chosen"),
///     None => println!("No subcommand provided"),
/// }
/// ```
#[derive(Debug, Parser)]
#[command(
    name = "rbackup",
    author = "Alessandro Maestri",
    version,
    about = "rBackup - New incremental directory backup",
    long_about = "rbackup: a Rust-based backup tool that copies only new or modified files from a source to a destination directory. Supports multithreading, language localization, logging, and progress display."
)]
pub struct Cli {
    /// Selected subcommand and its options. Optional to allow showing help
    /// without Clap treating the absence of a subcommand as an error.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available subcommands for `rbackup`.
///
/// Each variant contains the options relevant to that operation.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Perform an incremental backup
    Copy {
        /// Source directory to backup
        ///
        /// This is interpreted as a filesystem path. The program will traverse
        /// the directory recursively and copy new or modified files to the
        /// destination.
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
