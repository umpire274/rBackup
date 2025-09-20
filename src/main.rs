mod output;
mod ui;
mod utils;

use crate::output::print_message;
use clap::{CommandFactory, Parser};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, terminal};
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::stdout;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use utils::{copy_incremental, load_translations};

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

fn clear_terminal() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
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
    if (args.quiet || args.log_file.is_none())
        && (args.source.is_none() || args.destination.is_none())
    {
        eprintln!("Error: missing source or destination directory.\n");
        Args::command().print_help()?;
        println!();
        std::process::exit(1);
    }

    let logger = args.log_file.as_ref().map(|path| {
        let file = File::create(path).expect("Unable to create log file");
        Arc::new(Mutex::new(BufWriter::new(file)))
    });

    let source = args.source.as_ref().unwrap();
    let destination = args.destination.as_ref().unwrap();

    if !source.is_dir() {
        eprintln!("{}", msg.invalid_source);
        std::process::exit(1);
    }

    if !destination.exists() {
        std::fs::create_dir_all(destination)?;
    }

    clear_terminal();

    print_message(
        &msg.backup_init,
        &logger.as_ref(),
        args.quiet,
        args.timestamp,
        None,
    );
    print_message(
        &format!(
            "{} {} {} {}\n\n\n\n\n",
            msg.starting_backup,
            source.display(),
            msg.to,
            destination.display()
        ),
        &logger.as_ref(),
        args.quiet,
        args.timestamp,
        None,
    );

    let (_cols, rows) = terminal::size().unwrap_or((80, 24));
    let progress_row = rows.saturating_sub(1);

    match copy_incremental(
        source,
        destination,
        msg,
        &logger.as_ref(),
        args.quiet,
        args.timestamp,
        progress_row,
    ) {
        Ok((copied, skipped)) => {
            let done_msg = format!(
                "\n\n\n{} ({}, {})",
                msg.backup_ended,
                &msg.files_copied.replace("{}", &copied.to_string()),
                &msg.files_skipped.replace("{}", &skipped.to_string())
            );
            print_message(
                &done_msg,
                &logger.as_ref(),
                args.quiet,
                args.timestamp,
                None,
            );
        }
        Err(e) => {
            let error_msg = format!("{}: {}", msg.generic_error, e);
            print_message(&error_msg, &logger.as_ref(), false, args.timestamp, None);
            std::process::exit(1);
        }
    }

    Ok(())
}
