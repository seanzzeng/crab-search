use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, GENERIC_READ, HANDLE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_FLAG_BACKUP_SEMANTICS, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
use crate::types::FileRecord;

pub fn scan_directory(start_path: &str) -> Result<Vec<FileRecord>, String> {
    let volume_path = w!("\\\\.\\C:");

    let handle: Result<HANDLE, _> = unsafe {
        CreateFileW(
            volume_path,
            GENERIC_READ.0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            FILE_FLAG_BACKUP_SEMANTICS,
            None,
        )
    };

    match handle {
        Ok(h) => {
            println!("Successfully connected to drive");

            unsafe {
                let _ = CloseHandle(h);
            }

            Ok(Vec::new())
        }
        Err(e) => {
            Err(format!("Access Denied. Are you running as Adminstrator? (Error Code: {})", e))
        }
    }

}