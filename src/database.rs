use crate::types::FileRecord;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Database {
    pub records: HashMap<u64, FileRecord>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    pub fn insert(&mut self, record: FileRecord) {
        self.records.insert(record.id, record);
    }

    // reconstruct path string by climbing directory tree in reverse
    fn build_path(&self, mut current_parent_id: u64, file_name: &str) -> PathBuf {
        let mut path_components = Vec::new();

        // file name at end of path
        path_components.push(file_name.to_string());

        while let Some(parent_record) = self.records.get(&current_parent_id) {
            path_components.push(parent_record.name.clone());

            if parent_record.id == parent_record.parent_id {
                break; // hit root
            }

            current_parent_id = parent_record.parent_id;
        }

        path_components.push("C:\\".to_string());

        path_components.reverse();

        let mut full_path = PathBuf::new();
        for component in path_components {
            full_path.push(component);
        }

        full_path
    }

    pub fn search(&self, query: &str) -> Vec<FileRecord> {
        let mut search_term = String::new();
        let mut ext_filter: Option<String> = None;
        let mut folder_only = false;

        for part in query.split_whitespace() {
            let part_lower = part.to_lowercase();
            
            if part_lower.starts_with("ext:") {
                ext_filter = Some(part_lower.replace("ext:", ""));
            } else if part_lower == "type:folder" || part_lower == "type:dir"  {
                folder_only = true;
            } else {
                if !search_term.is_empty() {
                    search_term.push(' ');
                }
                search_term.push_str(&part_lower);
            }
        }

        self.records
            .values()
            .filter(|record| {
                let name_lower = record.name.to_lowercase();

                // reject anything thats not a folder
                if folder_only && !record.is_dir {
                    return false;
                }

                // reject anything with the wrong extension
                if let Some(ref ext) = ext_filter {
                    let target_ext = format!(".{}", ext);
                    if !name_lower.ends_with(&target_ext) {
                        return false;
                    }
                }

                if !search_term.is_empty() && !name_lower.contains(&search_term) {
                    return false;
                }

                true
            })
            .map(|record| {
                let mut completed_record = record.clone();
                completed_record.path = self.build_path(record.parent_id, &record.name);
                completed_record
                // only build path when we search for it for memory efficiency
            })
            .collect()
    }
}