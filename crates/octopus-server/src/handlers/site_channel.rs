use axum::{http::StatusCode, Json};
use crate::response::{ApiResponse, ok_response};
pub async fn list() -> Result<(StatusCode, Json<ApiResponse<Vec<()>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn delete() -> Result<StatusCode, (StatusCode, Json<ApiResponse<()>>)> { Ok(StatusCode::NO_CONTENT) }