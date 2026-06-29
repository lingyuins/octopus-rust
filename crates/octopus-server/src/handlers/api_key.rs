use axum::{extract::{Path, Query}, http::StatusCode, Json};
use serde::Deserialize;
use octopus_db::ops::api_key as key_ops;
use octopus_core::model::api_key::{ApiKey, ApiKeyCreateRequest, ApiKeyUpdateRequest};
use crate::middleware::auth::generate_api_key;
use crate::response::{ApiResponse, ok_response, created_response, not_found, PaginatedResponse};

#[derive(Deserialize)] pub struct KeyQuery { pub page: Option<i32>, pub page_size: Option<i32> }

pub async fn list(Query(q): Query<KeyQuery>) -> Result<(StatusCode, Json<ApiResponse<PaginatedResponse<ApiKey>>>), (StatusCode, Json<ApiResponse<()>>)> {
    let keys = key_ops::list_api_keys().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    let total = keys.len() as i64;
    Ok(ok_response(PaginatedResponse::new(keys, total, q.page.unwrap_or(1), q.page_size.unwrap_or(100))))
}
pub async fn get(Path(id): Path<i32>) -> Result<(StatusCode, Json<ApiResponse<ApiKey>>), (StatusCode, Json<ApiResponse<()>>)> {
    let k = key_ops::get_api_key(id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?.ok_or_else(|| not_found("API key not found"))?;
    Ok(ok_response(k))
}
pub async fn create(Json(req): Json<ApiKeyCreateRequest>) -> Result<(StatusCode, Json<ApiResponse<ApiKey>>), (StatusCode, Json<ApiResponse<()>>)> {
    let key = generate_api_key();
    let new_key = ApiKey {
        id: 0, key, name: req.name, enabled: true, allowed_models: req.allowed_models,
        expiry: req.expiry, max_cost: req.max_cost, rpm: req.rpm, tpm: req.tpm,
        per_model_quotas: req.per_model_quotas, ip_allowlist: req.ip_allowlist,
        created_at: chrono::Utc::now().to_rfc3339(), last_used_at: None, total_cost: 0.0, remark: req.remark,
    };
    let created = key_ops::create_api_key(&new_key).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    Ok(created_response(created))
}
pub async fn update(Path(id): Path<i32>, Json(req): Json<ApiKeyUpdateRequest>) -> Result<(StatusCode, Json<ApiResponse<ApiKey>>), (StatusCode, Json<ApiResponse<()>>)> {
    key_ops::update_api_key(id, &req).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    let k = key_ops::get_api_key(id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?.ok_or_else(|| not_found("API key not found"))?;
    Ok(ok_response(k))
}
pub async fn delete(Path(id): Path<i32>) -> Result<StatusCode, (StatusCode, Json<ApiResponse<()>>)> {
    key_ops::delete_api_key(id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    Ok(StatusCode::NO_CONTENT)
}