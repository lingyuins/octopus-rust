use axum::{extract::Path, http::StatusCode, Json};
use octopus_db::ops::alert as alert_ops;
use octopus_core::model::alert::*;
use crate::response::{ApiResponse, ok_response, created_response};

pub async fn list_rules() -> Result<(StatusCode, Json<ApiResponse<Vec<AlertRule>>>), (StatusCode, Json<ApiResponse<()>>)> {
    let rules = alert_ops::list_alert_rules().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    Ok(ok_response(rules))
}
pub async fn create_rule(Json(rule): Json<AlertRule>) -> Result<(StatusCode, Json<ApiResponse<AlertRule>>), (StatusCode, Json<ApiResponse<()>>)> {
    let created = alert_ops::create_alert_rule(&rule).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    Ok(created_response(created))
}
pub async fn update_rule(Path(id): Path<i32>, Json(updates): Json<AlertRuleRequest>) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> {
    alert_ops::update_alert_rule(id, &updates).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    Ok(ok_response(()))
}
pub async fn delete_rule(Path(id): Path<i32>) -> Result<StatusCode, (StatusCode, Json<ApiResponse<()>>)> {
    alert_ops::delete_alert_rule(id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn list_channels() -> Result<(StatusCode, Json<ApiResponse<Vec<AlertNotifChannel>>>), (StatusCode, Json<ApiResponse<()>>)> {
    let chs = alert_ops::list_alert_notif_channels().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    Ok(ok_response(chs))
}
pub async fn create_channel(Json(_ch): Json<AlertNotifChannel>) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> {
    Ok(ok_response(()))
}
pub async fn history() -> Result<(StatusCode, Json<ApiResponse<Vec<AlertHistory>>>), (StatusCode, Json<ApiResponse<()>>)> {
    Ok(ok_response(vec![]))
}