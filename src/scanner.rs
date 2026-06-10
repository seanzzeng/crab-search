use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, GENERIC_READ, HANDLE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_FLAG_BACKUP_SEMANTICS, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows::Win32::System::Ioctl::{FSCTL_ENUM_USN_DATA, MFT_ENUM_DATA_V0, USN_RECORD_V2};
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

                let mut discovered_files = Vec::new();

                // first 8 bytes is the bookmark, skip to get actual records
                let mut current_offset: usize = 8;

                while current_offset < bytes_returned as usize {
                    let record_ptr = unsafe { buffer.as_ptr().add(current_offset) as *const USN_RECORD_V2 };
                    let record = unsafe { &*record_ptr };

                    let record_length = record.RecordLength as usize;

                    // filenames are 16 bytes
                    let name_ptr = unsafe {
                        (record_ptr as *const u8).add(record.FileNameOffset as usize) as *const u16
                    };
                    let name_len = (record.FileNameLength / 2) as usize;

                    let name_slice = unsafe { std::slice::from_raw_parts(name_ptr, name_len) };
                    let file_name = String::from_utf16_lossy(name_slice);

                    discovered_files.push(FileRecord::new(
                        file_name,
                        std::path::PathBuf::new(),
                        0,
                        false, // place holders
                    ));

                    current_offset += record_length;
                }

                println!("Parsed {} files from buffer", discovered_files.len());

                unsafe {
                    let _ = CloseHandle(h);
                }

                Ok(discovered_files)

            } else {
                unsafe {
                    let _ = CloseHandle(h);
                }

                Err("Failed to read data".to_string())
            }
        }
        Err(e) => {
            Err(format!("access denied, code {}", e))
        }
    }

}