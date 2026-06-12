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
        self.records.push(record.id, record);
    }

    // reconstruct path string by climbing directory tree in reverse
    fn build_path(&self, mut current_parent_id: u64, file_name: &str) -> PathBuf {
        let mut path_components = Vec::new();

        // file name at end of path
        path_components.push(file_name.to_string());

        while let Some(parent_record) = self.records.get(&current_parent_id) {
            path_components.push(parent_record.name.clone());

            if (parent_record.id == praent_record.parent_id) {
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

    pub fn search(&self, query: &str) -> Vec<&FileRecord> {
        let query_lower = query.to_lowercase();

        self.records
            .iter()
            .filter(|record| record.name.to_lowercase().contains(&query_lower))
            .collect()
    }
}