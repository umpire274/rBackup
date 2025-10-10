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
    let progress = copied / total;
    let percent = (progress * 100.0).round();

    let bar_width = 50;
    let filled = (progress * bar_width as f32).round() as usize;
    let empty = bar_width - filled;

    let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));
    let progress_line = format!(
        "{} file {}/{} ({:.0}%) {}",
        msg.copy_progress, copied as usize, total as usize, percent, bar
    );

    execute!(
        stdout(),
        MoveTo(0, progress_row),
        Clear(ClearType::CurrentLine),
        Print(progress_line),
        ResetColor
    )
    .unwrap();

    stdout().flush().unwrap();
}
