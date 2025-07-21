use crate::ui::{draw_ui, print_above_progress};
use chrono::Local;
use serde::Deserialize;
use std::collections::HashMap;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread::sleep,
    time::Duration,
};
use terminal_size::{terminal_size, Height};
use walkdir::WalkDir;

#[derive(Deserialize)]
pub struct Messages {
    pub backup_init: String,
    pub backup_ended: String,
    pub starting_backup: String,
    pub to: String,
    pub copying_file: String,
    pub backup_done: String,
    pub invalid_source: String,
    pub language_not_supported: String,
    pub files_copied: String,
    pub files_skipped: String,
    pub copy_progress: String,
    pub file: String,
}

pub type Translations = HashMap<String, Messages>;
pub type Logger = Option<Arc<Mutex<File>>>;

pub fn now() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn log_output(msg: &str, logger: &Logger, quiet: bool, with_timestamp: bool) {
    let full_msg = if with_timestamp {
        format!("\n\n[{}] {}", now(), msg)
    } else {
        format!("{}", msg.to_string())
    };

    if !quiet {
        println!("{full_msg}");
    }

    if let Some(file) = logger {
        let mut file = file.lock().unwrap();
        writeln!(file, "{full_msg}").ok();
    }
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
) -> io::Result<(usize, usize)> {
    let entries: Vec<_> = WalkDir::new(src_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .collect();

    let copied = AtomicUsize::new(0);
    let skipped = AtomicUsize::new(0);
    let total_files = entries.len();

    let term_height = match terminal_size() {
        Some((_, Height(h))) => h,
        _ => 25,
    };

    let log_row = term_height.saturating_sub(3);
    let mut row = 0;

    entries.iter().enumerate().for_each(|(_i, entry)| {
        let src_path = entry.path();
        let rel_path = match src_path.strip_prefix(src_dir) {
            Ok(p) => p,
            Err(_) => return,
        };
        let dest_path = dest_dir.join(rel_path);

        let message = format!("{} {}", msg.copying_file, rel_path.display());
        print_above_progress(&message, row);

        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).ok();
        }

        match fs::copy(src_path, &dest_path) {
            Ok(_) => {
                copied.fetch_add(1, Ordering::SeqCst);
            }
            Err(_) => {
                skipped.fetch_add(1, Ordering::SeqCst);
            }
        }

        draw_ui(
            &format!("{}", rel_path.display()),
            copied.load(Ordering::SeqCst) as f32,
            total_files as f32,
            msg,
        );

        if row < log_row {
            row += 1;
        }

        sleep(Duration::from_millis(40));
    });

    Ok((
        copied.load(Ordering::SeqCst),
        skipped.load(Ordering::SeqCst),
    ))
}

/*pub fn test_ui_progress(msg: &Messages) {
    use crate::ui::{copy_ended, draw_ui};
    use crossterm::{execute, terminal::EnterAlternateScreen};
    use std::{io::stdout, thread::sleep, time::Duration};

    // Esempio di "file da copiare"
    let files = [
        "/home/user/Documents/report.pdf",
        "/home/user/Pictures/photo.jpg",
        "/home/user/Videos/video.mp4",
        "/home/user/Work/presentation.pptx",
        "/home/user/Backup/archive.zip",
    ];

    let total = files.len();

    // Entra in modalità schermo alternativo
    execute!(stdout(), EnterAlternateScreen).unwrap();
    let mut row = 0;

    for (i, file) in files.iter().enumerate() {
        let copied = i + 1;

        draw_ui(file, row, copied as f32, total as f32, msg);
        row += 1;
        sleep(Duration::from_millis(700));
    }

    // Fine
    copy_ended(row + 3, msg);
}*/
