#![cfg(windows)]

use windows::Win32::{
    Foundation::CloseHandle,
    Security::{GetTokenInformation, TOKEN_ELEVATION, TOKEN_INFORMATION_CLASS, TOKEN_QUERY},
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

pub fn is_running_as_admin() -> bool {
    unsafe {
        let mut token_handle = std::mem::zeroed();

        // Apri il token del processo corrente
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle).is_err() {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION::default();
        let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

        // 7 = TokenElevation (non esposto direttamente come costante)
        let result = GetTokenInformation(
            token_handle,
            TOKEN_INFORMATION_CLASS(7),
            Some(&mut elevation as *mut _ as *mut _),
            size,
            &mut size,
        );

        CloseHandle(token_handle);

        result.is_ok() && elevation.TokenIsElevated != 0
    }
}

pub fn require_admin() {
    if !is_running_as_admin() {
        eprintln!("This program must be run as administrator.");
        std::process::exit(1);
    }
}
