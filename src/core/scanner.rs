// src/core/scanner.rs

use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use std::sync::Arc;

/// ScannerEngine controls max concurrency, timeout, and retries per scan.
pub struct ScannerEngine {
    max_concurrency: usize,
    timeout: Duration,
    retry_attempts: u32,
}

impl ScannerEngine {
    /// Creates a new ScannerEngine with default parameters.
    pub fn new() -> Self {
        Self {
            max_concurrency: 10,
            timeout: Duration::from_secs(30),
            retry_attempts: 3,
        }
    }

    /// Scans a target URL using given security module with concurrency control.
    pub async fn scan_target(
        &self,
        target: &str,
        module: Arc<dyn crate::core::modules::SecurityModule>,
    ) -> Result<crate::core::ScanResult, Box<dyn std::error::Error>> {
        let semaphore = Arc::new(Semaphore::new(self.max_concurrency));
        let start_time = Instant::now();

        // Acquire a permit to enforce concurrency limits
        let permit = semaphore.acquire().await?;
        let result = self.execute_scan(target, module).await;
        drop(permit);

        let duration = start_time.elapsed();
        if let Ok(scan_result) = &result {
            log::info!(
                "Scan completed in {:?} - {} vulnerabilities found", 
                duration, 
                scan_result.vulnerabilities.len()
            );
        }

        result
    }

    /// Executes the scan with retry and timeout logic.
    async fn execute_scan(
        &self,
        target: &str,
        module: Arc<dyn crate::core::modules::SecurityModule>,
    ) -> Result<crate::core::ScanResult, Box<dyn std::error::Error>> {
        for attempt in 0..self.retry_attempts {
            match tokio::time::timeout(self.timeout, module.run(target)).await {
                Ok(Ok(result)) => return Ok(result),
                Ok(Err(e)) => {
                    log::warn!("Scan attempt {} failed: {}", attempt + 1, e);
                    if attempt == self.retry_attempts - 1 {
                        return Err(e);
                    }
                }
                Err(_) => {
                    log::warn!("Scan attempt {} timed out", attempt + 1);
                    if attempt == self.retry_attempts - 1 {
                        return Err("Scan timed out".into());
                    }
                }
            }
            // Exponential backoff before retrying
            tokio::time::sleep(Duration::from_secs(1 << attempt)).await;
        }
        Err("All scan attempts failed".into())
    }
}