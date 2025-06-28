#![cfg(windows)]

use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    Security::{
        GetTokenInformation, TokenElevationType, TokenElevationTypeDefault, TokenElevationTypeFull,
        TOKEN_ELEVATION_TYPE, TOKEN_INFORMATION_CLASS, TOKEN_QUERY,
    },
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

pub fn is_running_as_admin() -> bool {
    unsafe {
        let mut token_handle = HANDLE::default();

        // Apri il token di accesso al processo corrente
        let result = OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle);
        if result.0 == 0 {
            return false;
        }

        let mut elevation_type = TOKEN_ELEVATION_TYPE::default();
        let mut returned_length = std::mem::size_of::<TOKEN_ELEVATION_TYPE>() as u32;

        let result = GetTokenInformation(
            token_handle,
            TOKEN_INFORMATION_CLASS(18), // TokenElevationType
            Some(&mut elevation_type as *mut _ as *mut _),
            returned_length,
            &mut returned_length,
        );

        CloseHandle(token_handle);

        if result.0 == 0 {
            return false;
        }

        elevation_type == TokenElevationTypeFull
    }
}

pub fn require_admin() {
    if !is_running_as_admin() {
        eprintln!("This program must be run as administrator.");
        std::process::exit(1);
    }
}
