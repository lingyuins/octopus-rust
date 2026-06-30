use axum::{
    middleware as axum_middleware,
    routing::{get, post, put, delete},
    Router,
};
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};

use crate::handlers;
use crate::middleware;
use crate::static_files;

/// Build the complete Axum application
pub async fn build_app() -> Result<Router, octopus_core::error::AppError> {
    // CORS layer
    let cors = CorsLayer::permissive();

    // API routes (auth required)
    let api_routes = Router::new()
        // Auth — everything except login (which is public above)
        .route("/api/v1/auth/logout", post(handlers::auth::logout))
        .route("/api/v1/auth/refresh", post(handlers::auth::refresh))
        .route("/api/v1/auth/change-password", put(handlers::auth::change_password))
        .route("/api/v1/auth/change-username", put(handlers::auth::change_username))
        // User management
        .route("/api/v1/users", get(handlers::user::list))
        .route("/api/v1/users", post(handlers::user::create))
        .route("/api/v1/users/:id", put(handlers::user::update))
        .route("/api/v1/users/:id", delete(handlers::user::delete))
        // Channel management
        .route("/api/v1/channels", get(handlers::channel::list))
        .route("/api/v1/channels", post(handlers::channel::create))
        .route("/api/v1/channels/:id", get(handlers::channel::get))
        .route("/api/v1/channels/:id", put(handlers::channel::update))
        .route("/api/v1/channels/:id", delete(handlers::channel::delete))
        .route("/api/v1/channels/fetch-model", post(handlers::channel::fetch_model))
        // Group management
        .route("/api/v1/groups", get(handlers::group::list))
        .route("/api/v1/groups", post(handlers::group::create))
        .route("/api/v1/groups/:id", get(handlers::group::get))
        .route("/api/v1/groups/:id", put(handlers::group::update))
        .route("/api/v1/groups/:id", delete(handlers::group::delete))
        // API key management
        .route("/api/v1/api-keys", get(handlers::api_key::list))
        .route("/api/v1/api-keys", post(handlers::api_key::create))
        .route("/api/v1/api-keys/:id", get(handlers::api_key::get))
        .route("/api/v1/api-keys/:id", put(handlers::api_key::update))
        .route("/api/v1/api-keys/:id", delete(handlers::api_key::delete))
        // Settings
        .route("/api/v1/settings", get(handlers::setting::list))
        .route("/api/v1/settings", put(handlers::setting::batch_update))
        .route("/api/v1/settings/:key", get(handlers::setting::get))
        .route("/api/v1/settings/:key", put(handlers::setting::update))
        // Stats
        .route("/api/v1/stats/total", get(handlers::stats::total))
        .route("/api/v1/stats/daily", get(handlers::stats::daily))
        .route("/api/v1/stats/hourly", get(handlers::stats::hourly))
        .route("/api/v1/stats/channel", get(handlers::stats::channel))
        .route("/api/v1/stats/model", get(handlers::stats::model))
        .route("/api/v1/stats/api-key", get(handlers::stats::api_key))
        // Analytics
        .route("/api/v1/analytics/overview", get(handlers::analytics::overview))
        .route("/api/v1/analytics/activity", get(handlers::analytics::activity))
        .route("/api/v1/analytics/latency", get(handlers::analytics::latency_distribution))
        // Logs
        .route("/api/v1/logs", get(handlers::log::list))
        .route("/api/v1/logs/:id", get(handlers::log::get))
        // Audit
        .route("/api/v1/audit", get(handlers::audit::list))
        // Alert
        .route("/api/v1/alerts/rules", get(handlers::alert::list_rules))
        .route("/api/v1/alerts/rules", post(handlers::alert::create_rule))
        .route("/api/v1/alerts/rules/:id", put(handlers::alert::update_rule))
        .route("/api/v1/alerts/rules/:id", delete(handlers::alert::delete_rule))
        .route("/api/v1/alerts/channels", get(handlers::alert::list_channels))
        .route("/api/v1/alerts/channels", post(handlers::alert::create_channel))
        .route("/api/v1/alerts/history", get(handlers::alert::history))
        // Site management
        .route("/api/v1/sites", get(handlers::site::list))
        .route("/api/v1/sites", post(handlers::site::create))
        .route("/api/v1/sites/:id", get(handlers::site::get))
        .route("/api/v1/sites/:id", put(handlers::site::update))
        .route("/api/v1/sites/:id", delete(handlers::site::delete))
        .route("/api/v1/sites/:id/accounts", get(handlers::site::list_accounts))
        .route("/api/v1/sites/:id/accounts", post(handlers::site::create_account))
        .route("/api/v1/sites/:id/sync", post(handlers::site::sync))
        .route("/api/v1/sites/:id/checkin", post(handlers::site::checkin))
        .route("/api/v1/sites/:id/project", post(handlers::site::project))
        // Backup
        .route("/api/v1/backup/export", post(handlers::backup::export_db))
        .route("/api/v1/backup/import", post(handlers::backup::import_db))
        .route("/api/v1/backup/webdav", get(handlers::backup::webdav_list))
        .route("/api/v1/backup/webdav/backup", post(handlers::backup::webdav_backup))
        .route("/api/v1/backup/webdav/restore", post(handlers::backup::webdav_restore))
        // Ops
        .route("/api/v1/ops/health", get(handlers::ops::health))
        .route("/api/v1/ops/telemetry", get(handlers::ops::telemetry))
        .route("/api/v1/ops/cache", get(handlers::ops::cache_status))
        // Update
        .route("/api/v1/update/check", get(handlers::update::check))
        .route("/api/v1/update/apply", post(handlers::update::apply))
        // WebAuthn (register & credential management require auth; auth/begin & auth/finish are public, see public_routes)
        .route("/api/v1/webauthn/register/begin", post(handlers::webauthn::register_begin))
        .route("/api/v1/webauthn/register/finish", post(handlers::webauthn::register_finish))
        .route("/api/v1/webauthn/credentials", get(handlers::webauthn::list_credentials))
        .route("/api/v1/webauthn/credentials/:id", delete(handlers::webauthn::delete_credential))
        // Model
        .route("/api/v1/models", get(handlers::model::list))
        .route("/api/v1/models", post(handlers::model::create))
        .route("/api/v1/models/:id", put(handlers::model::update))
        .route("/api/v1/models/:id", delete(handlers::model::delete))
        .route("/api/v1/models/refresh-price", post(handlers::model::refresh_price))
        .route("/api/v1/models/sync-from-channels", post(handlers::model::sync_from_channels))
        // Model mapping
        .route("/api/v1/model-mappings", get(handlers::model_mapping::list))
        .route("/api/v1/model-mappings", post(handlers::model_mapping::create))
        .route("/api/v1/model-mappings/:id", put(handlers::model_mapping::update))
        .route("/api/v1/model-mappings/:id", delete(handlers::model_mapping::delete))
        // Proxy pool
        .route("/api/v1/proxy", get(handlers::proxy::list))
        .route("/api/v1/proxy", post(handlers::proxy::create))
        .route("/api/v1/proxy/:id", put(handlers::proxy::update))
        .route("/api/v1/proxy/:id", delete(handlers::proxy::delete))
        // API credentials
        .route("/api/v1/credentials", get(handlers::credential::list))
        .route("/api/v1/credentials", post(handlers::credential::create))
        .route("/api/v1/credentials/:id", delete(handlers::credential::delete))
        .route("/api/v1/credentials/cli-export", get(handlers::credential::cli_export))
        // Remote sites (hub)
        .route("/api/v1/remote-sites", get(handlers::remote_site::list))
        .route("/api/v1/remote-sites/balance", get(handlers::remote_site::balance))
        .route("/api/v1/remote-sites/checkin", get(handlers::remote_site::checkin))
        .route("/api/v1/remote-sites/announcements", get(handlers::remote_site::announcements))
        .route("/api/v1/remote-sites/usage-history", get(handlers::remote_site::usage_history))
        .route("/api/v1/remote-sites/redemption", post(handlers::remote_site::redemption))
        // Site channel bindings
        .route("/api/v1/site-channels", get(handlers::site_channel::list))
        .route("/api/v1/site-channels/:id", delete(handlers::site_channel::delete))
        // Media relay
        .route("/v1/images/generations", post(handlers::media::images))
        .route("/v1/audio/transcriptions", post(handlers::media::audio_transcriptions))
        .route("/v1/audio/translations", post(handlers::media::audio_translations))
        .route("/v1/moderations", post(handlers::media::moderations))
        .route("/v1/rerank", post(handlers::media::rerank))
        // Balance history
        .route("/api/v1/balance-history", get(handlers::balance::list))
        .route("/api/v1/usage-history", get(handlers::usage::list))
        // Announcement
        .route("/api/v1/announcements", get(handlers::announcement::list))
        .route("/api/v1/announcements", post(handlers::announcement::create))
        .route("/api/v1/announcements/:id", delete(handlers::announcement::delete))
        // DBMigration
        .route("/api/v1/db-migration/status", get(handlers::db_migration::status))
        .route("/api/v1/db-migration/migrate", post(handlers::db_migration::migrate))
        .layer(axum_middleware::from_fn(middleware::auth::auth_middleware))
        // Management write audit
        .layer(axum_middleware::from_fn(middleware::audit::audit_middleware));

    // Public routes — no auth required (login, bootstrap, verification, passkey login)
    let public_routes = Router::new()
        // Bootstrap (first-run setup, must work before any user exists)
        .route("/api/v1/bootstrap/status", get(handlers::bootstrap::status))
        .route("/api/v1/bootstrap/create", post(handlers::bootstrap::create))
        // Auth login (exchanges credentials for a JWT; refresh/logout stay protected)
        .route("/api/v1/auth/login", post(handlers::auth::login))
        // Verification (email/code send & verify, used during signup flows)
        .route("/api/v1/verification/send", post(handlers::verification::send))
        .route("/api/v1/verification/verify", post(handlers::verification::verify))
        // WebAuthn login (passkey authentication begins unauthenticated)
        .route("/api/v1/webauthn/auth/begin", post(handlers::webauthn::auth_begin))
        .route("/api/v1/webauthn/auth/finish", post(handlers::webauthn::auth_finish));

    // Relay routes (API key auth, no management auth)
    let relay_routes = Router::new()
        .route("/v1/chat/completions", post(handlers::relay::chat))
        .route("/v1/responses", post(handlers::relay::responses))
        .route("/v1/messages", post(handlers::relay::messages))
        .route("/v1/embeddings", post(handlers::relay::embeddings))
        .layer(axum_middleware::from_fn(middleware::api_key::api_key_middleware));

    // Combine all routes
    let app = Router::new()
        .merge(public_routes)
        .merge(api_routes)
        .merge(relay_routes)
        // Static files for frontend. Serves static/out, falls back to web/out,
        // and finally returns index.html for client-side routing (SPA deep links).
        .fallback_service(static_files::static_service())
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    Ok(app)
}