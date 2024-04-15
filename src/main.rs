use ferrodb::FerroDB;  // This assumes your lib.rs re-exports FerroDB
use ferrodb::run_shell;

use std::{sync::{Arc, Mutex}};
use clap::{Command};
use actix_web::{HttpServer, App as ActixApp, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = Command::new("FerroDB")
        .version("0.1.0")
        .author("Your Name")
        .about("FerroDB - A simple key-value store")
        .subcommand(Command::new("shell")
            .about("Runs an interactive shell"))
        .subcommand(Command::new("server")
            .about("Runs as an HTTP server"))
        .get_matches();

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

    match matches.subcommand() {
        Some(("shell", _)) => {
            run_shell(db, db_file.to_string()).await;
        }
        Some(("server", _)) => {
            HttpServer::new(move || {
                ActixApp::new()
                    .app_data(web::Data::new(db.clone()))
                    // Define routes
            })
            .bind("127.0.0.1:8080")?
            .run()
            .await?
        }
        _ => {
            println!("No command specified, use --help for info");
        }
    }

    Ok(())
}

