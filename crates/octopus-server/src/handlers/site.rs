use axum::{http::StatusCode, Json};
use crate::response::{ApiResponse, ok_response};
pub async fn list() -> Result<(StatusCode, Json<ApiResponse<Vec<()>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn create() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn get() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn update() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn delete() -> Result<StatusCode, (StatusCode, Json<ApiResponse<()>>)> { Ok(StatusCode::NO_CONTENT) }
pub async fn list_accounts() -> Result<(StatusCode, Json<ApiResponse<Vec<()>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn create_account() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn sync() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn checkin() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn project() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }