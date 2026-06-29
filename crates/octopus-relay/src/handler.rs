use reqwest::Client;
use octopus_core::model::channel::Channel;
use octopus_db::ops::channel as channel_ops;

/// Main relay handler - processes LLM requests through channels
pub struct RelayHandler {
    client: Client,
}

impl RelayHandler {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .unwrap_or_default(),
        }
    }

    /// Get an available channel for the given group
    pub async fn get_channel(&self, channel_id: i32) -> Result<Option<Channel>, octopus_core::error::AppError> {
        channel_ops::get_channel(channel_id).await
    }

    /// Forward request to upstream
    pub async fn forward(
        &self,
        url: &str,
        body: &[u8],
        headers: &[(&str, &str)],
        api_key: &str,
    ) -> Result<reqwest::Response, octopus_core::error::AppError> {
        let mut req = self.client
            .post(url)
            .body(body.to_vec());

        for (k, v) in headers {
            req = req.header(*k, *v);
        }
        req = req.header("Authorization", format!("Bearer {}", api_key));

        let resp = req.send().await?;
        Ok(resp)
    }
}

impl Default for RelayHandler {
    fn default() -> Self {
        Self::new()
    }
}