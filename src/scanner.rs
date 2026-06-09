use walkdir::WalkDir;
use crate::types::FileRecord;

pub fn scan_directory(start_path: &str) -> Vec<FileRecord> {
    let mut discovered_files = Vec::new();

    // only search permissable files
    for entry in WalkDir::new(start_path).into_iter().filter_map(Result::ok) {
        let metadata = match entry.metadata() {
            Ok(meta) => meta,
            Err(_) => continue,
        };

        let record = FileRecord::new(
            entry.file_name().to_string_lossy().to_string(),
            entry.path().to_path_buf(),
            metadata.len(),
            metadata.is_dir(),
        );

        discovered_files.push(record);

    }

    discovered_files

}