use octopus_core::model::stats::*;
use chrono::Utc;

use crate::get_db;

/// Increment stats for a successful relay
pub async fn increment_stats(
    channel_id: i32,
    api_key_id: i32,
    model_name: &str,
    prompt_tokens: i64,
    completion_tokens: i64,
    cost: f64,
    latency_ms: f64,
    is_success: bool,
) -> Result<(), octopus_core::error::AppError> {
    let pool = get_db();
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let hour = Utc::now().format("%Y-%m-%d %H:00:00").to_string();
    let total_tokens = prompt_tokens + completion_tokens;

    // Update total stats
    if is_success {
        sqlx::query(
            "INSERT INTO stats_total (id, total_requests, total_tokens, total_cost, success_count, fail_count, updated_at)
             VALUES (1, 1, ?, ?, 1, 0, datetime('now'))
             ON CONFLICT(id) DO UPDATE SET
             total_requests = total_requests + 1,
             total_tokens = total_tokens + ?,
             total_cost = total_cost + ?,
             success_count = success_count + 1,
             updated_at = datetime('now')"
        )
        .bind(total_tokens)
        .bind(cost)
        .bind(total_tokens)
        .bind(cost)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            "INSERT INTO stats_total (id, total_requests, total_tokens, total_cost, success_count, fail_count, updated_at)
             VALUES (1, 1, ?, ?, 0, 1, datetime('now'))
             ON CONFLICT(id) DO UPDATE SET
             total_requests = total_requests + 1,
             total_tokens = total_tokens + ?,
             total_cost = total_cost + ?,
             fail_count = fail_count + 1,
             updated_at = datetime('now')"
        )
        .bind(total_tokens)
        .bind(cost)
        .bind(total_tokens)
        .bind(cost)
        .execute(pool)
        .await?;
    }

    // Update daily stats
    sqlx::query(
        "INSERT INTO stats_daily (date, requests, tokens, cost, success_count, fail_count)
         VALUES (?, 1, ?, ?, ?, ?)
         ON CONFLICT(date) DO UPDATE SET
         requests = requests + 1,
         tokens = tokens + ?,
         cost = cost + ?,
         success_count = success_count + ?,
         fail_count = fail_count + ?"
    )
    .bind(&today)
    .bind(total_tokens)
    .bind(cost)
    .bind(if is_success { 1 } else { 0 })
    .bind(if is_success { 0 } else { 1 })
    .bind(total_tokens)
    .bind(cost)
    .bind(if is_success { 1 } else { 0 })
    .bind(if is_success { 0 } else { 1 })
    .execute(pool)
    .await?;

    // Update hourly stats
    sqlx::query(
        "INSERT INTO stats_hourly (hour, requests, tokens, cost, success_count, fail_count)
         VALUES (?, 1, ?, ?, ?, ?)
         ON CONFLICT(hour) DO UPDATE SET
         requests = requests + 1,
         tokens = tokens + ?,
         cost = cost + ?,
         success_count = success_count + ?,
         fail_count = fail_count + ?"
    )
    .bind(&hour)
    .bind(total_tokens)
    .bind(cost)
    .bind(if is_success { 1 } else { 0 })
    .bind(if is_success { 0 } else { 1 })
    .bind(total_tokens)
    .bind(cost)
    .bind(if is_success { 1 } else { 0 })
    .bind(if is_success { 0 } else { 1 })
    .execute(pool)
    .await?;

    // Update channel stats
    if channel_id > 0 {
        sqlx::query(
            "INSERT INTO stats_channel (channel_id, requests, tokens, cost, success_count, fail_count, avg_latency)
             VALUES (?, 1, ?, ?, ?, ?, ?)
             ON CONFLICT(channel_id) DO UPDATE SET
             requests = requests + 1,
             tokens = tokens + ?,
             cost = cost + ?,
             success_count = success_count + ?,
             fail_count = fail_count + ?,
             avg_latency = (avg_latency * (requests - 1) + ?) / requests"
        )
        .bind(channel_id)
        .bind(total_tokens)
        .bind(cost)
        .bind(if is_success { 1 } else { 0 })
        .bind(if is_success { 0 } else { 1 })
        .bind(latency_ms)
        .bind(total_tokens)
        .bind(cost)
        .bind(if is_success { 1 } else { 0 })
        .bind(if is_success { 0 } else { 1 })
        .bind(latency_ms)
        .execute(pool)
        .await?;
    }

    // Update model stats
    if !model_name.is_empty() {
        sqlx::query(
            "INSERT INTO stats_model (model_name, requests, tokens, cost, success_count, fail_count, avg_latency)
             VALUES (?, 1, ?, ?, ?, ?, ?)
             ON CONFLICT(model_name) DO UPDATE SET
             requests = requests + 1,
             tokens = tokens + ?,
             cost = cost + ?,
             success_count = success_count + ?,
             fail_count = fail_count + ?,
             avg_latency = (avg_latency * (requests - 1) + ?) / requests"
        )
        .bind(model_name)
        .bind(total_tokens)
        .bind(cost)
        .bind(if is_success { 1 } else { 0 })
        .bind(if is_success { 0 } else { 1 })
        .bind(latency_ms)
        .bind(total_tokens)
        .bind(cost)
        .bind(if is_success { 1 } else { 0 })
        .bind(if is_success { 0 } else { 1 })
        .bind(latency_ms)
        .execute(pool)
        .await?;
    }

    // Update API key stats
    if api_key_id > 0 {
        sqlx::query(
            "INSERT INTO stats_api_key (api_key_id, requests, tokens, cost, success_count, fail_count)
             VALUES (?, 1, ?, ?, ?, ?)
             ON CONFLICT(api_key_id) DO UPDATE SET
             requests = requests + 1,
             tokens = tokens + ?,
             cost = cost + ?,
             success_count = success_count + ?,
             fail_count = fail_count + ?"
        )
        .bind(api_key_id)
        .bind(total_tokens)
        .bind(cost)
        .bind(if is_success { 1 } else { 0 })
        .bind(if is_success { 0 } else { 1 })
        .bind(total_tokens)
        .bind(cost)
        .bind(if is_success { 1 } else { 0 })
        .bind(if is_success { 0 } else { 1 })
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Get total stats
pub async fn get_total_stats() -> Result<Option<StatsTotal>, octopus_core::error::AppError> {
    let pool = get_db();
    let row = sqlx::query_as::<_, StatsTotalRow>(
        "SELECT id, total_requests, total_tokens, total_cost, success_count, fail_count, updated_at
         FROM stats_total WHERE id = 1"
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| StatsTotal {
        id: r.id,
        total_requests: r.total_requests,
        total_tokens: r.total_tokens,
        total_cost: r.total_cost,
        success_count: r.success_count,
        fail_count: r.fail_count,
        updated_at: r.updated_at.unwrap_or_default(),
    }))
}

#[derive(sqlx::FromRow)]
struct StatsTotalRow {
    id: i32,
    total_requests: i64,
    total_tokens: i64,
    total_cost: f64,
    success_count: i64,
    fail_count: i64,
    updated_at: Option<String>,
}