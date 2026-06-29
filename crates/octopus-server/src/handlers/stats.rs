use axum::{http::StatusCode, Json};
use octopus_db::ops::stats as stats_ops;
use octopus_core::model::stats::*;
use crate::response::{ApiResponse, ok_response};

pub async fn total() -> Result<(StatusCode, Json<ApiResponse<StatsTotal>>), (StatusCode, Json<ApiResponse<()>>)> {
    let s = stats_ops::get_total_stats().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?.unwrap_or(StatsTotal { id: 1, total_requests: 0, total_tokens: 0, total_cost: 0.0, success_count: 0, fail_count: 0, updated_at: String::new() });
    Ok(ok_response(s))
}
pub async fn daily() -> Result<(StatusCode, Json<ApiResponse<Vec<StatsDaily>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn hourly() -> Result<(StatusCode, Json<ApiResponse<Vec<StatsHourly>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn channel() -> Result<(StatusCode, Json<ApiResponse<Vec<StatsChannel>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn model() -> Result<(StatusCode, Json<ApiResponse<Vec<StatsModel>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn api_key() -> Result<(StatusCode, Json<ApiResponse<Vec<StatsApiKey>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }