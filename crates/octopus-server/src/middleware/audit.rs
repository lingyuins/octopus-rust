use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use octopus_db::ops::audit;

/// Audit middleware for management write operations
pub async fn audit_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let response = next.run(req).await;

    // Only audit write operations
    if matches!(method, axum::http::Method::POST | axum::http::Method::PUT | axum::http::Method::DELETE | axum::http::Method::PATCH) {
        let status = response.status();
        if status.is_success() || status.is_client_error() {
            // Extract user info from response extensions
            let user_id = response.extensions().get::<i32>().copied().unwrap_or(0);
            let username = response.extensions().get::<String>().map(|s| s.as_str()).unwrap_or("unknown");

            let action = format!("{} {}", method, path);
            let resource = path.split('/').nth(2).unwrap_or("unknown");

            let _ = audit::create_audit_log(
                user_id,
                username,
                &action,
                resource,
                None,
                None,
                &ip,
                &user_agent,
            ).await;
        }
    }

    response
}