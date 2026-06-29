use axum::{http::StatusCode, Json};
use crate::response::{ApiResponse, ok_response};

pub async fn health() -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    (StatusCode::OK, Json(ApiResponse::success(serde_json::json!({"status": "ok", "uptime": 0}))))
}
pub async fn telemetry() -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), (StatusCode, Json<ApiResponse<()>>)> {
    Ok(ok_response(serde_json::json!({"goroutines": 0, "memory_mb": 0, "cpu_percent": 0.0})))
}
pub async fn cache_status() -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), (StatusCode, Json<ApiResponse<()>>)> {
    Ok(ok_response(serde_json::json!({"channels": 0, "groups": 0, "api_keys": 0, "users": 0})))
}