// AxonHub integration
pub async fn fetch_balance(url: &str, token: &str) -> Result<f64, octopus_core::error::AppError> {
    super::fetch_balance(url, token).await
}
// ClaudeCodeHub integration
pub async fn fetch_balance_cc(url: &str, token: &str) -> Result<f64, octopus_core::error::AppError> {
    super::fetch_balance(url, token).await
}
// LDoH integration
pub async fn fetch_balance_ldoh(url: &str, token: &str) -> Result<f64, octopus_core::error::AppError> {
    super::fetch_balance(url, token).await
}
// Octopus hub integration
pub async fn fetch_balance_octo(url: &str, token: &str) -> Result<f64, octopus_core::error::AppError> {
    super::fetch_balance(url, token).await
}
// SAPI integration
pub async fn fetch_balance_sapi(url: &str, token: &str) -> Result<f64, octopus_core::error::AppError> {
    super::fetch_balance(url, token).await
}
// Sub2API integration
pub async fn fetch_balance_sub2(url: &str, token: &str) -> Result<f64, octopus_core::error::AppError> {
    super::fetch_balance(url, token).await
}