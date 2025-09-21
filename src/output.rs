use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::fs::File;
use std::io::{BufWriter, Write, stdout};
use std::sync::{Arc, Mutex};

pub fn now() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn print_message(
    msg: &str,
    logger: &Option<&Arc<Mutex<BufWriter<File>>>>,
    quiet: bool,
    with_timestamp: bool,
    row: Option<u16>,
    on_log: bool,
) {
    let full_msg = if with_timestamp && !msg.is_empty() {
        format!("[{}] {}", now(), msg)
    } else {
        msg.to_string()
    };

    if let Some(row) = row {
        let mut stdout = stdout();
        let _ = stdout
            .queue(MoveTo(0, row))
            .and_then(|s| s.queue(Clear(ClearType::CurrentLine)))
            .and_then(|s| s.queue(Print(&full_msg)))
            .and_then(|s| s.flush());
    } else if !quiet {
        println!("{full_msg}");
    }

    if on_log {
        if let Some(file) = logger {
            let mut file = file.lock().unwrap();
            let _ = writeln!(file, "{full_msg}");
        }
    }
}
