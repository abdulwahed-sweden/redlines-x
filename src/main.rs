// src/main.rs
mod core;
mod cli;
mod modules;  // Add this line

use core::session::SessionManager;
use core::module_handler::ModuleHandler;
use core::database::Database;

#[tokio::main]
async fn main() {
    println!("Welcome to Redlines X - Advanced Security Scanner");

    // Construct minimal instances to avoid dead_code warnings
    let _session = SessionManager::new();
    let _module_handler = ModuleHandler::new();
    let _database = Database::new();

    cli::run_cli().await;
}
