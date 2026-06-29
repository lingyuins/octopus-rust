use axum::{http::StatusCode, Json};
use crate::response::{ApiResponse, ok_response};
pub async fn list() -> Result<(StatusCode, Json<ApiResponse<Vec<()>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn create() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn delete() -> Result<StatusCode, (StatusCode, Json<ApiResponse<()>>)> { Ok(StatusCode::NO_CONTENT) }
pub async fn cli_export() -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), (StatusCode, Json<ApiResponse<()>>)> {
    Ok(ok_response(serde_json::json!({"claude_code": "", "codex": "", "gemini_cli": "", "cherry_studio": "", "kilo_code": ""})))
}