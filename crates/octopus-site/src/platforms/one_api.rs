// OneAPI adapter
pub async fn fetch_models(base_url: &str, token: &str) -> Result<Vec<String>, octopus_core::error::AppError> {
    super::new_api::fetch_models(base_url, token).await
}