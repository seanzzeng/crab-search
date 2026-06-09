use std::path::PathBuf;


#[derive(Debug, Clone)]
pub struct FileRecord {
    pub name: String, // file name
    pub path: PathBuf, // full path
    pub size_bytes: u64,
    pub is_dir: bool,
}

impl FileRecord {
    pub fn new(name: String, path: PathBuf, size_bytes: u64, is_dir: bool) -> Self {
        Self {
            name,
            path,
            size_bytes,
            is_dir,
        }
    }
}