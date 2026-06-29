use axum::{http::StatusCode, Json};
use crate::response::{ApiResponse, PaginatedResponse};
pub async fn paginated<T: serde::Serialize>(items: Vec<T>, total: i64, page: i32, page_size: i32) -> (StatusCode, Json<ApiResponse<PaginatedResponse<T>>>) {
    (StatusCode::OK, Json(ApiResponse::success(PaginatedResponse::new(items, total, page, page_size))))
}