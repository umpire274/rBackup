use chrono::Local;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use walkdir::WalkDir;

#[derive(Deserialize)]
pub struct Messages {
    pub starting_backup: String,
    pub to: String,
    pub copying_file: String,
    pub backup_done: String,
    pub invalid_source: String,
    pub language_not_supported: String,
    pub files_copied: String,
    pub files_skipped: String,
}

pub type Translations = HashMap<String, Messages>;
pub type Logger = Option<Arc<Mutex<File>>>;

pub fn now() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn log_output(msg: &str, logger: &Logger, quiet: bool, with_timestamp: bool) {
    let full_msg = if with_timestamp {
        format!("[{}] {}", now(), msg)
    } else {
        msg.to_string()
    };

    if !quiet {
        println!("{}", full_msg);
    }

    if let Some(file) = logger {
        let mut file = file.lock().unwrap();
        writeln!(file, "{}", full_msg).ok();
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
    let translations: Translations = serde_json::from_str(data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(translations)
}

pub fn copy_incremental(
    src_dir: &Path,
    dest_dir: &Path,
    msg: &Messages,
    show_graph: bool,
    logger: &Logger,
    quiet: bool,
    with_timestamp: bool,
) -> io::Result<(usize, usize)> {
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

    let copied = AtomicUsize::new(0);
    let skipped = AtomicUsize::new(0);

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

    entries.par_iter().for_each(|entry| {
        let src_path = entry.path();
        let rel_path = match src_path.strip_prefix(src_dir) {
            Ok(p) => p,
            Err(_) => return,
        };
        let dest_path = dest_dir.join(rel_path);

        if let Ok(newer) = is_newer(src_path, &dest_path) {
            if newer {
                if let Some(parent) = dest_path.parent() {
                    if let Err(e) = fs::create_dir_all(parent) {
                        log_output(
                            &format!("Error creating directory {}: {}", parent.display(), e),
                            logger,
                            quiet,
                            with_timestamp,
                        );
                        skipped.fetch_add(1, Ordering::SeqCst);
                        if let Some(ref pb) = pb {
                            pb.inc(1);
                        }
                        return;
                    }
                }

                match fs::copy(src_path, &dest_path) {
                    Ok(_) => {
                        copied.fetch_add(1, Ordering::SeqCst);
                        if let Some(ref pb) = pb {
                            pb.inc(1);
                        } else {
                            log_output(
                                &format!("{} {}", msg.copying_file, rel_path.display()),
                                logger,
                                quiet,
                                with_timestamp,
                            );
                        }
                    }
                    Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                        skipped.fetch_add(1, Ordering::SeqCst);
                        log_output(
                            &format!("Permission denied: {}", rel_path.display()),
                            logger,
                            quiet,
                            with_timestamp,
                        );
                        if let Some(ref pb) = pb {
                            pb.inc(1);
                        }
                    }
                    Err(e) => {
                        skipped.fetch_add(1, Ordering::SeqCst);
                        log_output(
                            &format!("Error copying {}: {}", rel_path.display(), e),
                            logger,
                            quiet,
                            with_timestamp,
                        );
                        if let Some(ref pb) = pb {
                            pb.inc(1);
                        }
                    }
                }
            } else {
                skipped.fetch_add(1, Ordering::SeqCst);
                if let Some(ref pb) = pb {
                    pb.inc(1);
                }
            }
        } else {
            skipped.fetch_add(1, Ordering::SeqCst);
            log_output(
                &format!("Error comparing {}: skipped", rel_path.display()),
                logger,
                quiet,
                with_timestamp,
            );
            if let Some(ref pb) = pb {
                pb.inc(1);
            }
        }
    });

    if let Some(pb) = pb {
        pb.finish_with_message(msg.backup_done.clone());
    } else {
        log_output(&msg.backup_done, logger, quiet, with_timestamp);
    }

    Ok((copied.load(Ordering::SeqCst), skipped.load(Ordering::SeqCst)))
}

pub fn test_ui_progress() {
    use crossterm::{
        execute,
        terminal::EnterAlternateScreen,
    };
    use std::{io::stdout, thread::sleep, time::Duration};
    use crate::ui::{draw_ui, copy_ended};

    // Esempio di "file da copiare"
    let files = vec![
        "/home/user/Documents/report.pdf",
        "/home/user/Pictures/photo.jpg",
        "/home/user/Videos/video.mp4",
        "/home/user/Work/presentation.pptx",
        "/home/user/Backup/archive.zip",
    ];

    let total = files.len();

    // Entra in modalità schermo alternativo
    execute!(stdout(), EnterAlternateScreen).unwrap();
    let mut row=0;

    for (i, file) in files.iter().enumerate() {
        let copied = i + 1;

        draw_ui(file, row, copied as f32, total as f32);
        row += 1;
        sleep(Duration::from_millis(700));
    }

    // Fine
    copy_ended(row+3);
}
