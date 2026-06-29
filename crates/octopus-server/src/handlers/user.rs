use axum::{extract::{Path, Query}, http::StatusCode, Json};
use serde::Deserialize;
use octopus_db::ops::user as user_ops;
use octopus_core::model::user::{User, UserCreateRequest, UserInfo};
use crate::response::{ApiResponse, ok_response, created_response, bad_request, not_found, PaginatedResponse};

#[derive(Deserialize)]
pub struct UserQuery {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

/// List users
pub async fn list(
    Query(query): Query<UserQuery>,
) -> Result<(StatusCode, Json<ApiResponse<PaginatedResponse<UserInfo>>>), (StatusCode, Json<ApiResponse<()>>)> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    let users = user_ops::list_users().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    let total = users.len() as i64;
    let user_infos: Vec<UserInfo> = users.into_iter().map(UserInfo::from).collect();
    Ok(ok_response(PaginatedResponse::new(user_infos, total, page, page_size)))
}

/// Create user
pub async fn create(
    Json(req): Json<UserCreateRequest>,
) -> Result<(StatusCode, Json<ApiResponse<UserInfo>>), (StatusCode, Json<ApiResponse<()>>)> {
    if req.username.is_empty() || req.password.is_empty() {
        return Err(bad_request("Username and password are required"));
    }

    let mut user = User {
        id: 0,
        username: req.username,
        password: req.password,
        role: req.role,
    };
    user.hash_password().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;

    let created = user_ops::create_user(&user).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    Ok(created_response(UserInfo::from(created)))
}

/// Update user
pub async fn update(
    Path(id): Path<i32>,
    Json(req): Json<UserCreateRequest>,
) -> Result<(StatusCode, Json<ApiResponse<UserInfo>>), (StatusCode, Json<ApiResponse<()>>)> {
    user_ops::update_user(id, Some(&req.username), Some(&req.password), Some(&req.role))
        .await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;

    let user = user_ops::get_user(id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?
        .ok_or_else(|| not_found("User not found"))?;
    Ok(ok_response(UserInfo::from(user)))
}

/// Delete user
pub async fn delete(
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, Json<ApiResponse<()>>)> {
    user_ops::delete_user(id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&e.to_string()))))?;
    Ok(StatusCode::NO_CONTENT)
}