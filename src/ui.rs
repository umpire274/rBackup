use crate::utils::Messages;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, ResetColor},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

pub fn draw_ui(copied: f32, progress_row: u16, total: f32, msg: &Messages) {
    let progress = copied / total;
    let percent = (progress * 100.0).round();

    let bar_width = 50;
    let filled = (progress * bar_width as f32).round() as usize;
    let empty = bar_width - filled;

    let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));
    let progress_line = format!(
        "{}: file {}/{} ({:.0}%) {}",
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
