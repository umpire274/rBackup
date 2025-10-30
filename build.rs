#[cfg(target_os = "windows")]
use std::path::Path;

#[cfg(target_os = "windows")]
fn main() {
    use winresource::WindowsResource;

    // Path to the icon (edit if necessary)
    let icon_path = "assets/rbackup.ico";

    // Check if the file exists
    assert!(
        Path::new(icon_path).exists(),
        "Icon file not found at {}",
        icon_path
    );

    // Initialize Windows resource
    let mut res = WindowsResource::new();

    // Set the main icon
    res.set_icon(icon_path);

    // Main metadata (versions are read from Cargo.toml)
    let version = env!("CARGO_PKG_VERSION");
    res.set("FileDescription", "rBackup - Incremental Backup Utility");
    res.set("ProductName", "rBackup");
    res.set("OriginalFilename", "rbackup.exe");
    res.set("FileVersion", version);
    res.set("ProductVersion", version);
    res.set("LegalCopyright", "© 2025 Alessandro Maestri");

    // (Optional) if you want to integrate an additional .rc file
    // res.set_manifest_file("assets/rbackup.rc");

    // Compile the resources
    res.compile().expect("Failed to embed Windows resources");

    // Force recompile if icon changes
    println!("cargo:rerun-if-changed={}", icon_path);
}

#[cfg(not(target_os = "windows"))]
fn main() {
    // Nothing to do on Linux/macOS
}
