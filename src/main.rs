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
        
        let chunk_size = 15;
        let mut current_idx = 0;

        while current_idx < results.len() {
            let end_idx = std::cmp::min(current_idx + chunk_size, results.len());

            for i in current_idx..end_idx {
                println!(" -> {}", results[i].path.display());
            }

            current_idx = end_idx;

            if current_idx < results.len() {
                print!("\x1B[33m\n--- Showing {} of {} results --- [Press ENTER for more, or 'q' to search again] > \x1B[0m", current_idx, results.len());
                io::stdout().flush().unwrap();

                let mut scroll_input = String::new();
                io::stdin().read_line(&mut scroll_input).unwrap();

                if scroll_input.trim().eq_ignore_ascii_case("q") {
                    break;
                }
            }
        }


    }
    

}