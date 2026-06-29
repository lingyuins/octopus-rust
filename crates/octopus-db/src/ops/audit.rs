use octopus_core::model::audit::AuditLog;

use crate::get_db;

/// Create an audit log entry
pub async fn create_audit_log(
    user_id: i32,
    username: &str,
    action: &str,
    resource: &str,
    resource_id: Option<&str>,
    details: Option<&str>,
    ip_address: &str,
    user_agent: &str,
) -> Result<(), octopus_core::error::AppError> {
    let pool = get_db();
    sqlx::query(
        "INSERT INTO audit_logs (user_id, username, action, resource, resource_id, details, ip_address, user_agent)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(user_id)
    .bind(username)
    .bind(action)
    .bind(resource)
    .bind(resource_id)
    .bind(details)
    .bind(ip_address)
    .bind(user_agent)
    .execute(pool)
    .await?;
    Ok(())
}

/// List audit logs with pagination
pub async fn list_audit_logs(
    page: i32,
    page_size: i32,
    user_id: Option<i32>,
    action: Option<&str>,
    resource: Option<&str>,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<(Vec<AuditLog>, i64), octopus_core::error::AppError> {
    let pool = get_db();

    // Build count query
    let mut count_sql = String::from("SELECT COUNT(*) as count FROM audit_logs WHERE 1=1");
    let mut query_sql = String::from(
        "SELECT id, user_id, username, action, resource, resource_id, details, ip_address, user_agent, created_at
         FROM audit_logs WHERE 1=1"
    );

    if user_id.is_some() {
        count_sql.push_str(" AND user_id = ?");
        query_sql.push_str(" AND user_id = ?");
    }
    if action.is_some() {
        count_sql.push_str(" AND action = ?");
        query_sql.push_str(" AND action = ?");
    }
    if resource.is_some() {
        count_sql.push_str(" AND resource = ?");
        query_sql.push_str(" AND resource = ?");
    }
    if start_date.is_some() {
        count_sql.push_str(" AND created_at >= ?");
        query_sql.push_str(" AND created_at >= ?");
    }
    if end_date.is_some() {
        count_sql.push_str(" AND created_at <= ?");
        query_sql.push_str(" AND created_at <= ?");
    }

    query_sql.push_str(" ORDER BY id DESC LIMIT ? OFFSET ?");

    let offset = (page - 1) * page_size;

    // Count
    let count_row = sqlx::query_scalar(&count_sql)
        .fetch_one(pool)
        .await?;
    let total: i64 = count_row;

    // Query
    let rows = sqlx::query_as::<_, AuditLogRow>(&query_sql)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    let logs = rows.into_iter().map(|r| AuditLog {
        id: r.id,
        user_id: r.user_id,
        username: r.username.unwrap_or_default(),
        action: r.action,
        resource: r.resource.unwrap_or_default(),
        resource_id: r.resource_id,
        details: r.details,
        ip_address: r.ip_address.unwrap_or_default(),
        user_agent: r.user_agent.unwrap_or_default(),
        created_at: r.created_at.unwrap_or_default(),
    }).collect();

    Ok((logs, total))
}

#[derive(sqlx::FromRow)]
struct AuditLogRow {
    id: i32,
    user_id: i32,
    username: Option<String>,
    action: String,
    resource: Option<String>,
    resource_id: Option<String>,
    details: Option<String>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    created_at: Option<String>,
}