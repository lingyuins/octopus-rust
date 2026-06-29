pub mod aihubmix;
pub mod axonhub;
pub mod claudecodehub;
pub mod ldoh;
pub mod octopus_hub;
pub mod sapi;
pub mod sub2api;

/// Hub integration - manages connections to third-party LLM gateway platforms
pub struct HubManager;

impl HubManager {
    pub fn new() -> Self { Self }
}

impl Default for HubManager {
    fn default() -> Self { Self::new() }
}

/// Fetch balance from a hub platform
pub async fn fetch_balance(url: &str, token: &str) -> Result<f64, octopus_core::error::AppError> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/balance", url.trim_end_matches('/')))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    let json: serde_json::Value = resp.json().await?;
    Ok(json.get("balance").and_then(|v| v.as_f64()).unwrap_or(0.0))
}

/// Fetch announcements from a hub platform
pub async fn fetch_announcements(url: &str, token: &str) -> Result<Vec<serde_json::Value>, octopus_core::error::AppError> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/announcements", url.trim_end_matches('/')))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    let json: serde_json::Value = resp.json().await?;
    Ok(json.get("data").and_then(|v| v.as_array()).cloned().unwrap_or_default())
}