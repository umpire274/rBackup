use std::path::Path;

#[cfg(target_os = "windows")]
fn main() {
    use winresource::WindowsResource;

    // Percorso all'icona (modifica se necessario)
    let icon_path = "assets/rbackup.ico";

    // Verifica che il file esista
    assert!(
        Path::new(icon_path).exists(),
        "Icon file not found at {}",
        icon_path
    );

    // Inizializza risorsa Windows
    let mut res = WindowsResource::new();

    // Imposta l'icona principale
    res.set_icon(icon_path);

    // Metadati principali (le versioni vengono lette da Cargo.toml)
    let version = env!("CARGO_PKG_VERSION");
    res.set("FileDescription", "rBackup - Incremental Backup Utility");
    res.set("ProductName", "rBackup");
    res.set("OriginalFilename", "rbackup.exe");
    res.set("FileVersion", version);
    res.set("ProductVersion", version);
    res.set("LegalCopyright", "© 2025 Alessandro Maestri");

    // (Opzionale) se vuoi integrare un file .rc aggiuntivo
    // res.set_manifest_file("assets/rbackup.rc");

    // Compila le risorse
    res.compile().expect("Failed to embed Windows resources");

    // Forza la ricompilazione se cambia l’icona
    println!("cargo:rerun-if-changed={}", icon_path);
}

#[cfg(not(target_os = "windows"))]
fn main() {
    // Niente da fare su Linux/macOS
}
