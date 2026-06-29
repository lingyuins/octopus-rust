use axum::{extract::{Path, Query}, http::StatusCode, Json};
use serde::Deserialize;
use octopus_db::ops::group as group_ops;
use octopus_core::model::group::{Group, GroupUpdateRequest};
use crate::response::{ApiResponse, ok_response, created_response, not_found, PaginatedResponse};

#[derive(Deserialize)] pub struct GroupQuery { pub page: Option<i32>, pub page_size: Option<i32> }

pub async fn list(Query(q): Query<GroupQuery>) -> Result<(StatusCode, Json<ApiResponse<PaginatedResponse<Group>>>), (StatusCode, Json<ApiResponse<()>>)> {
    let gs = group_ops::list_groups().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    let total = gs.len() as i64;
    Ok(ok_response(PaginatedResponse::new(gs, total, q.page.unwrap_or(1), q.page_size.unwrap_or(100))))
}
pub async fn get(Path(id): Path<i32>) -> Result<(StatusCode, Json<ApiResponse<Group>>), (StatusCode, Json<ApiResponse<()>>)> {
    let g = group_ops::get_group(id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?.ok_or_else(|| not_found("Group not found"))?;
    Ok(ok_response(g))
}
pub async fn create(Json(req): Json<Group>) -> Result<(StatusCode, Json<ApiResponse<Group>>), (StatusCode, Json<ApiResponse<()>>)> {
    let created = group_ops::create_group(&req).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    Ok(created_response(created))
}
pub async fn update(Path(id): Path<i32>, Json(req): Json<GroupUpdateRequest>) -> Result<(StatusCode, Json<ApiResponse<Group>>), (StatusCode, Json<ApiResponse<()>>)> {
    group_ops::update_group(id, &req).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    let g = group_ops::get_group(id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?.ok_or_else(|| not_found("Group not found"))?;
    Ok(ok_response(g))
}
pub async fn delete(Path(id): Path<i32>) -> Result<StatusCode, (StatusCode, Json<ApiResponse<()>>)> {
    group_ops::delete_group(id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    Ok(StatusCode::NO_CONTENT)
}