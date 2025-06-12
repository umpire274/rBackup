use clap::{Parser, CommandFactory};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(
    author = "Alessandro Maestri",
    version,
    about = "Incremental directory backup for Windows",
    long_about = "winrsync: a Rust-based backup tool that copies only new or modified files from a source to a destination directory. Supports multithreading, language localization, and progress display.",
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
}

#[derive(Deserialize)]
struct Messages {
    starting_backup: String,
    to: String,
    copying_file: String,
    backup_done: String,
    invalid_source: String,
    language_not_supported: String,
}

type Translations = HashMap<String, Messages>;

fn load_translations() -> io::Result<Translations> {
    let data = include_str!("../assets/translations.json");
    let translations: Translations = serde_json::from_str(data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(translations)
}

fn is_newer(src: &Path, dest: &Path) -> io::Result<bool> {
    if !dest.exists() {
        return Ok(true);
    }

    let src_meta = src.metadata()?;
    let dest_meta = dest.metadata()?;

    Ok(src_meta.modified()? > dest_meta.modified()?)
}

fn copy_incremental(
    src_dir: &Path,
    dest_dir: &Path,
    msg: &Messages,
    show_graph: bool,
) -> io::Result<()> {
    let entries: Vec<_> = WalkDir::new(src_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.path().is_file()
                && !e.path().extension().map_or(false, |ext| {
                    matches!(ext.to_str(), Some("gsheet" | "gdoc" | "gslides"))
                })
        })
        .collect();

    let pb = if show_graph {
        let bar = ProgressBar::new(entries.len() as u64);
        bar.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}",
            )
            .unwrap()
            .progress_chars("=> "),
        );
        Some(bar)
    } else {
        None
    };

    entries.par_iter().try_for_each(|entry| -> io::Result<()> {
        let src_path = entry.path();
        let rel_path = src_path.strip_prefix(src_dir).unwrap();
        let dest_path = dest_dir.join(rel_path);

        if is_newer(src_path, &dest_path)? {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::copy(src_path, dest_path)?;

            if let Some(ref pb) = pb {
                pb.inc(1);
            } else {
                println!("{} {}", msg.copying_file, rel_path.display());
            }
        } else if let Some(ref pb) = pb {
            pb.inc(1);
        }

        Ok(())
    })?;

    if let Some(pb) = pb {
        pb.finish_with_message(msg.backup_done.clone());
    } else {
        println!("{}", msg.backup_done);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Mostra help se mancano sorgente o destinazione
    if args.source.is_none() || args.destination.is_none() {
        eprintln!("Error: missing source or destination directory.\n");
        Args::command().print_help().unwrap();
        println!();
        std::process::exit(1);
    }

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
            let fallback = translations.get("en").expect("English translations missing");
            eprintln!(
                "{}",
                fallback
                    .language_not_supported
                    .replace("{}", &lang_code)
            );
            fallback
        }
    };

    let source = args.source.as_ref().unwrap();
    let destination = args.destination.as_ref().unwrap();

    if !source.is_dir() {
        eprintln!("{}", msg.invalid_source);
        std::process::exit(1);
    }

    if !destination.exists() {
        fs::create_dir_all(&destination)?;
    }

    println!(
        "{}\n  {}\n{}\n  {}",
        msg.starting_backup,
        source.display(),
        msg.to,
        destination.display()
    );

    copy_incremental(source, destination, msg, args.show_graph)?;

    Ok(())
}
