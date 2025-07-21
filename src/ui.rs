use crate::utils::Messages;
use crossterm::{
    cursor::{MoveTo, RestorePosition, SavePosition},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

pub fn draw_ui(file: &str, copied: f32, total: f32, msg: &Messages) {
    let progress = copied / total;
    let percent = (progress * 100.0).round();

    let bar_width = 50;
    let filled = (progress * bar_width as f32).round() as usize;
    let empty = bar_width - filled;

    let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));

    // Stampa normale del messaggio "Copio: ..." con println!
    println!("{} {}", msg.copying_file, file);

    // Salva posizione corrente del cursore
    let mut stdout = stdout();
    execute!(stdout, SavePosition).unwrap();

    // Sposta il cursore in basso (ultima riga), cancella e stampa la barra
    let (_cols, rows) = crossterm::terminal::size().unwrap();
    execute!(
        stdout,
        MoveTo(0, rows - 1),
        Clear(ClearType::CurrentLine),
        Print(format!(
            "{} file {}/{} ({}%) {}",
            msg.copy_progress, copied as u32, total as u32, percent, bar
        )),
        RestorePosition
    )
    .unwrap();

    stdout.flush().unwrap();
}

pub fn copy_ended(msg: &Messages) {
    println!("\n{}", msg.backup_done);
}

pub fn print_above_progress(message: &str, line: u16) {
    execute!(
        stdout(),
        MoveTo(0, line),
        Clear(ClearType::CurrentLine),
        Print(message)
    )
    .unwrap();
    stdout().flush().unwrap();
}
