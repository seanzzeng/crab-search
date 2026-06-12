use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FileRecord {
    pub name: String, // file name
    pub path: PathBuf, // full path
    pub size_bytes: u64,
    pub is_dir: bool,
    pub id: u64, // file id
    pub parent_id: u64,
}

impl FileRecord {
    pub fn new(name: String, path: PathBuf, size_bytes: u64, is_dir: bool, id: u64, parent_id: u64) -> Self {
        Self {
            name,
            path,
            size_bytes,
            is_dir,
            id,
            parent_id,
        }
    }
}