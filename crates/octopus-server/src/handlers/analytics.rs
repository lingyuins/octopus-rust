use axum::{http::StatusCode, Json};
use octopus_core::model::stats::*;
use crate::response::{ApiResponse, ok_response};

pub async fn overview() -> Result<(StatusCode, Json<ApiResponse<AnalyticsOverview>>), (StatusCode, Json<ApiResponse<()>>)> {
    Ok(ok_response(AnalyticsOverview { total_requests: 0, total_tokens: 0, total_cost: 0.0, success_rate: 100.0, avg_latency: 0.0, active_channels: 0, active_keys: 0, today_requests: 0, today_cost: 0.0 }))
}
pub async fn activity() -> Result<(StatusCode, Json<ApiResponse<Vec<ActivityEntry>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }
pub async fn latency_distribution() -> Result<(StatusCode, Json<ApiResponse<Vec<LatencyDistribution>>>), (StatusCode, Json<ApiResponse<()>>)> { Ok(ok_response(vec![])) }