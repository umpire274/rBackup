#[cfg(target_os = "windows")]
use std::path::Path;

#[cfg(target_os = "windows")]
fn main() {
    use winresource::WindowsResource;

    // Path to the icon file
    let icon_path = "assets/rbackup.ico";
    // Path to the resource script
    let rc_path = "assets/rbackup.rc";

    // Check if the file exists
    assert!(
        Path::new(icon_path).exists(),
        "Icon file not found at {}",
        icon_path
    );
    // Check if the resource script exists
    assert!(
        Path::new(rc_path).exists(),
        "Resource script not found at {}",
        rc_path
    );

    // Initialize Windows resource
    let mut res = WindowsResource::new();

    // Specify the .rc file to compile
    res.set_resource_file(rc_path);

    // Set the icon
    res.set_icon(icon_path);

    // Main metadata (versions are read from Cargo.toml)
    let version = env!("CARGO_PKG_VERSION");
    res.set("FileDescription", "rBackup - Incremental Backup Utility");
    res.set("ProductName", "rBackup");
    res.set("OriginalFilename", "rbackup.exe");
    res.set("FileVersion", version);
    res.set("ProductVersion", version);
    res.set("LegalCopyright", "© 2025 Alessandro Maestri");

    // Compile the resources
    res.compile().expect("Failed to embed Windows resources");

    // 🪶 Trigger for automatic rebuild
    println!("cargo:rerun-if-changed={}", icon_path);
    println!("cargo:rerun-if-changed={}", rc_path);
}

#[cfg(not(target_os = "windows"))]
fn main() {
    // Nothing to do on Linux/macOS
}
