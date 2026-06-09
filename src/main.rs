mod types;
mod database;
mod scanner;

use crate::database::Database;
use std::time::Instant;

fn main() {
    let mut db = Database::new();

    let target_folder = ".";

    println!("scanning directory '{}'...", target_folder);

    let start_scan = Instant::now();
    let found_files = scanner::scan_directory(target_folder);

    for file in found_files {
        db.insert(file);
    }

    let scan_duration = start_scan.elapsed();
    println!("Database loaded with {} files in {:?}.", db.records.len(), scan_duration);

    let search_term = "main";
    
    let start_search = Instant::now();
    let results = db.search(search_term);
    let search_duration = start_search.elapsed();
    
    println!("\nSearch for '{}' took {:?}", search_term, search_duration);
    
    for result in results.iter().take(5) {
        println!(" -> {}", result.path.display());
    }

}