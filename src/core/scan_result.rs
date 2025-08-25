// src/core/scan_result.rs

use std::fmt;

#[derive(Debug)]
pub struct ScanResult {
    pub success: bool,
    pub details: String,
}

impl ScanResult {
    pub fn new(success: bool, details: impl Into<String>) -> Self {
        Self {
            success,
            details: details.into(),
        }
    }
}

// Optional: Implement Display for user friendly printing
impl fmt::Display for ScanResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scan Success: {}, Details: {}", self.success, self.details)
    }
}
