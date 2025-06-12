use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
#[warn(unused_imports)]
use rayon::prelude::*;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Source directory
    source: PathBuf,

    /// Destination directory
    destination: PathBuf,

    /// Language code: en or it (default: en)
    #[arg(short, long, default_value = "auto")]
    lang: String,
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

fn copy_incremental(src_dir: &Path, dest_dir: &Path, msg: &Messages) -> io::Result<()> {
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

    entries.par_iter().try_for_each(|entry| -> io::Result<()> {
        let src_path = entry.path();
        let rel_path = src_path.strip_prefix(src_dir).unwrap();
        let dest_path = dest_dir.join(rel_path);

        if is_newer(src_path, &dest_path)? {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            println!("{} {}", msg.copying_file, rel_path.display());
            fs::copy(src_path, dest_path)?;
        }

        Ok(())
    })
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

    if !args.source.is_dir() {
        eprintln!("{}", msg.invalid_source);
        std::process::exit(1);
    }

    if !args.destination.exists() {
        fs::create_dir_all(&args.destination)?;
    }

    println!(
        "{}\n  {}\n{}\n  {}",
        msg.starting_backup,
        args.source.display(),
        msg.to,
        args.destination.display()
    );

    copy_incremental(&args.source, &args.destination, msg)?;

    println!("{}", msg.backup_done);
    Ok(())
}
