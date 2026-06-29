use octopus_core::model::setting::Setting;

use crate::get_db;

/// Get a setting by key
pub async fn get_setting(key: &str) -> Result<Option<Setting>, octopus_core::error::AppError> {
    let pool = get_db();
    let row = sqlx::query_as::<_, SettingRow>(
        "SELECT key, value, description, updated_at FROM settings WHERE key = ?"
    )
    .bind(key)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Setting {
        key: r.key,
        value: r.value,
        description: r.description.unwrap_or_default(),
        updated_at: r.updated_at.unwrap_or_default(),
    }))
}

/// Get a setting value as string
pub async fn get_setting_string(key: &str) -> Result<Option<String>, octopus_core::error::AppError> {
    let setting = get_setting(key).await?;
    Ok(setting.map(|s| s.value))
}

/// Get a setting value as integer
pub async fn get_setting_int(key: &str) -> Result<Option<i32>, octopus_core::error::AppError> {
    let value = get_setting_string(key).await?;
    Ok(value.and_then(|v| v.parse::<i32>().ok()))
}

/// Get a setting value as float
pub async fn get_setting_float(key: &str) -> Result<Option<f64>, octopus_core::error::AppError> {
    let value = get_setting_string(key).await?;
    Ok(value.and_then(|v| v.parse::<f64>().ok()))
}

/// Get a setting value as boolean
pub async fn get_setting_bool(key: &str) -> Result<Option<bool>, octopus_core::error::AppError> {
    let value = get_setting_string(key).await?;
    Ok(value.map(|v| v.to_lowercase() == "true" || v == "1"))
}

/// Upsert a setting
pub async fn upsert_setting(key: &str, value: &str) -> Result<(), octopus_core::error::AppError> {
    let pool = get_db();
    sqlx::query(
        "INSERT INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = datetime('now')"
    )
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;
    Ok(())
}

/// Add a setting (returns error if already exists)
pub async fn add_setting(key: &str, value: &str) -> Result<(), octopus_core::error::AppError> {
    let pool = get_db();
    sqlx::query(
        "INSERT INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))"
    )
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete a setting
pub async fn delete_setting(key: &str) -> Result<(), octopus_core::error::AppError> {
    let pool = get_db();
    sqlx::query("DELETE FROM settings WHERE key = ?")
        .bind(key)
        .execute(pool)
        .await?;
    Ok(())
}

/// List all settings
pub async fn list_settings() -> Result<Vec<Setting>, octopus_core::error::AppError> {
    let pool = get_db();
    let rows = sqlx::query_as::<_, SettingRow>(
        "SELECT key, value, description, updated_at FROM settings ORDER BY key"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| Setting {
        key: r.key,
        value: r.value,
        description: r.description.unwrap_or_default(),
        updated_at: r.updated_at.unwrap_or_default(),
    }).collect())
}

#[derive(sqlx::FromRow)]
struct SettingRow {
    key: String,
    value: String,
    description: Option<String>,
    updated_at: Option<String>,
}