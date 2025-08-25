// src/core/logger.rs

use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;

/// Logger helper for initialization and structured scan health logs.
pub struct Logger;

impl Logger {
    /// Initialize logging to file and stdout with timestamps.
    pub fn init() {
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("redlines.log")
            .unwrap();

        env_logger::Builder::new()
            .format(|buf, record| {
                let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
                writeln!(buf, "[{}] {} - {}", record.level(), timestamp, record.args())
            })
            .target(env_logger::Target::Pipe(Box::new(log_file)))
            .init();
    }

    /// Log start of scan.
    pub fn scan_start(target: &str) {
        log::info!("🚀 Starting scan: {}", target);
    }

    /// Log completion of scan with summary.
    pub fn scan_complete(target: &str, vuln_count: usize, duration: std::time::Duration) {
        log::info!("✅ Scan completed: {} - Found {} vulnerabilities in {:?}", target, vuln_count, duration);
    }

    /// Log single vulnerability discovered.
    pub fn vulnerability_found(vuln_type: &str, location: &str) {
        log::warn!("⚠️ Vulnerability found: {} at {}", vuln_type, location);
    }
}
