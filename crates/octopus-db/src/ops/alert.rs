use octopus_core::model::alert::*;

use crate::get_db;

/// List alert rules
pub async fn list_alert_rules() -> Result<Vec<AlertRule>, octopus_core::error::AppError> {
    let pool = get_db();
    let rows = sqlx::query_as::<_, AlertRuleRow>(
        "SELECT id, name, enabled, condition_type, threshold, scope, notif_channel_id,
         cooldown_minutes, created_at, updated_at FROM alert_rules ORDER BY id"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| AlertRule {
        id: r.id,
        name: r.name,
        enabled: r.enabled != 0,
        condition_type: serde_json::from_str(&format!("\"{}\"", r.condition_type)).unwrap_or(AlertConditionType::ErrorRate),
        threshold: r.threshold,
        scope: r.scope.unwrap_or_default(),
        notif_channel_id: r.notif_channel_id,
        cooldown_minutes: r.cooldown_minutes,
        created_at: r.created_at.unwrap_or_default(),
        updated_at: r.updated_at.unwrap_or_default(),
    }).collect())
}

/// Get alert rule by ID
pub async fn get_alert_rule(id: i32) -> Result<Option<AlertRule>, octopus_core::error::AppError> {
    let pool = get_db();
    let row = sqlx::query_as::<_, AlertRuleRow>(
        "SELECT id, name, enabled, condition_type, threshold, scope, notif_channel_id,
         cooldown_minutes, created_at, updated_at FROM alert_rules WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| AlertRule {
        id: r.id,
        name: r.name,
        enabled: r.enabled != 0,
        condition_type: serde_json::from_str(&format!("\"{}\"", r.condition_type)).unwrap_or(AlertConditionType::ErrorRate),
        threshold: r.threshold,
        scope: r.scope.unwrap_or_default(),
        notif_channel_id: r.notif_channel_id,
        cooldown_minutes: r.cooldown_minutes,
        created_at: r.created_at.unwrap_or_default(),
        updated_at: r.updated_at.unwrap_or_default(),
    }))
}

/// Create an alert rule
pub async fn create_alert_rule(rule: &AlertRule) -> Result<AlertRule, octopus_core::error::AppError> {
    let pool = get_db();
    let condition_type = serde_json::to_string(&rule.condition_type)?.trim_matches('"').to_string();

    let result = sqlx::query(
        "INSERT INTO alert_rules (name, enabled, condition_type, threshold, scope, notif_channel_id, cooldown_minutes)
         VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&rule.name)
    .bind(rule.enabled as i32)
    .bind(&condition_type)
    .bind(rule.threshold)
    .bind(&rule.scope)
    .bind(rule.notif_channel_id)
    .bind(rule.cooldown_minutes)
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid() as i32;
    get_alert_rule(id).await?.ok_or_else(|| octopus_core::error::AppError::Internal("failed to create alert rule".to_string()))
}

/// Update an alert rule
pub async fn update_alert_rule(id: i32, updates: &AlertRuleRequest) -> Result<(), octopus_core::error::AppError> {
    let pool = get_db();

    if let Some(ref name) = updates.name {
        sqlx::query("UPDATE alert_rules SET name = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(name).bind(id).execute(pool).await?;
    }
    if let Some(enabled) = updates.enabled {
        sqlx::query("UPDATE alert_rules SET enabled = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(enabled as i32).bind(id).execute(pool).await?;
    }
    if let Some(ref condition_type) = updates.condition_type {
        let ct = serde_json::to_string(condition_type)?.trim_matches('"').to_string();
        sqlx::query("UPDATE alert_rules SET condition_type = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(&ct).bind(id).execute(pool).await?;
    }
    if let Some(threshold) = updates.threshold {
        sqlx::query("UPDATE alert_rules SET threshold = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(threshold).bind(id).execute(pool).await?;
    }

    Ok(())
}

/// Delete an alert rule
pub async fn delete_alert_rule(id: i32) -> Result<(), octopus_core::error::AppError> {
    let pool = get_db();
    sqlx::query("DELETE FROM alert_rules WHERE id = ?").bind(id).execute(pool).await?;
    Ok(())
}

/// List alert notification channels
pub async fn list_alert_notif_channels() -> Result<Vec<AlertNotifChannel>, octopus_core::error::AppError> {
    let pool = get_db();
    let rows = sqlx::query_as::<_, AlertNotifChannelRow>(
        "SELECT id, name, enabled, channel_type, config, created_at, updated_at
         FROM alert_notif_channels ORDER BY id"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| AlertNotifChannel {
        id: r.id,
        name: r.name,
        enabled: r.enabled != 0,
        channel_type: serde_json::from_str(&format!("\"{}\"", r.channel_type)).unwrap_or(NotifChannelType::Webhook),
        config: r.config.unwrap_or_default(),
        created_at: r.created_at.unwrap_or_default(),
        updated_at: r.updated_at.unwrap_or_default(),
    }).collect())
}

#[derive(sqlx::FromRow)]
struct AlertRuleRow {
    id: i32,
    name: String,
    enabled: i32,
    condition_type: String,
    threshold: f64,
    scope: Option<String>,
    notif_channel_id: i32,
    cooldown_minutes: i32,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(sqlx::FromRow)]
struct AlertNotifChannelRow {
    id: i32,
    name: String,
    enabled: i32,
    channel_type: String,
    config: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

// Re-export types
use octopus_core::types::{AlertConditionType, NotifChannelType};