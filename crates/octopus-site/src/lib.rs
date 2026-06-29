pub mod sync;
pub mod checkin;
pub mod balance;
pub mod project;
pub mod platforms;
pub mod detect;

use octopus_core::types::SitePlatform;

/// Sync a site account
pub async fn sync_account(site_id: i32, account_id: i32) -> Result<(), octopus_core::error::AppError> {
    tracing::info!("Syncing site {} account {}", site_id, account_id);
    Ok(())
}

/// Sync all accounts for a site
pub async fn sync_all(site_id: i32) -> Result<(), octopus_core::error::AppError> {
    tracing::info!("Syncing all accounts for site {}", site_id);
    Ok(())
}

/// Checkin a site account
pub async fn checkin_account(site_id: i32, account_id: i32) -> Result<(), octopus_core::error::AppError> {
    tracing::info!("Checkin site {} account {}", site_id, account_id);
    Ok(())
}

/// Detect platform type from URL
pub fn detect_platform(url: &str) -> Option<SitePlatform> {
    let lower = url.to_lowercase();
    if lower.contains("new-api") || lower.contains("newapi") { Some(SitePlatform::NewAPI) }
    else if lower.contains("one-api") || lower.contains("oneapi") { Some(SitePlatform::OneAPI) }
    else if lower.contains("one-hub") || lower.contains("onehub") { Some(SitePlatform::OneHub) }
    else if lower.contains("sub2api") { Some(SitePlatform::Sub2API) }
    else if lower.contains("anyrouter") { Some(SitePlatform::AnyRouter) }
    else { None }
}