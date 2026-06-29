use axum::{http::StatusCode, Json};
use crate::response::{ApiResponse, ok_response};
pub async fn status() -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), (StatusCode, Json<ApiResponse<()>>)> {
    Ok(ok_response(serde_json::json!({"current_type": "sqlite", "available_types": ["sqlite", "mysql", "postgres"]})))
}
pub async fn migrate() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }