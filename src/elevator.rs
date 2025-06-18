#[cfg(target_os = "windows")]
pub fn require_admin() {
    use std::process::Command;
    use std::env;
    
    if !is_running_as_admin() {
        let args: Vec<String> = env::args().collect();
        let mut cmd = Command::new("powershell");
        let params = format!("Start-Process -Verb runAs -FilePath '{}' -ArgumentList '{}'", &args[0], &args[1..].join(" "));
        let _ = cmd.arg("-Command").arg(params).status();
        std::process::exit(0);
    }
}

#[cfg(target_os = "windows")]
fn is_running_as_admin() -> bool {
    use windows::Win32::Security::*;
    use windows::Win32::System::Threading::*;
    use windows::Win32::Foundation::*;
    
    unsafe {
        let mut token_handle: HANDLE = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle).as_bool() {
            let mut elevation = TOKEN_ELEVATION::default();
            let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
            if GetTokenInformation(
                token_handle,
                TokenElevation,
                Some(&mut elevation as *mut _ as *mut _),
                size,
                &mut size,
            )
            .as_bool()
            {
                return elevation.TokenIsElevated != 0;
            }
        }
    }
    false
}

#[cfg(not(target_os = "windows"))]
pub fn require_admin() {
    // No elevation needed on Unix-based systems
}
