//! Utility functions and shared types used across the crate.
//!
//! This module contains localization message structures, logging helpers,
//! translation loading, file comparison helpers and the core incremental
//! copying implementation. Public items are documented with examples where
//! relevant.

use crate::output::LogContext;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use globset::{Glob, GlobBuilder, GlobSet, GlobSetBuilder};
use rayon::prelude::*; // parallel iterator utilities
use serde::Deserialize;
use std::collections::VecDeque;
use std::io::{BufWriter, Write, stdout};
use std::sync::mpsc;
use std::thread;
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
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
use walkdir::WalkDir; // added for UI scroll buffer

/// Truncate a string so that its displayed width (in monospace columns)
/// does not exceed `max`. This preserves Unicode character boundaries.
fn truncate_to_width(s: &str, max: usize) -> String {
    if max == 0 {
        return String::new();
    }
    let mut out = String::new();
    let mut width = 0usize;
    for c in s.chars() {
        let w = UnicodeWidthChar::width(c).unwrap_or(0);
        if width + w > max {
            break;
        }
        out.push(c);
        width += w;
    }
    out
}

/// Events sent to the single UI thread to serialize terminal access.
///
/// Message: a textual line to append to the scrollable area.
/// Progress: update the fixed progress bar displayed on the bottom row.
#[derive(Debug)]
enum UiEvent {
    Message(String),
    Progress { copied: f32, total: f32 },
}

/// Localizable messages loaded from `assets/translations.json`.
///
/// The fields map to the string keys used in the translations bundle. This
/// struct is deserialized automatically by `serde`.
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
    pub files_total: String,
    pub files_copied: String,
    pub files_skipped: String,
    pub copy_progress: String,
    pub copied_file: String,
    pub skipped_file: String,
    pub generic_error: String,
    pub error_exclude_parsing: String,
}

/// Thread-safe file logger type: `Arc<Mutex<BufWriter<File>>>`.
///
/// Use `create_logger` to build this type from an optional path.
pub type Logger = Arc<Mutex<BufWriter<File>>>;

/// Clear the entire terminal screen.
///
/// A tiny helper wrapping `crossterm` functionality.
pub fn clear_terminal() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

/// Create an optional logger for the provided path.
///
/// Returns `Ok(Some(Logger))` when a file was created, `Ok(None)` when no
/// path was provided, or an `Err` when the file could not be created.
///
/// # Example
///
/// ```rust
/// use rbackup::utils::create_logger;
/// let logger = create_logger(None).unwrap(); // returns Ok(None)
/// ```
pub fn create_logger(path: Option<&Path>) -> io::Result<Option<Logger>> {
    if let Some(p) = path {
        let file = File::create(p)?;
        Ok(Some(Arc::new(Mutex::new(BufWriter::new(file)))))
    } else {
        Ok(None)
    }
}

/// Type alias for the translations map loaded from JSON.
pub type Translations = HashMap<String, Messages>;

