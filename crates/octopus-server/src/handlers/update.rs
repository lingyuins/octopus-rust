use axum::{http::StatusCode, Json};
use crate::response::{ApiResponse, ok_response};

pub async fn check() -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), (StatusCode, Json<ApiResponse<()>>)> {
    Ok(ok_response(serde_json::json!({"latest": "v2.0.0", "current": "v2.0.0", "update_available": false})))
}
pub async fn apply() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }