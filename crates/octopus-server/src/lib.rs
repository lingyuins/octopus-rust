pub mod app;
pub mod middleware;
pub mod handlers;
pub mod response;
pub mod static_files;

use tokio::net::TcpListener;

pub async fn start(addr: &str) -> Result<(), octopus_core::error::AppError> {
    let app = app::build_app().await?;

    let listener = TcpListener::bind(addr).await.map_err(|e| {
        octopus_core::error::AppError::Internal(format!("failed to bind to {}: {}", addr, e))
    })?;

    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app)
        .await
        .map_err(|e| octopus_core::error::AppError::Internal(format!("server error: {}", e)))?;

    Ok(())
}

pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    tracing::info!("Received shutdown signal");
}