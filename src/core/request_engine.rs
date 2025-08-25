use reqwest::{Client, Response};
use std::time::Duration;
use tokio::sync::Mutex;
use std::collections::HashMap;

pub struct RequestEngine {
    client: Client,
    rate_limiter: Mutex<()>,
    request_count: Mutex<u32>,
}

impl RequestEngine {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Redlines-X-Scanner/1.0")
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        Self {
            client,
            rate_limiter: Mutex::new(()),
            request_count: Mutex::new(0),
        }
    }

    pub async fn send_request(
        &self,
        url: &str,
        method: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        let _guard = self.rate_limiter.lock().await;
        tokio::time::sleep(Duration::from_millis(100)).await;

        let mut request = match method.to_uppercase().as_str() {
            "GET" => self.client.get(url),
            "POST" => self.client.post(url),
            "PUT" => self.client.put(url),
            "DELETE" => self.client.delete(url),
            _ => return Err("Unsupported HTTP method".into()),
        };

        if let Some(headers_map) = headers {
            for (key, value) in headers_map {
                request = request.header(&key, &value);
            }
        }

        if let Some(body_content) = body {
            request = request.body(body_content);
        }

        let mut count = self.request_count.lock().await;
        *count += 1;
        log::debug!("Sending request #{} to {}", count, url);

        let response = request.send().await?;
        Ok(response)
    }

    pub async fn get_request_count(&self) -> u32 {
        *self.request_count.lock().await
    }
}