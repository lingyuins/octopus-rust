// Octopus Hub
pub async fn fetch_balance(url: &str, token: &str) -> Result<f64, octopus_core::error::AppError> { super::fetch_balance(url, token).await }