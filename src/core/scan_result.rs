// src/core/scan_result.rs

use crate::core::vulnerability::Vulnerability;
use std::time::Duration;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub success: bool,
    pub vulnerabilities: Vec<Vulnerability>,
    pub warnings: Vec<String>,
    pub duration: Duration,
}