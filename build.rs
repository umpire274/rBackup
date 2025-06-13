#[cfg(target_os = "windows")]
fn main() {
    embed_resource::compile("rbackup.rc", std::iter::empty::<&str>());
}

#[cfg(not(target_os = "windows"))]
fn main() {
    // Niente da fare su macOS/Linux
}
