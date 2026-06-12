mod types;
mod database;
mod scanner;

use crate::database::Database;
use std::io::{self, Write};
use std::time::Instant;
use std::process::Command;

fn main() {
    let mut db = Database::new();

    let target_folder = ".";

    // force colour codes to work
    colored::control::set_virtual_terminal(true).unwrap_or(());

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
                println!("[{}] {}", i, results[i].path.display());
            }

            current_idx = end_idx;

            loop {
                if current_idx >= results.len() {
                    print!("\x1B[33m\n--- End of results --- [q: New Search | NUMBER: Open File] > \x1B[0m");
                } else {
                    print!("\x1B[33m\n--- Showing {} of {} --- [ENTER: Next Page | q: New Search | NUMBER: Open File] > \x1B[0m", current_idx, results.len());
                }
                io::stdout().flush().unwrap();

                let mut action_input = String::new();
                io::stdin().read_line(&mut action_input).unwrap();
                let trimmed = action_input.trim();
                
                if trimmed.is_empty() {
                    break;
                } else if trimmed.eq_ignore_ascii_case("q") {
                    current_idx = results.len(); 
                    break;
                } else if let Ok(num) = trimmed.parse::<usize>() {
                    if num < results.len() {
                        let _ = Command::new("explorer")
                            .arg("/select,")
                            .arg(&results[num].path)
                            .spawn();
                    } else {
                        println!("Invalid number");
                    }
                } else {
                    println!("Unrecognized command");
                }
            }
        }


    }
    

}