#![cfg(windows)]

use std::ptr::null_mut;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    Security::{
        GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_INFORMATION_CLASS, TOKEN_QUERY,
    },
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

pub fn is_running_as_admin() -> bool {
    unsafe {
        let mut token_handle = HANDLE::default();

        // Open the access token for the current process
        if !OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle).as_bool() {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION::default();
        let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

        // Get elevation information
        if !GetTokenInformation(
            token_handle,
            TOKEN_INFORMATION_CLASS::TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            size,
            &mut size,
        ).as_bool()
        {
            CloseHandle(token_handle);
            return false;
        }

        CloseHandle(token_handle);
        elevation.TokenIsElevated != 0
    }
}

pub fn require_admin() {
    if !is_running_as_admin() {
        eprintln!("This program must be run as administrator.");
        std::process::exit(1);
    }
}
