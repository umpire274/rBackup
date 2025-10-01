use crate::output::{LogContext, log_output};
use crate::ui::draw_ui;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use globset::{Glob, GlobBuilder, GlobSet, GlobSetBuilder};
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
    pub files_total: String,
    pub files_copied: String,
    pub files_skipped: String,
    pub copy_progress: String,
    pub copied_file: String,
    pub skipped_file: String,
    pub generic_error: String,
    pub error_exclude_parsing: String,
}

pub type Logger = Arc<Mutex<BufWriter<File>>>;

pub fn clear_terminal() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

// Now returns io::Result<Option<Logger>> instead of panicking
pub fn create_logger(path: Option<&Path>) -> io::Result<Option<Logger>> {
    if let Some(p) = path {
        let file = File::create(p)?;
        Ok(Some(Arc::new(Mutex::new(BufWriter::new(file)))))
    } else {
        Ok(None)
    }
}

pub type Translations = HashMap<String, Messages>;

pub fn load_translations() -> io::Result<Translations> {
    let data = include_str!("../assets/translations.json");
    let translations: Translations =
        serde_json::from_str(data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(translations)
}

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

pub fn copy_incremental(
    src_dir: &Path,
    dest_dir: &Path,
    msg: &Messages,
    options: &LogContext,
) -> io::Result<(usize, usize)> {
    // Count total files in a first streaming pass (no Vec allocation)
    let total_files = WalkDir::new(src_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .count();

    let copied = AtomicUsize::new(0);
    let skipped = AtomicUsize::new(0);

    // Second streaming pass to perform the copy
    for entry in WalkDir::new(src_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
    {
        let src_path = entry.path();
        let rel_path = match src_path.strip_prefix(src_dir) {
            Ok(p) => p,
            Err(_) => continue,
        };

        // Apply 'exclude matcher' on path relative to src_dir (or absolute if requested)
        if let Some(ex) = &options.exclude_matcher {
            // Determine target to match and ask ExcludeMatcher which pattern matched (if any)
            let target_path = if options.exclude_match_absolute {
                src_path
            } else {
                rel_path
            };
            // also try basename if no match yet
            let matched_pattern = ex.is_match(target_path).or_else(|| {
                rel_path
                    .file_name()
                    .and_then(|name| ex.is_match(Path::new(name)))
            });

            if let Some(pat) = matched_pattern {
                skipped.fetch_add(1, Ordering::SeqCst);

                // Log skipped (excluded) file similar to other skipped cases, include pattern
                let skip_ctx = LogContext {
                    logger: options.logger.clone(),
                    quiet: options.quiet,
                    with_timestamp: options.with_timestamp,
                    timestamp_format: options.timestamp_format.clone(),
                    row: options.row.map(|r| r.saturating_sub(3)),
                    on_log: true,
                    exclude_matcher: None,
                    exclude_match_absolute: options.exclude_match_absolute,
                    dry_run: options.dry_run,
                    exclude_patterns: options.exclude_patterns.clone(),
                };

                // blank line (to keep UI clean)
                let mut blank_ctx = skip_ctx.clone();
                blank_ctx.with_timestamp = false;
                blank_ctx.on_log = false;
                blank_ctx.row = options.row.map(|r| r.saturating_sub(2));
                log_output("", &blank_ctx);

                let log_line = format!(
                    "#{} {} {} - {} (pattern: {}).",
                    (copied.load(Ordering::SeqCst) + skipped.load(Ordering::SeqCst)) as f32,
                    msg.copying_file,
                    src_path.display(),
                    &msg.skipped_file,
                    pat
                );

                log_output(&log_line, &skip_ctx);

                // Update UI
                let total_f = if total_files == 0 {
                    1.0
                } else {
                    total_files as f32
                };
                draw_ui(
                    (copied.load(Ordering::SeqCst) + skipped.load(Ordering::SeqCst)) as f32,
                    options.row.unwrap_or(1).saturating_sub(1),
                    total_f,
                    msg,
                );

                continue;
            }
        }

        let dest_path = dest_dir.join(rel_path);

        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).ok();
        }

        let status = match is_newer(src_path, &dest_path) {
            Ok(true) => {
                if options.dry_run {
                    // In dry-run mode, don't actually copy
                    copied.fetch_add(1, Ordering::SeqCst);
                    &msg.copied_file
                } else {
                    match fs::copy(src_path, &dest_path) {
                        Ok(_) => {
                            copied.fetch_add(1, Ordering::SeqCst);
                            &msg.copied_file
                        }
                        Err(_) => {
                            skipped.fetch_add(1, Ordering::SeqCst);
                            &msg.skipped_file
                        }
                    }
                }
            }
            Ok(false) | Err(_) => {
                skipped.fetch_add(1, Ordering::SeqCst);
                &msg.skipped_file
            }
        };

        // Build a minimal temporary LogContext to avoid cloning the whole options
        let mut tmp_ctx = LogContext {
            logger: options.logger.clone(),
            quiet: options.quiet,
            with_timestamp: false,
            timestamp_format: options.timestamp_format.clone(),
            row: options.row.map(|r| r.saturating_sub(2)),
            on_log: false,
            exclude_matcher: None,
            // propagate new fields
            exclude_match_absolute: options.exclude_match_absolute,
            dry_run: options.dry_run,
            exclude_patterns: options.exclude_patterns.clone(),
        };

        log_output("", &tmp_ctx);

        let log_line = format!(
            "#{} {} {} - {}.",
            (copied.load(Ordering::SeqCst) + skipped.load(Ordering::SeqCst)) as f32,
            msg.copying_file,
            src_path.display(),
            status
        );

        tmp_ctx.with_timestamp = options.with_timestamp;
        tmp_ctx.row = options.row.map(|r| r.saturating_sub(3));
        tmp_ctx.on_log = true;
        log_output(&log_line, &tmp_ctx);

        let total_f = if total_files == 0 {
            1.0
        } else {
            total_files as f32
        };
        draw_ui(
            (copied.load(Ordering::SeqCst) + skipped.load(Ordering::SeqCst)) as f32,
            options.row.unwrap_or(1).saturating_sub(1),
            total_f,
            msg,
        );
    }

    Ok((
        copied.load(Ordering::SeqCst),
        skipped.load(Ordering::SeqCst),
    ))
}

#[derive(Clone, Debug)]
pub struct ExcludeMatcher {
    pub combined: GlobSet,
    // store pairs (pattern, GlobSet for single pattern) to find which pattern matched
    pub singles: Vec<(String, GlobSet)>,
}

impl ExcludeMatcher {
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
