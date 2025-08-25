// src/cli/mod.rs

use clap::{Parser, Subcommand};
// Import the SQLInjectionScanner type from its module
use crate::modules::auxiliary::sql_injection::SQLInjectionScanner;


#[derive(Parser)]
#[command(name = "redlines-x")]
#[command(about = "Advanced Security Scanner CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run SQL Injection scan
    SqlInjection {
        #[arg(short, long)]
        url: String,
    },
    /// List available modules
    List,
    /// Show version info
    Version,
}

pub async fn run_cli() {
    let cli = Cli::parse();

    match cli.command {
        Commands::SqlInjection { url } => {
            println!("Running SQL Injection scan on: {}", url);
            let scanner = SQLInjectionScanner::new();
            let result = scanner.run_scan(&url).await;
            println!("{:?}", result);
        }
        Commands::List => {
            println!("Available modules:\n - sql_injection");
        }
        Commands::Version => {
            println!("Redlines X version 0.1");
        }
    }
}
