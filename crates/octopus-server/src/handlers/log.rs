use axum::{extract::Query, http::StatusCode, Json};
use serde::Deserialize;
use octopus_core::model::relay_log::RelayLog;
use crate::response::{ApiResponse, ok_response, PaginatedResponse};

#[derive(Deserialize)] pub struct LogQuery { pub page: Option<i32>, pub page_size: Option<i32>, pub request_id: Option<String>, pub channel_id: Option<i32>, pub status_code: Option<i32> }

pub async fn list(Query(_q): Query<LogQuery>) -> Result<(StatusCode, Json<ApiResponse<PaginatedResponse<RelayLog>>>), (StatusCode, Json<ApiResponse<()>>)> {
    Ok(ok_response(PaginatedResponse::new(vec![], 0, 1, 20)))
}
pub async fn get(axum::extract::Path(_id): axum::extract::Path<i64>) -> Result<(StatusCode, Json<ApiResponse<RelayLog>>), (StatusCode, Json<ApiResponse<()>>)> {
    Err((StatusCode::NOT_FOUND, Json(ApiResponse::err("Log not found"))))
}