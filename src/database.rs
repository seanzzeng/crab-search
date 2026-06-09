use crate::types::FileRecord;

pub struct Database {
    pub records: Vec<FileRecord>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub fn insert(&mut self, record: FileRecord) {
        self.records.push(record);
    }

    pub fn search(&self, query: &str) -> Vec<&FileRecord> {
        let query_lower = query.to_lowercase();

        self.records
            .iter()
            .filter(|record| record.name.to_lowercase().contains(&query_lower))
            .collect()
    }
}