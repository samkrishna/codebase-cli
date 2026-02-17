use anyhow::{Context, Result};
use reqwest::{Client, Method, Response, StatusCode};
use std::time::Duration;
use tokio::time::sleep;

const MAX_RETRIES: u32 = 5;
const INITIAL_BACKOFF_MS: u64 = 1000;

#[derive(Debug, Clone)]
pub struct CodebaseClient {
    pub base_url: String,
    pub account: String,
    pub username: String,
    pub api_key: String,
    pub http: Client,
}

impl CodebaseClient {
    pub fn new(account: String, username: String, api_key: String) -> Self {
        Self {
            base_url: "https://api3.codebasehq.com".to_string(),
            account,
            username,
            api_key,
            http: Client::new(),
        }
    }

    /// Create a client with a custom base URL (for testing with mock servers).
    pub fn with_base_url(
        base_url: String,
        account: String,
        username: String,
        api_key: String,
    ) -> Self {
        Self {
            base_url,
            account,
            username,
            api_key,
            http: Client::new(),
        }
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    async fn send_request(
        &self,
        method: &Method,
        url: &str,
        body: Option<&str>,
    ) -> Result<Response> {
        let mut req = self
            .http
            .request(method.clone(), url)
            .basic_auth(&self.username, Some(&self.api_key))
            .header("Accept", "application/xml")
            .header("Content-Type", "application/xml");

        if let Some(body) = body {
            req = req.body(body.to_string());
        }

        req.send().await.context("Failed to send request")
    }

    pub async fn request(
        &self,
        method: Method,
        path: &str,
        body: Option<String>,
    ) -> Result<Response> {
        let url = self.url(path);
        let body_ref = body.as_deref();

        for attempt in 0..MAX_RETRIES {
            let resp = self.send_request(&method, &url, body_ref).await?;

            if resp.status() == StatusCode::from_u16(529).unwrap_or(StatusCode::SERVICE_UNAVAILABLE)
                || resp.status() == StatusCode::TOO_MANY_REQUESTS
                || resp.status() == StatusCode::SERVICE_UNAVAILABLE
            {
                let backoff = Duration::from_millis(INITIAL_BACKOFF_MS * 2u64.pow(attempt));
                eprintln!(
                    "Server returned {} — retrying in {}s (attempt {}/{})",
                    resp.status(),
                    backoff.as_secs(),
                    attempt + 1,
                    MAX_RETRIES
                );
                sleep(backoff).await;
                continue;
            }

            return Ok(resp);
        }

        // Final attempt with no retry
        self.send_request(&method, &url, body_ref).await
    }

    pub async fn get(&self, path: &str) -> Result<String> {
        let resp = self.request(Method::GET, path, None).await?;
        let status = resp.status();
        let text = resp.text().await.context("Failed to read response body")?;
        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }
        Ok(text)
    }

    pub async fn post(&self, path: &str, body: String) -> Result<String> {
        let resp = self.request(Method::POST, path, Some(body)).await?;
        let status = resp.status();
        let text = resp.text().await.context("Failed to read response body")?;
        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }
        Ok(text)
    }

    pub async fn put(&self, path: &str, body: String) -> Result<String> {
        let resp = self.request(Method::PUT, path, Some(body)).await?;
        let status = resp.status();
        let text = resp.text().await.context("Failed to read response body")?;
        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }
        Ok(text)
    }

    pub async fn delete(&self, path: &str) -> Result<String> {
        let resp = self.request(Method::DELETE, path, None).await?;
        let status = resp.status();
        let text = resp.text().await.context("Failed to read response body")?;
        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, text);
        }
        Ok(text)
    }
}
