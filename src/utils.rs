use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Deserialize)]
pub struct Messages {
    pub starting_backup: String,
    pub to: String,
    pub copying_file: String,
    pub backup_done: String,
    pub invalid_source: String,
    pub language_not_supported: String,
}

pub type Translations = HashMap<String, Messages>;
pub type Logger = Option<Arc<Mutex<File>>>;

pub fn load_translations() -> io::Result<Translations> {
    let data = include_str!("../assets/translations.json");
    let translations: Translations = serde_json::from_str(data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(translations)
}

pub fn log_output(msg: &str, logger: &Logger, quiet: bool) {
    if !quiet {
        println!("{}", msg);
    }
    if let Some(file) = logger {
        let mut file = file.lock().unwrap();
        writeln!(file, "{}", msg).ok();
    }
}

fn is_newer(src: &Path, dest: &Path) -> io::Result<bool> {
    if !dest.exists() {
        return Ok(true);
    }
    let src_meta = src.metadata()?;
    let dest_meta = dest.metadata()?;
    Ok(src_meta.modified()? > dest_meta.modified()?)
}

pub fn copy_incremental(
    src_dir: &Path,
    dest_dir: &Path,
    msg: &Messages,
    show_graph: bool,
    logger: &Logger,
    quiet: bool,
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
            ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}")
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
                log_output(
                    &format!("{} {}", msg.copying_file, rel_path.display()),
                    logger,
                    quiet,
                );
            }
        } else if let Some(ref pb) = pb {
            pb.inc(1);
        }

        Ok(())
    })?;

    if let Some(pb) = pb {
        pb.finish_with_message(msg.backup_done.clone());
    } else {
        log_output(&msg.backup_done, logger, quiet);
    }

    Ok(())
}
