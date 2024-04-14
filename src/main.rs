extern crate ferrodb; // This line is often optional in the latest editions of Rust

use ferrodb::FerroDB;
use rustyline::Editor;
use std::{sync::{Arc, Mutex}, thread, time::Duration};

fn main() {
    // Define the path to the data file
    let db_file = "data.fdbz";

    // Try to load the database from the file, or create a new instance if it fails
    let db = Arc::new(Mutex::new(match FerroDB::load_from_file(db_file) {
        Ok(loaded_db) => {
            println!("Database loaded successfully.");
            loaded_db
        },
        Err(e) => {
            println!("Failed to load database: {}. A new database will be used.", e);
            FerroDB::new()
        }
    }));

    let db_clone = db.clone();

    // Background thread for saving the database periodically
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(60)); // Save every minute
            let db = db_clone.lock().unwrap();
            match db.save_to_file(db_file) {
                Ok(_) => println!("Database saved successfully."),
                Err(e) => println!("Failed to save database: {}", e),
            }
        }
    });

    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("ferrodb> ");
        match readline {
            Ok(line) => {
                let commands: Vec<&str> = line.split_whitespace().collect();
                if commands.is_empty() {
                    continue;
                }
                match commands[0] {
                    "CREATE" if commands.len() == 2 => {
                        let mut db = db.lock().unwrap(); // Acquire the lock
                        db.create_collection(commands[1].to_string());
                        println!("Collection created");
                    },
                    "USE" if commands.len() == 2 => {
                        let mut db = db.lock().unwrap(); // Acquire the lock
                        match db.use_collection(commands[1].to_string()) {
                            Ok(_) => println!("Now using collection: {}", commands[1]),
                            Err(err) => println!("{}", err),
                        }
                    },
                    "SET" if commands.len() == 3 => {
                        let mut db = db.lock().unwrap(); // Acquire the lock
                        match db.set(commands[1].to_string(), commands[2].to_string()) {
                            Ok(_) => println!("OK"),
                            Err(err) => println!("{}", err),
                        }
                    },
                    "GET" if commands.len() == 2 => {
                        let db = db.lock().unwrap(); // Acquire the lock
                        match db.get(commands[1]) {
                            Ok(Some(value)) => println!("{}", value),
                            Ok(None) => println!("(nil)"),
                            Err(err) => println!("{}", err),
                        }
                    },
                    "UNSET" if commands.len() == 2 => {
                        let mut db = db.lock().unwrap(); // Acquire the lock
                        match db.unset(commands[1]) {
                            Ok(Some(value)) => println!("OK, removed: {}", value),
                            Ok(None) => println!("Key not found"),
                            Err(err) => println!("{}", err),
                        }
                    },
                    "EXIT" => {
                        let db = db.lock().unwrap(); // Acquire the lock
                        match db.save_to_file("data.fdbz") {
                            Ok(_) => println!("Database saved successfully."),
                            Err(e) => println!("Failed to save database: {}", e),
                        }
                        break;
                    },
                    _ => println!("Unknown command or wrong number of arguments"),
                }
            },
            Err(_) => {
                println!("Error reading line");
                break;
            }
        }
    }
}
