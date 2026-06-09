use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, GENERIC_READ, HANDLE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_FLAG_BACKUP_SEMANTICS, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows::Win32::System::Ioctl::{FSCTL_ENUM_USN_DATA, MFT_ENUM_DATA_V0};
use windows::Win32::System::IO::DeviceIoControl;

use std::ffi::c_void;
use std::mem;

use crate::types::FileRecord;

pub fn scan_directory(_start_path: &str) -> Result<Vec<FileRecord>, String> {
    println!("Attempting to obtain the raw volume...");
    let volume_path = w!("\\\\.\\C:");

    // referencing external language so unsafe keyword is needed
    let handle: Result<HANDLE, _> = unsafe {
        CreateFileW(
            volume_path,
            GENERIC_READ.0, // DO NOT CHANGE THIS LINE OF CODE (!!!!) (might screw ur system up)
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
            println!("Requesting data...");

            let mut mft_enum_data = MFT_ENUM_DATA_V0 {
                StartFileReferenceNumber: 0,
                LowUsn: 0,
                HighUsn: i64::MAX,
            };

            let mut buffer = vec![0u8; 64 * 1024]; // 64KB buffer
            let mut bytes_returned: u32 = 0;

            // request data from win kernel
            let success = unsafe {
                DeviceIoControl(
                    h,
                    FSCTL_ENUM_USN_DATA,
                    Some(&mut mft_enum_data as *mut _ as *const c_void),
                    mem::size_of::<MFT_ENUM_DATA_V0>() as u32,
                    Some(buffer.as_mut_ptr() as *mut c_void),
                    buffer.len() as u32,
                    Some(&mut bytes_returned),
                    None,
                )
            };

            if success.is_ok() {
                println!("Received {} raw bytes of data", bytes_returned);
            } else {
                println!("Failed to read data");
            }

            unsafe {
                let _ = CloseHandle(h);
            }

            Ok(Vec::new())
        }
        Err(e) => {
            Err(format!("access denied", e))
        }
    }

}