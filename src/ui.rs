// ======== ui.rs ========
use crate::utils::Messages;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

pub fn copy_ended(row: u16, msg: &Messages) {
    execute!(
        stdout(),
        MoveTo(0, row),
        Clear(ClearType::FromCursorDown),
        Print(format!("{}\n", msg.backup_done)),
    )
    .unwrap();
}

pub fn draw_ui(file: &str, row: u16, copied: f32, total: f32, msg: &Messages) {
    let progress = copied / total;
    let percent = (progress * 100.0).round();

    let bar_width = 50;
    let filled = (progress * bar_width as f32).round() as usize;
    let empty = bar_width - filled;

    let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));

    let final_mex = format!(
        "{} {} {}/{} ({:.0}%) {}",
        msg.copy_progress, msg.file, copied, total, percent, bar
    );

    // Pulisci e stampa su 3 righe
    execute!(
        stdout(),
        MoveTo(0, row),
        Clear(ClearType::FromCursorDown),
        Print(format!("{} {}", msg.copying_file, file)),
        MoveTo(0, row + 2),
        Print(final_mex),
    )
    .unwrap();

    stdout().flush().unwrap();
}
