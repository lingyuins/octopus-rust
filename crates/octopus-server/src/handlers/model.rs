use axum::{http::StatusCode, Json};
use crate::response::{ApiResponse, ok_response};
use octopus_core::model::llm::LLMInfo;

pub async fn list() -> Result<(StatusCode, Json<ApiResponse<Vec<LLMInfo>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn create() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn update() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn delete() -> Result<StatusCode, (StatusCode, Json<ApiResponse<()>>)> { Ok(StatusCode::NO_CONTENT) }
pub async fn refresh_price() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }
pub async fn sync_from_channels() -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(())) }