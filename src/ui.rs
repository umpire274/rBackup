use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

use crate::utils::Messages;

pub fn draw_ui(file: &str, copied: f32, total: f32, msg: &Messages) {
    let progress = copied / total;
    let percent = (progress * 100.0).round();

    let bar_width = 50;
    let filled = (progress * bar_width as f32).round() as usize;
    let empty = bar_width - filled;

    let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));

    let file_line = format!("{} {}", msg.copying_file, file);
    let progress_line = format!(
        "{}: file {}/{} ({:.0}%) {}",
        msg.copy_progress, copied as usize, total as usize, percent, bar
    );

    let (_cols, rows) = crossterm::terminal::size().unwrap_or((80, 24));
    let progress_row = rows.saturating_sub(1);
    let file_row = progress_row.saturating_sub(1);

    execute!(
        stdout(),
        MoveTo(0, file_row),
        Clear(ClearType::FromCursorDown),
        Print(file_line),
        MoveTo(0, progress_row),
        Clear(ClearType::CurrentLine),
        Print(progress_line),
    )
    .unwrap();

    stdout().flush().unwrap();
}
