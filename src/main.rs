mod ui;
mod utils;

use clap::{CommandFactory, Parser};
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use utils::{copy_incremental, load_translations, log_output, Logger};

#[derive(Parser, Debug)]
#[command(
    author = "Alessandro Maestri",
    version,
    about = "rBackup - New incremental directory backup",
    long_about = "rbackup: a Rust-based backup tool that copies only new or modified files from a source to a destination directory. Supports multithreading, language localization, logging, and progress display.",
    arg_required_else_help = true
)]
struct Args {
    /// Source directory
    source: Option<PathBuf>,

    /// Destination directory
    destination: Option<PathBuf>,

    /// Language code: en or it (default: en)
    #[arg(short, long, default_value = "auto")]
    lang: String,

    /// Show graphical progress bar instead of filenames
    #[arg(short = 'g', long = "graph")]
    show_graph: bool,

    /// Quiet mode: suppress console output
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Write output to a log file
    #[arg(long = "log", value_name = "FILE")]
    log_file: Option<PathBuf>,

    /// Add timestamp to log and console output
    #[arg(short = 't', long = "timestamp")]
    timestamp: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let translations = load_translations()?;
    let lang_code = if args.lang == "auto" {
        sys_locale::get_locale()
            .and_then(|val| val.split(&['_', '-']).next().map(str::to_lowercase))
            .unwrap_or_else(|| "en".to_string())
    } else {
        args.lang.to_lowercase()
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

    // Mostra help o version senza elevazione
    if (args.show_graph || args.quiet || args.log_file.is_none())
        && (args.source.is_none() || args.destination.is_none())
    {
        eprintln!("Error: missing source or destination directory.\n");
        Args::command().print_help()?;
        println!();
        std::process::exit(1);
    }

    let logger: Logger = if let Some(ref path) = args.log_file {
        match File::create(path) {
            Ok(file) => Some(Arc::new(Mutex::new(file))),
            Err(e) => {
                eprintln!("Failed to create log file: {e}");
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    let source = args.source.as_ref().unwrap();
    let destination = args.destination.as_ref().unwrap();

    if !source.is_dir() {
        eprintln!("{}", msg.invalid_source);
        std::process::exit(1);
    }

    if !destination.exists() {
        std::fs::create_dir_all(destination)?;
    }

    log_output(
        msg.backup_init.as_str(),
        &logger,
        args.quiet,
        args.timestamp,
    );

    log_output(
        &format!(
            "{} {} {} {}\n\n",
            msg.starting_backup,
            source.display(),
            msg.to,
            destination.display()
        ),
        &logger,
        args.quiet,
        args.timestamp,
    );

    let (copied, skipped) = copy_incremental(
        source,
        destination,
        msg,
        &logger,
        args.quiet,
        args.timestamp,
    )?;

    log_output(
        format!("\n\n{}", msg.backup_ended.as_str()).as_str(),
        &logger,
        args.quiet,
        args.timestamp,
    );
    log_output(
        &msg.files_copied.replace("{}", &copied.to_string()),
        &logger,
        args.quiet,
        args.timestamp,
    );
    log_output(
        &msg.files_skipped.replace("{}", &skipped.to_string()),
        &logger,
        args.quiet,
        args.timestamp,
    );

    Ok(())
}
