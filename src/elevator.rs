#![cfg(windows)]

use windows::core::Result;
use windows::Win32::{
    Foundation::CloseHandle,
    Security::{
        GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_INFORMATION_CLASS, TOKEN_QUERY,
    },
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

pub fn is_running_as_admin() -> bool {
    unsafe {
        let mut token_handle = std::mem::zeroed();

        // Apri il token del processo corrente
        if let Err(_) = OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION::default();
        let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

        let result = GetTokenInformation(
            token_handle,
            TOKEN_INFORMATION_CLASS::TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            size,
            &mut size,
        );

        CloseHandle(token_handle);

        match result {
            Ok(_) => elevation.TokenIsElevated != 0,
            Err(_) => false,
        }
    }
}

pub fn require_admin() {
    if !is_running_as_admin() {
        eprintln!("This program must be run as administrator.");
        std::process::exit(1);
    }
}
