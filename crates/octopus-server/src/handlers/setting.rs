use axum::{extract::Path, http::StatusCode, Json};
use octopus_db::ops::setting as setting_ops;
use octopus_core::model::setting::{Setting, SettingUpdateRequest, SettingBatchUpdateRequest};
use crate::response::{ApiResponse, ok_response, not_found};

pub async fn list() -> Result<(StatusCode, Json<ApiResponse<Vec<Setting>>>), (StatusCode, Json<ApiResponse<()>>)> {
    let s = setting_ops::list_settings().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    Ok(ok_response(s))
}
pub async fn get(Path(key): Path<String>) -> Result<(StatusCode, Json<ApiResponse<Setting>>), (StatusCode, Json<ApiResponse<()>>)> {
    let s = setting_ops::get_setting(&key).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?.ok_or_else(|| not_found("Setting not found"))?;
    Ok(ok_response(s))
}
pub async fn update(Path(key): Path<String>, Json(req): Json<SettingUpdateRequest>) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> {
    setting_ops::upsert_setting(&key, &req.value).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    Ok(ok_response(()))
}
pub async fn batch_update(Json(req): Json<SettingBatchUpdateRequest>) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> {
    for s in &req.settings {
        setting_ops::upsert_setting(&s.key, &s.value).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    }
    Ok(ok_response(()))
}