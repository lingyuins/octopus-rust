use axum::{http::StatusCode, Json};
use crate::response::{ApiResponse, ok_response};

pub async fn register_begin() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn register_finish() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn auth_begin() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn auth_finish() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn list_credentials() -> Result<(StatusCode, Json<ApiResponse<Vec<()>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn delete_credential() -> Result<StatusCode, (StatusCode, Json<ApiResponse<()>>)> { Ok(StatusCode::NO_CONTENT) }