use crate::output::print_message;
use crate::ui::draw_ui;
use serde::Deserialize;
use std::io::BufWriter;
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
    pub backup_init: String,
    pub backup_ended: String,
    pub starting_backup: String,
    pub to: String,
    pub copying_file: String,
    //    pub backup_done: String,
    pub invalid_source: String,
    pub language_not_supported: String,
    pub files_copied: String,
    pub files_skipped: String,
    pub copy_progress: String,
    pub copied_file: String,
    pub skipped_file: String,
    pub generic_error: String,
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
    logger: &Option<&Arc<Mutex<BufWriter<File>>>>,
    quiet: bool,
    with_timestamp: bool,
    progress_row: u16,
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

        print_message(
            "",
            logger,
            quiet,
            with_timestamp,
            Option::from(progress_row - 2),
            false,
        );
        let log_line = format!(
            "#{} {} {} - {}.",
            (copied.load(Ordering::SeqCst) + skipped.load(Ordering::SeqCst)) as f32,
            msg.copying_file,
            src_path.display(),
            status
        );
        print_message(
            &log_line,
            logger,
            quiet,
            with_timestamp,
            Option::from(progress_row - 3),
            true,
        );

        draw_ui(
            (copied.load(Ordering::SeqCst) + skipped.load(Ordering::SeqCst)) as f32,
            progress_row - 1,
            total_files as f32,
            msg,
        );
    }

    Ok((
        copied.load(Ordering::SeqCst),
        skipped.load(Ordering::SeqCst),
    ))
}
