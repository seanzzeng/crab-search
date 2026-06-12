mod types;
mod database;
mod scanner;

use crate::database::Database;
use std::io::{self, Write};
use std::time::Instant;

fn main() {
    let mut db = Database::new();

    let target_folder = ".";

    println!("Scanning directory '{}'...", target_folder);

    let start_scan = Instant::now();

    match scanner::scan_directory(target_folder) {
        Ok(found_files) => {
            for file in found_files {
                db.insert(file);
            }
            let scan_duration = start_scan.elapsed();
            println!("Database loaded with {} files in {:?}", db.records.len(), scan_duration);
        }
        Err(e_msg) => {
            eprintln!("Error message: {}, are you opening as adminstrator?", e_msg);
            return;
        }
    }

    println!("Search engine initiated. Type 'quit' to quit");

    loop {
        print!("\nSearch > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let search_term = input.trim();

        if search_term.eq_ignore_ascii_case("quit") {
            println!("Exiting...");
            break;
        }

        if search_term.is_empty() {
            continue;
        }

        let start_search = Instant::now();
        let results = db.search(search_term);
        let search_duration = start_search.elapsed();
        
        println!("\nSearch for '{}' took {:?}", search_term, search_duration);
        println!("{} results found", results.len());
        
        for result in results.iter().take(10) {
            println!(" -> {}", result.path.display());
        }
    }
    

}