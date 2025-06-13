#[cfg(target_os = "windows")]
fn main() {
    embed_resource::compile("rbackup.rc").unwrap();
}

#[cfg(not(target_os = "windows"))]
fn main() {
    // No resource compilation needed on non-Windows platforms
}
