extern crate ferrodb; // This line is often optional in the latest editions of Rust

use ferrodb::FerroDB;
use rustyline::Editor;

fn main() {
    // Check if we can load the previous state
    let db_file = "db.json";
    let mut db = match FerroDB::load_from_file(db_file) {
        Ok(loaded_db) => loaded_db,
        Err(_) => {
            println!("Starting with a new database.");
            FerroDB::new()
        },
    };

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
                        db.create_collection(commands[1].to_string());
                        println!("Collection created");
                    },
                    "USE" if commands.len() == 2 => {
                        match db.use_collection(commands[1].to_string()) {
                            Ok(_) => println!("Now using collection: {}", commands[1]),
                            Err(err) => println!("{}", err),
                        }
                    },
                    "SET" if commands.len() == 3 => {
                        match db.set(commands[1].to_string(), commands[2].to_string()) {
                            Ok(_) => println!("OK"),
                            Err(err) => println!("{}", err),
                        }
                    },
                    "GET" if commands.len() == 2 => {
                        match db.get(commands[1]) {
                            Ok(Some(value)) => println!("{}", value),
                            Ok(None) => println!("(nil)"),
                            Err(err) => println!("{}", err),
                        }
                    },
                    "UNSET" if commands.len() == 2 => {
                        match db.unset(commands[1]) {
                            Ok(Some(value)) => println!("OK, removed: {}", value),
                            Ok(None) => println!("Key not found"),
                            Err(err) => println!("{}", err),
                        }
                    },
                    "EXIT" => {
                        db.save_to_file(db_file).expect("Error saving the database");
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
