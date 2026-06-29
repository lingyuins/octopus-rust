use axum::{http::StatusCode, Json};
use crate::response::{ApiResponse, ok_response};
pub async fn export_db() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn import_db() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn webdav_list() -> Result<(StatusCode, Json<ApiResponse<Vec<()>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn webdav_backup() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn webdav_restore() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }