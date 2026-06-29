use axum::{extract::Query, http::StatusCode, Json};
use serde::Deserialize;
use octopus_db::ops::audit as audit_ops;
use octopus_core::model::audit::AuditLog;
use crate::response::{ApiResponse, ok_response, PaginatedResponse};

#[derive(Deserialize)] pub struct AuditQuery { pub page: Option<i32>, pub page_size: Option<i32>, pub user_id: Option<i32>, pub action: Option<String>, pub resource: Option<String>, pub start_date: Option<String>, pub end_date: Option<String> }

pub async fn list(Query(q): Query<AuditQuery>) -> Result<(StatusCode, Json<ApiResponse<PaginatedResponse<AuditLog>>>), (StatusCode, Json<ApiResponse<()>>)> {
    let (logs, total) = audit_ops::list_audit_logs(q.page.unwrap_or(1), q.page_size.unwrap_or(20), q.user_id, q.action.as_deref(), q.resource.as_deref(), q.start_date.as_deref(), q.end_date.as_deref()).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    Ok(ok_response(PaginatedResponse::new(logs, total, q.page.unwrap_or(1), q.page_size.unwrap_or(20))))
}