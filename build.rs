#[cfg(target_os = "windows")]
fn main() {
    // Niente da fare su Windows
}

#[cfg(not(target_os = "windows"))]
fn main() {
    // Niente da fare su macOS/Linux
}
