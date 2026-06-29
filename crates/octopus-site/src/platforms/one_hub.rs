// OneHub adapter
pub async fn fetch_models(base_url: &str, token: &str) -> Result<Vec<String>, octopus_core::error::AppError> {
    super::new_api::fetch_models(base_url, token).await
}
// Sub2API adapter
pub async fn fetch_models_sub2api(base_url: &str, token: &str) -> Result<Vec<String>, octopus_core::error::AppError> {
    super::new_api::fetch_models(base_url, token).await
}
// AnyRouter adapter
pub async fn fetch_models_anyrouter(base_url: &str, token: &str) -> Result<Vec<String>, octopus_core::error::AppError> {
    super::new_api::fetch_models(base_url, token).await
}