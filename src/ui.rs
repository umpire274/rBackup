//! Terminal UI helpers used for displaying progress bars and simple status lines.
//!
//! This module contains a small helper to draw a file-level progress bar.

use crate::utils::Messages;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, ResetColor},
    terminal::{Clear, ClearType},
};
use std::io::{Write, stdout};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Draw a simple progress bar and file counter on the given terminal row.
///
/// # Parameters
/// - `copied`: number of items processed so far (as float to support partial updates)
/// - `progress_row`: terminal row where the progress bar will be drawn
/// - `total`: total number of items to process
/// - `msg`: localized message bundle used for the label
///
/// # Example
///
/// ```rust,ignore
/// // Example usage (ignored by doctest because it requires full `Messages` setup)
/// use rbackup::ui::draw_ui;
/// // prepare `msg` by loading translations in real code
/// // draw_ui(10.0, 20, 100.0, &msg);
/// ```
pub fn draw_ui(copied: f32, progress_row: u16, total: f32, msg: &Messages) {
    let progress = if total == 0.0 { 0.0 } else { copied / total };
    let percent = (progress * 100.0).round();

    // Determine terminal width to avoid producing lines longer than the
    // available space (which would wrap and push the progress row).
    let (cols, _rows) = crossterm::terminal::size().unwrap_or((80, 24));
    let cols = cols as usize;

    // Reserve some space for textual prefix and brackets; compute a
    // bar width that fits the terminal. Keep a sensible minimum.
    let reserved = 30; // approximate space for prefix (file counts and percent)
    let bar_width = if cols > reserved + 10 {
        std::cmp::min(50, cols - reserved)
    } else {
        10_usize
    };

    let filled = (progress * bar_width as f32).round() as usize;
    let empty = bar_width.saturating_sub(filled);

    let bar = format!(
        "[{}{}]",
        "\u{2588}".repeat(filled),
        "\u{2591}".repeat(empty)
    );
    let mut progress_line = format!(
        "{} file {}/{} ({:.0}%) {}",
        msg.copy_progress, copied as usize, total as usize, percent, bar
    );

    // Truncate progress line by display width to avoid terminal wrapping
    // which would push content below the bottom row.
    if cols >= 1 {
        let max = cols.saturating_sub(1);
        if UnicodeWidthStr::width(progress_line.as_str()) > max {
            progress_line = truncate_to_display_width(progress_line.as_str(), max);
        }
    }

    let _ = execute!(
        stdout(),
        MoveTo(0, progress_row),
        Clear(ClearType::CurrentLine),
        Print(progress_line),
        ResetColor
    );

    let _ = stdout().flush();
}

/// Truncate a string preserving Unicode character boundaries so its
/// displayed width does not exceed `max` columns.
fn truncate_to_display_width(s: &str, max: usize) -> String {
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
