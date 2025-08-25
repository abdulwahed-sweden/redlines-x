pub mod session;
pub mod database;
pub mod modules;
pub mod module_handler;  // ← تأكد من إضافة هذا السطر
pub mod logger;
pub mod scanner;
pub mod request_engine;
pub mod scan_result;
pub mod vulnerability;

// Re-exports for easier access
pub use session::Session;
pub use database::Database;
pub use modules::SecurityModule;
pub use module_handler::ModuleHandler;  // ← وتأكد من إضافة هذا السطر
pub use scanner::ScannerEngine;
pub use request_engine::RequestEngine;
pub use scan_result::ScanResult;
pub use vulnerability::{Vulnerability, SeverityLevel};