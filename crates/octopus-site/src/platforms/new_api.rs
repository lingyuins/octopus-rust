// NewAPI platform adapter
pub async fn fetch_models(base_url: &str, token: &str) -> Result<Vec<String>, octopus_core::error::AppError> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/models", base_url.trim_end_matches('/')))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    let json: serde_json::Value = resp.json().await?;
    Ok(json.get("data").and_then(|d| d.as_array()).map(|a| a.iter().filter_map(|v| v.get("id").and_then(|i| i.as_str()).map(String::from)).collect()).unwrap_or_default())
}