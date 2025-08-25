use async_trait::async_trait;
use crate::core::scan_result::ScanResult;
use crate::core::vulnerability::{Vulnerability, SeverityLevel};

#[async_trait]
pub trait SecurityModule {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn author(&self) -> &str;
    fn version(&self) -> &str;
    
    async fn run(&self, target: &str) -> Result<ScanResult, Box<dyn std::error::Error + Send + Sync>>;
}

// Re-exports
pub use crate::core::scan_result::ScanResult;
pub use crate::core::vulnerability::{Vulnerability, SeverityLevel};