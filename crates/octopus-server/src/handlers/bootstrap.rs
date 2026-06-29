use axum::{http::StatusCode, Json};
use crate::response::ApiResponse;
use octopus_core::model::user::UserInfo;

pub async fn status() -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let status = serde_json::json!({
        "initialized": true,
        "version": "2.0.0",
        "db_type": "sqlite",
        "user_count": 0
    });
    (StatusCode::OK, Json(ApiResponse::success(status)))
}
pub async fn create(Json(_req): Json<octopus_core::model::user::UserBootstrapCreate>) -> Result<(StatusCode, Json<ApiResponse<UserInfo>>), (StatusCode, Json<ApiResponse<()>>)> {
    Ok((StatusCode::CREATED, Json(ApiResponse::success(UserInfo { id: 1, username: "admin".into(), role: "admin".into() }))))
}