/// Load the embedded translations JSON and deserialize it.
///
/// The translations file is embedded at compile-time using `include_str!` and
/// then parsed as JSON. Returns `io::Error` if parsing fails.
pub fn load_translations() -> io::Result<Translations> {
    let data = include_str!("../assets/translations.json");
    let translations: Translations =
        serde_json::from_str(data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(translations)
}

/// Return true if `src` is newer than `dest` (based on filesystem modification time).
///
/// If the destination does not exist, this returns `Ok(true)`. If either path
/// cannot be stat-ed for other reasons, the function returns `Ok(false)` or
/// an `Err` depending on the underlying `fs::metadata` error.
pub fn is_newer(src: &Path, dest: &Path) -> io::Result<bool> {
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

/// Perform an incremental copy from `src_dir` to `dest_dir`.
///
/// The function walks the source directory recursively, applies the optional
/// exclude matcher (if present in `options`), copies files that are newer
/// (or missing) and updates simple progress output via `LogContext` and the
/// `ui::draw_ui` helper.
///
/// Returns a tuple `(copied_count, skipped_count)` on success.
///
/// # Parameters
/// - `src_dir`: source directory path
/// - `dest_dir`: destination directory path
/// - `msg`: localized messages used for output lines
/// - `options`: `LogContext` carrying runtime options like logger, dry-run, etc.
///
/// # Errors
/// Returns an `io::Error` if listing directories or reading/writing files fails
/// in an unexpected way.
///
/// # Example (high level)
///
/// ```rust,no_run
/// use std::path::Path;
/// use rbackup::{copy_incremental, Messages, LogContext};
///
/// // Prepare placeholders (in real code load translations and build a LogContext)
/// let msg: Messages = serde_json::from_str("{}") /* load proper messages */ .unwrap_or_else(|_| panic!());
/// let ctx = LogContext { logger: None, quiet: false, with_timestamp: false, timestamp_format: None, row: None, on_log: true, exclude_matcher: None, exclude_match_absolute: false, dry_run: true, exclude_patterns: None };
///
/// // Run a dry-run copy (will not actually copy files because dry_run = true)
/// let (copied, skipped) = copy_incremental(Path::new("/tmp/src"), Path::new("/tmp/dest"), &msg, &ctx).unwrap();
/// println!("copied={}, skipped={}", copied, skipped);
/// ```
pub fn copy_incremental(
    src_dir: &Path,
    dest_dir: &Path,
    msg: &Messages,
    options: &LogContext,
) -> io::Result<(usize, usize)> {
    // Collect all file entries in a single pass to avoid walking the tree twice.
    // This trades memory for reduced filesystem traversal overhead which is
    // beneficial for very large hierarchies where walking twice is expensive.
    let mut entries: Vec<_> = WalkDir::new(src_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .collect();

    let total_files = entries.len();

    // Prepare UI channel and spawn a dedicated UI thread that owns all
    // terminal writes. This prevents concurrent calls to crossterm from
    // multiple worker threads which produced garbled output.
    let (ui_tx, ui_handle) = {
        let (tx, rx) = mpsc::channel::<UiEvent>();
        // Clone all strings we need from `msg` into an owned `Messages`
        // instance that can be used inside the UI thread.
        let ui_msg = Messages {
            cur_conf: msg.cur_conf.clone(),
            conf_file_not_found: msg.conf_file_not_found.clone(),
            conf_initialized: msg.conf_initialized.clone(),
            backup_init: msg.backup_init.clone(),
            backup_ended: msg.backup_ended.clone(),
            starting_backup: msg.starting_backup.clone(),
            to: msg.to.clone(),
            copying_file: msg.copying_file.clone(),
            language_not_supported: msg.language_not_supported.clone(),
            files_total: msg.files_total.clone(),
            files_copied: msg.files_copied.clone(),
            files_skipped: msg.files_skipped.clone(),
            copy_progress: msg.copy_progress.clone(),
            copied_file: msg.copied_file.clone(),
            skipped_file: msg.skipped_file.clone(),
            generic_error: msg.generic_error.clone(),
            error_exclude_parsing: msg.error_exclude_parsing.clone(),
        };

        let handle = thread::spawn(move || {
            use crossterm::cursor::MoveTo;
            use crossterm::execute;
            use crossterm::style::Print;
            use crossterm::terminal;
            use crossterm::terminal::ClearType;
            use std::io::stdout;

            let mut buffer: VecDeque<String> = VecDeque::new();

            // Helper to redraw the scrollable area above the progress bar.
            // This truncates messages to the terminal width to avoid wrapping
            // onto the progress bar row and pads with spaces to clear leftover
            // characters from previous longer lines.
            let redraw = |buf: &VecDeque<String>, cols: u16, scroll_rows: usize| {
                // Use cols-1 as maximum printable content width to avoid terminals
                // that wrap when writing exactly the last column. Clear each line
                // first and then print at most `max_content` display columns.
                if cols == 0 {
                    return;
                }
                let max_content = cols.saturating_sub(1) as usize;

                for i in 0..scroll_rows {
                    if i < buf.len() {
                        // Truncate by display width, then pad with spaces to clear
                        // leftover characters from previous content.
                        let mut s = truncate_to_width(&buf[i], max_content);
                        let disp = UnicodeWidthStr::width(s.as_str());
                        if disp < max_content {
                            s.push_str(&" ".repeat(max_content - disp));
                        }
                        let _ = execute!(
                            stdout(),
                            MoveTo(0, i as u16),
                            Clear(ClearType::CurrentLine),
                            Print(s)
                        );
                    } else {
                        // clear the line if there's no content for that row
                        let _ =
                            execute!(stdout(), MoveTo(0, i as u16), Clear(ClearType::CurrentLine));
                    }
                }
            };

            // initial draw: clear whole screen
            let _ = execute!(stdout(), Clear(ClearType::All));

            for ev in rx {
                // recompute terminal size on each event to handle resizes
                let (cols, rows) = terminal::size().unwrap_or((80, 24));
                let progress_row = rows.saturating_sub(1);
                let scroll_rows = progress_row as usize; // number of rows for scrolling area

                // adjust buffer size if terminal height changed
                while buffer.len() > scroll_rows {
                    buffer.pop_front();
                }

                match ev {
                    UiEvent::Message(s) => {
                        if scroll_rows == 0 {
                            // no space to render messages; just ignore
                            continue;
                        }
                        buffer.push_back(s);
                        if buffer.len() > scroll_rows {
                            buffer.pop_front();
                        }
                        redraw(&buffer, cols, scroll_rows);
                    }
                    UiEvent::Progress { copied, total } => {
                        // draw progress bar on the current bottom row
                        crate::ui::draw_ui(copied, progress_row, total, &ui_msg);
                    }
                }
            }
        });

        (tx, handle)
    };

    // Prepare channel and logger thread if a file logger is configured.
    let (log_tx, log_handle) = if let Some(logger) = &options.logger {
        let (tx, rx) = mpsc::channel::<String>();
        let logger = logger.clone();
        // Spawn a dedicated thread that serializes writes to the log file.
        let handle = thread::spawn(move || {
            for line in rx {
                match logger.lock() {
                    Ok(mut guard) => {
                        let _ = writeln!(guard, "{}", line);
                        let _ = guard.flush();
                    }
                    Err(poisoned) => {
                        if let Ok(mut guard) = std::panic::catch_unwind(|| poisoned.into_inner()) {
                            let _ = writeln!(guard, "{}", line);
                            let _ = guard.flush();
                        }
                    }
                }
            }
        });
        (Some(tx), Some(handle))
    } else {
        (None, None)
    };

    let copied = AtomicUsize::new(0);
    let skipped = AtomicUsize::new(0);

    // Sort entries deterministically to improve cache behaviour and make output stable.
    entries.sort_by_key(|e| e.path().to_owned());

    // Use parallel iterator over entries to perform copies concurrently.
    // keep the UI sender as an `Option<mpsc::Sender<UiEvent>>` so worker
    // threads can check its presence before sending.
    let ui_tx: Option<mpsc::Sender<UiEvent>> = Some(ui_tx);
    entries.into_par_iter().for_each(|entry| {
        let src_path = entry.path().to_owned();
        let rel_path = match src_path.strip_prefix(src_dir) {
            Ok(p) => p.to_owned(),
            Err(_) => return,
        };

        // Apply 'exclude matcher' on path relative to src_dir (or absolute if requested)
        if let Some(ex) = &options.exclude_matcher {
            let target_path = if options.exclude_match_absolute {
                src_path.as_path()
            } else {
                rel_path.as_path()
            };
            let matched_pattern = ex.is_match(target_path).or_else(|| {
                rel_path
                    .file_name()
                    .and_then(|name| ex.is_match(Path::new(name)))
            });

            if let Some(pat) = matched_pattern {
                skipped.fetch_add(1, Ordering::Relaxed);

                // Build log line; send to file logger via channel if configured
                let count =
                    (copied.load(Ordering::Relaxed) + skipped.load(Ordering::Relaxed)) as f32;
                let log_line = format!(
                    "#{} {} {} - {} (pattern: {}).",
                    count,
                    msg.copying_file,
                    src_path.display(),
                    &msg.skipped_file,
                    pat
                );

                if let Some(tx) = &log_tx {
                    // Prepend timestamp if requested
                    let full = if options.with_timestamp && !log_line.trim().is_empty() {
                        let fmt = options
                            .timestamp_format
                            .as_deref()
                            .unwrap_or("%Y-%m-%d %H:%M:%S");
                        format!("[{}] {}", crate::output::now(fmt), log_line)
                    } else {
                        log_line.clone()
                    };
                    let _ = tx.send(full);
                }

                // Send message to UI thread for terminal rendering (scroll area)
                if let Some(tx) = &ui_tx {
                    let _ = tx.send(UiEvent::Message(log_line.clone()));
                }

                // Update UI occasionally to reduce contention on terminal
                let total_f = if total_files == 0 {
                    1.0
                } else {
                    total_files as f32
                };
                let progress =
                    (copied.load(Ordering::Relaxed) + skipped.load(Ordering::Relaxed)) as f32;
                if ((progress as usize).is_multiple_of(16) || (progress as usize) == total_files)
                    && let Some(tx) = &ui_tx
                {
                    let _ = tx.send(UiEvent::Progress {
                        copied: progress,
                        total: total_f,
                    });
                }

                return;
            }
        }

        let dest_path = dest_dir.join(&rel_path);

        if let Some(parent) = dest_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let status = match is_newer(src_path.as_path(), &dest_path) {
            Ok(true) => {
                if options.dry_run {
                    copied.fetch_add(1, Ordering::Relaxed);
                    &msg.copied_file
                } else {
                    match fs::copy(src_path.as_path(), &dest_path) {
                        Ok(_) => {
                            copied.fetch_add(1, Ordering::Relaxed);
                            &msg.copied_file
                        }
                        Err(_) => {
                            skipped.fetch_add(1, Ordering::Relaxed);
                            &msg.skipped_file
                        }
                    }
                }
            }
            Ok(false) | Err(_) => {
                skipped.fetch_add(1, Ordering::Relaxed);
                &msg.skipped_file
            }
        };

        // Build log line for both file logger and UI
        let count = (copied.load(Ordering::Relaxed) + skipped.load(Ordering::Relaxed)) as f32;
        let log_line = format!(
            "#{} {} {} - {}.",
            count,
            msg.copying_file,
            src_path.display(),
            status
        );

        // Send to file logger thread if present
        if let Some(tx) = &log_tx {
            let full = if options.with_timestamp && !log_line.trim().is_empty() {
                let fmt = options
                    .timestamp_format
                    .as_deref()
                    .unwrap_or("%Y-%m-%d %H:%M:%S");
                format!("[{}] {}", crate::output::now(fmt), log_line)
            } else {
                log_line.clone()
            };
            let _ = tx.send(full);
        }

        // Send message to UI thread (terminal scroll area)
        if let Some(tx) = &ui_tx {
            let _ = tx.send(UiEvent::Message(log_line.clone()));
        }

        let total_f = if total_files == 0 {
            1.0
        } else {
            total_files as f32
        };
        let progress = (copied.load(Ordering::Relaxed) + skipped.load(Ordering::Relaxed)) as f32;
        if ((progress as usize).is_multiple_of(16) || (progress as usize) == total_files)
            && let Some(tx) = &ui_tx
        {
            let _ = tx.send(UiEvent::Progress {
                copied: progress,
                total: total_f,
            });
        }
    });

    // Close the UI channel and join the UI thread
    drop(ui_tx);
    let _ = ui_handle.join();

    // Close the logger channel and join the logger thread if it was spawned
    drop(log_tx);
    if let Some(handle) = log_handle {
        let _ = handle.join();
    }

    Ok((
        copied.load(Ordering::Relaxed),
        skipped.load(Ordering::Relaxed),
    ))
}

/// Pattern matcher for exclude lists.
///
/// The matcher holds a combined `GlobSet` for fast checking and a vector of
/// single-pattern `GlobSet`s so the code can determine which pattern matched
/// (useful for logging the pattern that caused a skip).
#[derive(Clone, Debug)]
pub struct ExcludeMatcher {
    pub combined: GlobSet,
    // store pairs (pattern, GlobSet for single pattern) to find which pattern matched
    pub singles: Vec<(String, GlobSet)>,
}

impl ExcludeMatcher {
    /// Return the pattern that matched `path`, or `None` if no pattern matched.
    ///
    /// If the combined set matches but none of the single-pattern sets match
    /// (unlikely), the function returns the first pattern as a fallback.
    pub fn is_match(&self, path: &Path) -> Option<&str> {
        if self.combined.is_match(path) {
            for (pat, gs) in &self.singles {
                if gs.is_match(path) {
                    return Some(pat.as_str());
                }
            }
            // fallback: if combined matched but no single found, return first pattern
            self.singles.first().map(|(p, _)| p.as_str())
        } else {
            None
        }
    }
}

/// Build an `ExcludeMatcher` from a list of glob patterns.
///
/// The `case_insensitive` flag controls whether the globs are built in a
/// case-insensitive manner. Returns an I/O-like error when a pattern cannot
/// be parsed to keep the API consistent with other file operations.
///
/// # Example
///
/// ```rust
/// use rbackup::utils::build_exclude_matcher;
/// use std::path::Path;
/// let patterns = vec!["*.tmp".to_string(), "target".to_string()];
/// let matcher = build_exclude_matcher(&patterns, true).unwrap();
/// assert!(matcher.is_match(Path::new("foo.tmp")).is_some());
/// ```
pub fn build_exclude_matcher(
    patterns: &[String],
    case_insensitive: bool,
) -> io::Result<ExcludeMatcher> {
    let mut combined_builder = GlobSetBuilder::new();
    let mut singles: Vec<(String, GlobSet)> = Vec::new();

    for pattern in patterns {
        // build Glob with optional case insensitivity
        let glob = if case_insensitive {
            GlobBuilder::new(pattern)
                .case_insensitive(true)
                .build()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?
        } else {
            Glob::new(pattern).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?
        };

        // add to combined
        combined_builder.add(glob.clone());

        // build single-set for pattern to identify which pattern matched
        let mut single_builder = GlobSetBuilder::new();
        single_builder.add(glob);
        let single_set = single_builder
            .build()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        singles.push((pattern.clone(), single_set));
    }

    let combined = combined_builder
        .build()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    Ok(ExcludeMatcher { combined, singles })
}
