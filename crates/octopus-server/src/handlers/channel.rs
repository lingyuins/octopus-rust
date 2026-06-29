use axum::{extract::{Path, Query}, http::StatusCode, Json};
use serde::Deserialize;
use octopus_db::ops::channel as channel_ops;
use octopus_core::model::channel::{Channel, ChannelUpdateRequest, ChannelFetchModelRequest};
use crate::response::{ApiResponse, ok_response, created_response, not_found, PaginatedResponse};

#[derive(Deserialize)]
pub struct ChannelQuery { pub page: Option<i32>, pub page_size: Option<i32> }

pub async fn list(Query(q): Query<ChannelQuery>) -> Result<(StatusCode, Json<ApiResponse<PaginatedResponse<Channel>>>), (StatusCode, Json<ApiResponse<()>>)> {
    let chs = channel_ops::list_channels().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    let total = chs.len() as i64;
    Ok(ok_response(PaginatedResponse::new(chs, total, q.page.unwrap_or(1), q.page_size.unwrap_or(100))))
}
pub async fn get(Path(id): Path<i32>) -> Result<(StatusCode, Json<ApiResponse<Channel>>), (StatusCode, Json<ApiResponse<()>>)> {
    let ch = channel_ops::get_channel(id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?.ok_or_else(|| not_found("Channel not found"))?;
    Ok(ok_response(ch))
}
pub async fn create(Json(req): Json<Channel>) -> Result<(StatusCode, Json<ApiResponse<Channel>>), (StatusCode, Json<ApiResponse<()>>)> {
    let created = channel_ops::create_channel(&req).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    Ok(created_response(created))
}
pub async fn update(Path(id): Path<i32>, Json(req): Json<ChannelUpdateRequest>) -> Result<(StatusCode, Json<ApiResponse<Channel>>), (StatusCode, Json<ApiResponse<()>>)> {
    channel_ops::update_channel(id, &req).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    let ch = channel_ops::get_channel(id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?.ok_or_else(|| not_found("Channel not found"))?;
    Ok(ok_response(ch))
}
pub async fn delete(Path(id): Path<i32>) -> Result<StatusCode, (StatusCode, Json<ApiResponse<()>>)> {
    channel_ops::delete_channel(id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn fetch_model(Json(_req): Json<ChannelFetchModelRequest>) -> Result<(StatusCode, Json<ApiResponse<Vec<String>>>), (StatusCode, Json<ApiResponse<()>>)> {
    Ok(ok_response(vec![]))
}