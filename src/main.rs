// src/main.rs
pub mod core;
pub mod modules;  // This line must be added to expose `modules`
pub mod cli;



#[tokio::main]
async fn main() {
    println!("Welcome to Redlines X - Advanced Security Scanner");

    // Example of session and database usage
    let mut session = core::session::Session::new("https://example.com");
    session.add_cookie("sessionid", "abcd1234");

    let db = core::database::Database::new().expect("Failed to open database");
    db.save_session(&session).expect("Failed to save session");

    println!("Session saved with ID: {}", session.id);

    cli::run_cli().await;
}
