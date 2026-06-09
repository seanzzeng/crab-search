mod types;
mod database;

use crate::types::FileRecord;
use crate::database::Database;
use std::path::PathBuf;

fn main() {
    let mut db = Database::new();

    let record1 = FileRecord::new(
        "secret.txt".to_string(),
        PathBuf::from("C:\\Users\\Documents\\secret.txt"),
        1024,
        false,
    );

    let record2 = FileRecord::new(
        "shop.txt".to_string(),
        PathBuf::from("C:\\Users\\Documents\\shop.txt"),
        256,
        false,
    );

    db.insert(record1);
    db.insert(record2);

    println!("Database loaded with {} files.", db.records.len());

    let search_term = "secret";
    println!("Searching for: '{}'", search_term);
    
    let results = db.search(search_term);
    
    println!("Found {} results:\n{:#?}", results.len(), results);

}