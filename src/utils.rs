use crate::output::log_output;
use crate::ui::draw_ui;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::Deserialize;
use std::io::{BufWriter, stdout};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self},
    path::Path,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};
use walkdir::WalkDir;

#[derive(Deserialize)]
pub struct Messages {
    pub cur_conf: String,
    pub conf_file_not_found: String,
    pub conf_initialized: String,
    pub backup_init: String,
    pub backup_ended: String,
    pub starting_backup: String,
    pub to: String,
    pub copying_file: String,
    //    pub invalid_source: String,
    pub language_not_supported: String,
    pub files_copied: String,
    pub files_skipped: String,
    pub copy_progress: String,
    pub copied_file: String,
    pub skipped_file: String,
    pub generic_error: String,
    pub error_exclude_parsing: String,
}

pub fn clear_terminal() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

/// Raggruppa le opzioni di copia.
pub struct CopyOptions<'a> {
    pub logger: &'a Option<Arc<Mutex<BufWriter<File>>>>,
    pub quiet: bool,
    pub timestamp: bool,
    pub exclude_matcher: Option<GlobSet>,
    pub progress_row: u16,
}

pub type Translations = HashMap<String, Messages>;

pub fn load_translations() -> io::Result<Translations> {
    let data = include_str!("../assets/translations.json");
    let translations: Translations =
        serde_json::from_str(data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(translations)
}

fn is_newer(src: &Path, dest: &Path) -> io::Result<bool> {
    match (fs::metadata(src), fs::metadata(dest)) {
        (Ok(src_meta), Ok(dest_meta)) => {
            let src_time = src_meta.modified()?;
            let dest_time = dest_meta.modified()?;
            Ok(src_time > dest_time)
        }
        (Ok(_), Err(_)) => Ok(true),
        _ => Ok(false),
    }
}

pub fn copy_incremental(
    src_dir: &Path,
    dest_dir: &Path,
    msg: &Messages,
    options: &CopyOptions,
) -> io::Result<(usize, usize)> {
    let entries: Vec<_> = WalkDir::new(src_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .collect();

    let copied = AtomicUsize::new(0);
    let skipped = AtomicUsize::new(0);
    let total_files = entries.len();

    for entry in entries {
        let src_path = entry.path();
        if let Some(ex) = &options.exclude_matcher
            && ex.is_match(src_path)
        {
            skipped.fetch_add(1, Ordering::SeqCst);
            continue;
        }
        let rel_path = match src_path.strip_prefix(src_dir) {
            Ok(p) => p,
            Err(_) => continue,
        };
        let dest_path = dest_dir.join(rel_path);

        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).ok();
        }

        let status = match is_newer(src_path, &dest_path) {
            Ok(true) => match fs::copy(src_path, &dest_path) {
                Ok(_) => {
                    copied.fetch_add(1, Ordering::SeqCst);
                    &msg.copied_file
                }
                Err(_) => {
                    skipped.fetch_add(1, Ordering::SeqCst);
                    &msg.skipped_file
                }
            },
            Ok(false) | Err(_) => {
                skipped.fetch_add(1, Ordering::SeqCst);
                &msg.skipped_file
            }
        };

        log_output(
            "",
            options.logger,
            options.quiet,
            options.timestamp,
            Option::from(options.progress_row - 2),
            false,
        );
        let log_line = format!(
            "#{} {} {} - {}.",
            (copied.load(Ordering::SeqCst) + skipped.load(Ordering::SeqCst)) as f32,
            msg.copying_file,
            src_path.display(),
            status
        );
        log_output(
            &log_line,
            options.logger,
            options.quiet,
            options.timestamp,
            Option::from(options.progress_row - 3),
            true,
        );

        draw_ui(
            (copied.load(Ordering::SeqCst) + skipped.load(Ordering::SeqCst)) as f32,
            options.progress_row - 1,
            total_files as f32,
            msg,
        );
    }

    Ok((
        copied.load(Ordering::SeqCst),
        skipped.load(Ordering::SeqCst),
    ))
}

pub fn build_exclude_matcher(patterns: &[String]) -> io::Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        builder
            .add(Glob::new(pattern).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?);
    }
    builder
        .build()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}
