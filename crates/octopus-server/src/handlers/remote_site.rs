use axum::{http::StatusCode, Json};
use crate::response::{ApiResponse, ok_response};
pub async fn list() -> Result<(StatusCode, Json<ApiResponse<Vec<()>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn balance() -> Result<(StatusCode, Json<ApiResponse<Vec<()>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn checkin() -> Result<(StatusCode, Json<ApiResponse<Vec<()>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn announcements() -> Result<(StatusCode, Json<ApiResponse<Vec<()>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn usage_history() -> Result<(StatusCode, Json<ApiResponse<Vec<()>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn redemption() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }