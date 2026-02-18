#[cfg(target_os = "windows")]
fn main() {
    use winresource::WindowsResource;

    // Assicurati che res/rbackup.ico esista
    let mut res = WindowsResource::new();
    res.set_icon("res/rbackup.ico")
        .set("FileDescription", "rBackup - Incremental Backup Utility")
        .set("ProductName", "rBackup")
        .set("OriginalFilename", "rbackup.exe")
        .set("FileVersion", env!("CARGO_PKG_VERSION"))
        .set("ProductVersion", env!("CARGO_PKG_VERSION"))
        .compile()
        .expect("Failed to embed icon resource");
}

#[cfg(not(target_os = "windows"))]
fn main() {}
