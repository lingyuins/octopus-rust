use axum::{http::StatusCode, Json};
use octopus_db::ops::user as user_ops;
use octopus_core::model::user::{UserLogin, UserLoginResponse, UserChangePassword, UserChangeUsername};
use crate::middleware::auth::{generate_token, get_claims, JwtClaims};
use crate::response::{ApiResponse, ok_response, error_response, bad_request, not_found};
use octopus_core::AppConfig;

/// Login
pub async fn login(
    Json(req): Json<UserLogin>,
) -> Result<(StatusCode, Json<ApiResponse<UserLoginResponse>>), (StatusCode, Json<ApiResponse<()>>)> {
    let user = user_ops::get_user_by_username(&req.username)
        .await
        .map_err(|e| error_response(e))?
        .ok_or_else(|| bad_request("Invalid username or password"))?;

    if !user.verify_password(&req.password).unwrap_or(false) {
        return Err(bad_request("Invalid username or password"));
    }

    let config = AppConfig::load(None).unwrap_or_default();
    let (token, expire_at) = generate_token(
        user.id,
        &user.role,
        req.expire,
        &config.auth.jwt_secret,
    ).map_err(|e| error_response(e))?;

    Ok(ok_response(UserLoginResponse { token, expire_at }))
}

/// Logout (client-side token discard)
pub async fn logout() -> (StatusCode, Json<ApiResponse<()>>) {
    ok_response(())
}

/// Refresh token
pub async fn refresh(
    req: axum::extract::Request,
) -> Result<(StatusCode, Json<ApiResponse<UserLoginResponse>>), (StatusCode, Json<ApiResponse<()>>)> {
    let claims = get_claims(&req).ok_or_else(|| bad_request("Unauthorized"))?;
    let user_id = claims.user_id.unwrap_or(0);
    let role = claims.role.as_deref().unwrap_or("admin");

    let config = AppConfig::load(None).unwrap_or_default();
    let (token, expire_at) = generate_token(
        user_id,
        role,
        0, // default expiry
        &config.auth.jwt_secret,
    ).map_err(|e| error_response(e))?;

    Ok(ok_response(UserLoginResponse { token, expire_at }))
}

/// Change password
pub async fn change_password(
    axum::Extension(claims): axum::Extension<JwtClaims>,
    Json(body): Json<UserChangePassword>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> {
    let user_id = claims.user_id.unwrap_or(0);

    let user = user_ops::get_user(user_id)
        .await
        .map_err(|e| error_response(e))?
        .ok_or_else(|| not_found("User not found"))?;

    if !user.verify_password(&body.old_password).unwrap_or(false) {
        return Err(bad_request("Old password is incorrect"));
    }

    let mut new_user = user.clone();
    new_user.password = body.new_password;
    new_user.hash_password().map_err(|e| error_response(e))?;

    user_ops::update_user(user_id, None, Some(&new_user.password), None)
        .await
        .map_err(|e| error_response(e))?;

    Ok(ok_response(()))
}

/// Change username
pub async fn change_username(
    axum::Extension(claims): axum::Extension<JwtClaims>,
    Json(body): Json<UserChangeUsername>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> {
    let user_id = claims.user_id.unwrap_or(0);

    user_ops::update_user(user_id, Some(&body.new_username), None, None)
        .await
        .map_err(|e| error_response(e))?;

    Ok(ok_response(()))
}