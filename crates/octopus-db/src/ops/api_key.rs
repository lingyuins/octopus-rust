use octopus_core::model::api_key::ApiKey;

use crate::get_db;

/// List all API keys
pub async fn list_api_keys() -> Result<Vec<ApiKey>, octopus_core::error::AppError> {
    let pool = get_db();
    let rows = sqlx::query_as::<_, ApiKeyRow>(
        "SELECT id, key, name, enabled, allowed_models, expiry, max_cost, rpm, tpm,
         per_model_quotas, ip_allowlist, created_at, last_used_at, total_cost, remark
         FROM api_keys ORDER BY id"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| ApiKey {
        id: r.id,
        key: r.key,
        name: r.name,
        enabled: r.enabled != 0,
        allowed_models: r.allowed_models,
        expiry: r.expiry,
        max_cost: r.max_cost,
        rpm: r.rpm,
        tpm: r.tpm,
        per_model_quotas: r.per_model_quotas,
        ip_allowlist: r.ip_allowlist,
        created_at: r.created_at.unwrap_or_default(),
        last_used_at: r.last_used_at,
        total_cost: r.total_cost,
        remark: r.remark.unwrap_or_default(),
    }).collect())
}

/// Get API key by ID
pub async fn get_api_key(id: i32) -> Result<Option<ApiKey>, octopus_core::error::AppError> {
    let pool = get_db();
    let row = sqlx::query_as::<_, ApiKeyRow>(
        "SELECT id, key, name, enabled, allowed_models, expiry, max_cost, rpm, tpm,
         per_model_quotas, ip_allowlist, created_at, last_used_at, total_cost, remark
         FROM api_keys WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| ApiKey {
        id: r.id,
        key: r.key,
        name: r.name,
        enabled: r.enabled != 0,
        allowed_models: r.allowed_models,
        expiry: r.expiry,
        max_cost: r.max_cost,
        rpm: r.rpm,
        tpm: r.tpm,
        per_model_quotas: r.per_model_quotas,
        ip_allowlist: r.ip_allowlist,
        created_at: r.created_at.unwrap_or_default(),
        last_used_at: r.last_used_at,
        total_cost: r.total_cost,
        remark: r.remark.unwrap_or_default(),
    }))
}

/// Get API key by key string
pub async fn get_api_key_by_key(key: &str) -> Result<Option<ApiKey>, octopus_core::error::AppError> {
    let pool = get_db();
    let row = sqlx::query_as::<_, ApiKeyRow>(
        "SELECT id, key, name, enabled, allowed_models, expiry, max_cost, rpm, tpm,
         per_model_quotas, ip_allowlist, created_at, last_used_at, total_cost, remark
         FROM api_keys WHERE key = ?"
    )
    .bind(key)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| ApiKey {
        id: r.id,
        key: r.key,
        name: r.name,
        enabled: r.enabled != 0,
        allowed_models: r.allowed_models,
        expiry: r.expiry,
        max_cost: r.max_cost,
        rpm: r.rpm,
        tpm: r.tpm,
        per_model_quotas: r.per_model_quotas,
        ip_allowlist: r.ip_allowlist,
        created_at: r.created_at.unwrap_or_default(),
        last_used_at: r.last_used_at,
        total_cost: r.total_cost,
        remark: r.remark.unwrap_or_default(),
    }))
}

/// Create a new API key
pub async fn create_api_key(key: &ApiKey) -> Result<ApiKey, octopus_core::error::AppError> {
    let pool = get_db();
    let result = sqlx::query(
        "INSERT INTO api_keys (key, name, enabled, allowed_models, expiry, max_cost, rpm, tpm,
         per_model_quotas, ip_allowlist, remark)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&key.key)
    .bind(&key.name)
    .bind(key.enabled as i32)
    .bind(&key.allowed_models)
    .bind(&key.expiry)
    .bind(key.max_cost)
    .bind(key.rpm)
    .bind(key.tpm)
    .bind(&key.per_model_quotas)
    .bind(&key.ip_allowlist)
    .bind(&key.remark)
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid() as i32;
    let created = get_api_key(id).await?.unwrap_or_else(|| ApiKey {
        id,
        ..key.clone()
    });

    super::get_api_key_cache().insert(id, created.clone());
    Ok(created)
}

/// Update an API key
pub async fn update_api_key(id: i32, updates: &octopus_core::model::api_key::ApiKeyUpdateRequest) -> Result<(), octopus_core::error::AppError> {
    let pool = get_db();

    if let Some(ref name) = updates.name {
        sqlx::query("UPDATE api_keys SET name = ? WHERE id = ?").bind(name).bind(id).execute(pool).await?;
    }
    if let Some(enabled) = updates.enabled {
        sqlx::query("UPDATE api_keys SET enabled = ? WHERE id = ?").bind(enabled as i32).bind(id).execute(pool).await?;
    }
    if let Some(ref allowed_models) = updates.allowed_models {
        sqlx::query("UPDATE api_keys SET allowed_models = ? WHERE id = ?").bind(allowed_models).bind(id).execute(pool).await?;
    }
    if let Some(ref expiry) = updates.expiry {
        sqlx::query("UPDATE api_keys SET expiry = ? WHERE id = ?").bind(expiry).bind(id).execute(pool).await?;
    }
    if let Some(max_cost) = updates.max_cost {
        sqlx::query("UPDATE api_keys SET max_cost = ? WHERE id = ?").bind(max_cost).bind(id).execute(pool).await?;
    }
    if let Some(rpm) = updates.rpm {
        sqlx::query("UPDATE api_keys SET rpm = ? WHERE id = ?").bind(rpm).bind(id).execute(pool).await?;
    }
    if let Some(tpm) = updates.tpm {
        sqlx::query("UPDATE api_keys SET tpm = ? WHERE id = ?").bind(tpm).bind(id).execute(pool).await?;
    }
    if let Some(ref ip_allowlist) = updates.ip_allowlist {
        sqlx::query("UPDATE api_keys SET ip_allowlist = ? WHERE id = ?").bind(ip_allowlist).bind(id).execute(pool).await?;
    }
    if let Some(ref remark) = updates.remark {
        sqlx::query("UPDATE api_keys SET remark = ? WHERE id = ?").bind(remark).bind(id).execute(pool).await?;
    }

    // Refresh cache
    if let Some(k) = get_api_key(id).await? {
        super::get_api_key_cache().insert(id, k);
    }

    Ok(())
}

/// Delete an API key
pub async fn delete_api_key(id: i32) -> Result<(), octopus_core::error::AppError> {
    let pool = get_db();
    sqlx::query("DELETE FROM api_keys WHERE id = ?").bind(id).execute(pool).await?;
    super::get_api_key_cache().remove(&id);
    Ok(())
}

#[derive(sqlx::FromRow)]
struct ApiKeyRow {
    id: i32,
    key: String,
    name: String,
    enabled: i32,
    allowed_models: Option<String>,
    expiry: Option<String>,
    max_cost: Option<f64>,
    rpm: Option<i32>,
    tpm: Option<i32>,
    per_model_quotas: Option<String>,
    ip_allowlist: Option<String>,
    created_at: Option<String>,
    last_used_at: Option<String>,
    total_cost: f64,
    remark: Option<String>,
}