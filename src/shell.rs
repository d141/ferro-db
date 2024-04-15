use super::FerroDB;
use rustyline::Editor;
use std::{sync::{Arc, Mutex}, thread, time::Duration};

pub async fn run_shell(db: Arc<Mutex<FerroDB>>, db_file: String) {

    let db_clone = db.clone();

    // Background thread for saving the database periodically
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(60)); // Save every minute
            let db = db_clone.lock().unwrap();
            match db.save_to_file(&db_file) {
                Ok(_) => println!("Database saved successfully."),
                Err(e) => println!("Failed to save database: {}", e),
            }
            match db.save_to_file("changes.fdbz") {
                Ok(_) => println!("Changes saved successfully."),
                Err(e) => println!("Failed to save changes: {}", e),
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
                    "SET" if commands.len() == 4 => {
                        // commands[1] is the collection, commands[2] is the key, commands[3] is the value
                        let mut db = db.lock().unwrap();
                        match db.set(commands[1], commands[2].to_string(), commands[3].to_string()) {
                            Ok(_) => println!("OK"),
                            Err(err) => println!("Error: {}", err),
                        }
                    },
                    "GET" if commands.len() == 3 => {
                        // commands[1] is the collection, commands[2] is the key
                        let db = db.lock().unwrap();
                        match db.get(commands[1], commands[2]) {
                            Ok(Some(value)) => println!("Value: {}", value),
                            Ok(None) => println!("Key not found"),
                            Err(err) => println!("Error: {}", err),
                        }
                    },
                    "UNSET" if commands.len() == 3 => {
                        // commands[1] is the collection, commands[2] is the key
                        let mut db = db.lock().unwrap();
                        match db.unset(commands[1], commands[2]) {
                            Ok(Some(value)) => println!("Removed: {}", value),
                            Ok(None) => println!("Key not found"),
                            Err(err) => println!("Error: {}", err),
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
