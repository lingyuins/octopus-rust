use axum::{http::StatusCode, Json};
use crate::response::{ApiResponse, ok_response};
pub async fn send() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn verify() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }