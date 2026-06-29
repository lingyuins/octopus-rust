use axum::{
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use tower_http::services::{ServeDir, ServeFile};

/// Public assets served by the frontend (Next.js static export).
const STATIC_DIR: &str = "static/out";

/// Build the static-asset service for the frontend.
///
/// Serves files from `static/out` (the production build, copied from `web/out`).
/// When a path does not match a static file — e.g. a client-side deep link such
/// as `/channels` or a refresh on a nested route — falls back to `index.html` so
/// the SPA can handle routing instead of returning a 404.
///
/// `ServeFile` (rather than a handler) is used as the fallback service so the
/// resulting `ServeDir<ServeFile>` keeps a `Send` future and a response body
/// type compatible with `Router::fallback_service`. (`not_found_service` wraps
/// the fallback in `SetStatus`, breaking both.)
pub fn static_service() -> ServeDir<ServeFile> {
    ServeDir::new(STATIC_DIR).fallback(ServeFile::new(format!("{}/index.html", STATIC_DIR)))
}

/// SPA fallback handler: serve `index.html` for any path not matching a static file.
///
/// Currently unused by `build_app` (which wires `static_service()` with a
/// `ServeFile` not-found fallback), but kept as a handler-based alternative.
#[allow(dead_code)]
pub async fn spa_fallback() -> impl IntoResponse {
    let index_path = format!("{}/index.html", STATIC_DIR);
    match tokio::fs::read_to_string(&index_path).await {
        Ok(content) => (
            StatusCode::OK,
            [("content-type", "text/html; charset=utf-8")],
            content,
        )
            .into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            [("content-type", "text/plain; charset=utf-8")],
            "Frontend not built. Run `cd web && pnpm build` first, then copy `web/out` to `static/out`.".to_string(),
        )
            .into_response(),
    }
}

/// Mount the static service on a router (kept for API symmetry with other
/// builders; currently unused because `build_app` wires `static_service()`
/// directly as the router's `fallback_service`).
#[allow(dead_code)]
pub fn static_router() -> Router {
    Router::new().fallback_service(static_service())
}
