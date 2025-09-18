use crate::ui::draw_ui;
use chrono::Local;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::{Print, ResetColor};
use crossterm::terminal::{Clear, ClearType};
use serde::Deserialize;
use std::io::stdout;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
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
}

pub type Translations = HashMap<String, Messages>;
pub type Logger = Option<Arc<Mutex<File>>>;

pub fn now() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn log_output(msg: &str, logger: &Logger, quiet: bool, with_timestamp: bool, file: bool) {
    let full_msg = if with_timestamp {
        format!("[{}] {}", now(), msg)
    } else {
        msg.to_string()
    };

    if !quiet && !file {
        println!("{full_msg}");
    }

    if let Some(file) = logger {
        let mut file = file.lock().unwrap();
        writeln!(file, "{full_msg}").ok();
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

pub fn load_translations() -> io::Result<Translations> {
    let data = include_str!("../assets/translations.json");
    let translations: Translations =
        serde_json::from_str(data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(translations)
}

pub fn copy_incremental(
    src_dir: &Path,
    dest_dir: &Path,
    msg: &Messages,
    logger: &Logger,
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

        execute!(
            stdout(),
            MoveTo(0, progress_row - 2),
            Clear(ClearType::CurrentLine),
            ResetColor
        )?;

        let log_line = format!("{} {} - {}.", msg.copying_file, src_path.display(), status);

        execute!(
            stdout(),
            MoveTo(0, progress_row - 3),
            Clear(ClearType::CurrentLine),
            Print(&log_line),
            ResetColor
        )?;

        // print the copy message on the log file if requested
        log_output(&log_line, logger, quiet, with_timestamp, true);

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
