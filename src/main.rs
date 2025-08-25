mod core;
mod modules;
mod cli;

use crate::core::{Database, ModuleHandler, Logger, ScannerEngine};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    Logger::init();
    
    println!("🚀 Redlines X - Advanced Security Scanner");
    println!("=========================================");
    
    // Initialize database
    let db = Database::new()?;
    Logger::info("Database initialized successfully");
    
    // Initialize module handler
    let mut module_handler = ModuleHandler::new();
    
    // Load and register modules
    let modules = modules::load_all_modules();
    for module in modules {
        module_handler.register_module(module);
    }
    
    Logger::info(&format!("Loaded {} modules", module_handler.module_count()));
    
    // List available modules
    for module_name in module_handler.list_modules() {
        if let Some(module) = module_handler.get_module(&module_name) {
            Logger::info(&format!("- {}: {}", module.name(), module.description()));
        }
    }
    
    // Initialize scanner engine
    let scanner_engine = ScannerEngine::new();
    
    // Start CLI with all components
    cli::run_cli(module_handler, db, scanner_engine).await?;
    
    Ok(())
}