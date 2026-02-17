use anyhow::{Context, Result};
use reqwest::{Client, Method, Response};

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

    pub async fn request(
        &self,
        method: Method,
        path: &str,
        body: Option<String>,
    ) -> Result<Response> {
        let url = self.url(path);
        let mut req = self
            .http
            .request(method, &url)
            .basic_auth(&self.username, Some(&self.api_key))
            .header("Accept", "application/xml")
            .header("Content-Type", "application/xml");

        if let Some(body) = body {
            req = req.body(body);
        }

        let resp = req.send().await.context("Failed to send request")?;
        Ok(resp)
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
