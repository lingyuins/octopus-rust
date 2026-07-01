use axum::{http::StatusCode, Json};
use octopus_core::model::user::{User, UserBootstrapCreate, UserInfo, ROLE_ADMIN};
use octopus_db::ops::user as user_ops;
use crate::response::{ApiResponse, bad_request, conflict, error_response};

/// Bootstrap status — reports whether the instance has been initialized
/// (i.e. at least one admin user exists) plus build metadata.
///
/// This is a public route (no auth) so the frontend can decide whether to
/// show the first-run setup screen. It is also the Docker HEALTHCHECK target,
/// so it must reflect real state rather than a constant.
pub async fn status() -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let user_count = user_ops::count_users().await.unwrap_or(0);
    let status = serde_json::json!({
        "initialized": user_count > 0,
        "version": env!("CARGO_PKG_VERSION"),
        "db_type": octopus_db::is_sqlite().then_some("sqlite").unwrap_or("unknown"),
        "user_count": user_count,
    });
    (StatusCode::OK, Json(ApiResponse::success(status)))
}

/// Create the first admin user (first-run bootstrap).
///
/// Only succeeds when no user exists yet; subsequent calls return 409.
/// The password is hashed with argon2 before being stored.
pub async fn create(
    Json(req): Json<UserBootstrapCreate>,
) -> Result<(StatusCode, Json<ApiResponse<UserInfo>>), (StatusCode, Json<ApiResponse<()>>)> {
    // Reject bootstrap once an admin already exists.
    if user_ops::count_users().await.map_err(error_response)? > 0 {
        return Err(conflict("Instance already initialized"));
    }

    if req.username.trim().is_empty() || req.password.is_empty() {
        return Err(bad_request("Username and password are required"));
    }

    let mut user = User {
        id: 0,
        username: req.username,
        password: req.password,
        role: ROLE_ADMIN.to_string(),
    };
    user.hash_password().map_err(error_response)?;

    let created = user_ops::create_user(&user).await.map_err(error_response)?;
    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(UserInfo::from(created))),
    ))
}
