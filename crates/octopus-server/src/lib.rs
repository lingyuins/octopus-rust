pub mod app;
pub mod middleware;
pub mod handlers;
pub mod response;
pub mod static_files;

use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::watch;

/// Start the HTTP server.
///
/// `config` is shared with all handlers via `axum::State` so that the config
/// file is read exactly once at startup instead of on every request.
/// `shutdown_rx` fires when the process receives a shutdown signal; it is
/// forwarded to `axum::serve` so in-flight requests drain before exit.
pub async fn start(
    addr: &str,
    config: Arc<octopus_core::AppConfig>,
    shutdown_rx: watch::Receiver<bool>,
) -> Result<(), octopus_core::error::AppError> {
    let app = app::build_app(config).await?;

    let listener = TcpListener::bind(addr).await.map_err(|e| {
        octopus_core::error::AppError::Internal(format!("failed to bind to {}: {}", addr, e))
    })?;

    tracing::info!("Server listening on {}", addr);

    // `into_make_service_with_connect_info` injects the peer `SocketAddr` as
    // `ConnectInfo` into each request's extensions, which the audit middleware
    // uses to honor `server.trusted_proxies`.
    axum::serve(listener, app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .with_graceful_shutdown(async move {
            let mut rx = shutdown_rx;
            // Either the caller signals shutdown or the channel closes.
            let _ = rx.changed().await;
            tracing::info!("Graceful shutdown signaled, draining in-flight requests");
        })
        .await
        .map_err(|e| octopus_core::error::AppError::Internal(format!("server error: {}", e)))?;

    Ok(())
}
