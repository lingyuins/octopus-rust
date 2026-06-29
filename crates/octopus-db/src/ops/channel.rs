use octopus_core::model::channel::{Channel, ChannelKey};
use sqlx::SqlitePool;

use crate::get_db;

/// List all channels
pub async fn list_channels() -> Result<Vec<Channel>, octopus_core::error::AppError> {
    let pool = get_db();
    let rows = sqlx::query_as::<_, ChannelRow>(
        "SELECT id, name, group_id, type, enabled, base_urls, model, custom_model, 
         proxy_mode, proxy_config_id, proxy, auto_sync, auto_group, skip_model_test,
         key_selection_strategy, custom_header, param_override, channel_proxy, 
         request_rewrite, match_regex FROM channels ORDER BY id"
    )
    .fetch_all(pool)
    .await?;

    let mut channels = Vec::new();
    for row in rows {
        let keys = list_channel_keys(pool, row.id).await?;
        let ch = Channel {
            id: row.id,
            name: row.name,
            group_id: row.group_id,
            r#type: serde_json::from_str(&row.type_).unwrap_or_default(),
            enabled: row.enabled != 0,
            base_urls: serde_json::from_str(&row.base_urls).unwrap_or_default(),
            keys,
            model: row.model.unwrap_or_default(),
            custom_model: row.custom_model.unwrap_or_default(),
            proxy_mode: serde_json::from_str(&format!("\"{}\"", row.proxy_mode.unwrap_or_default())).unwrap_or_default(),
            proxy_config_id: row.proxy_config_id,
            proxy: row.proxy != 0,
            auto_sync: row.auto_sync != 0,
            auto_group: serde_json::from_str(&format!("{}", row.auto_group)).unwrap_or_default(),
            skip_model_test: row.skip_model_test != 0,
            key_selection_strategy: row.key_selection_strategy.unwrap_or_default(),
            custom_header: serde_json::from_str(&row.custom_header.unwrap_or_default()).unwrap_or_default(),
            param_override: row.param_override,
            channel_proxy: row.channel_proxy,
            request_rewrite: row.request_rewrite.and_then(|s| serde_json::from_str(&s).ok()),
            match_regex: row.match_regex,
            managed: false,
            managed_source: None,
            stats: None,
        };
        channels.push(ch);
    }

    Ok(channels)
}

/// Get channel by ID
pub async fn get_channel(id: i32) -> Result<Option<Channel>, octopus_core::error::AppError> {
    let pool = get_db();
    let row = sqlx::query_as::<_, ChannelRow>(
        "SELECT id, name, group_id, type, enabled, base_urls, model, custom_model, 
         proxy_mode, proxy_config_id, proxy, auto_sync, auto_group, skip_model_test,
         key_selection_strategy, custom_header, param_override, channel_proxy, 
         request_rewrite, match_regex FROM channels WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => {
            let keys = list_channel_keys(pool, row.id).await?;
            Ok(Some(Channel {
                id: row.id,
                name: row.name,
                group_id: row.group_id,
                r#type: serde_json::from_str(&row.type_).unwrap_or_default(),
                enabled: row.enabled != 0,
                base_urls: serde_json::from_str(&row.base_urls).unwrap_or_default(),
                keys,
                model: row.model.unwrap_or_default(),
                custom_model: row.custom_model.unwrap_or_default(),
                proxy_mode: serde_json::from_str(&format!("\"{}\"", row.proxy_mode.unwrap_or_default())).unwrap_or_default(),
                proxy_config_id: row.proxy_config_id,
                proxy: row.proxy != 0,
                auto_sync: row.auto_sync != 0,
                auto_group: serde_json::from_str(&format!("{}", row.auto_group)).unwrap_or_default(),
                skip_model_test: row.skip_model_test != 0,
                key_selection_strategy: row.key_selection_strategy.unwrap_or_default(),
                custom_header: serde_json::from_str(&row.custom_header.unwrap_or_default()).unwrap_or_default(),
                param_override: row.param_override,
                channel_proxy: row.channel_proxy,
                request_rewrite: row.request_rewrite.and_then(|s| serde_json::from_str(&s).ok()),
                match_regex: row.match_regex,
                managed: false,
                managed_source: None,
                stats: None,
            }))
        }
        None => Ok(None),
    }
}

/// Create a new channel
pub async fn create_channel(ch: &Channel) -> Result<Channel, octopus_core::error::AppError> {
    let pool = get_db();
    let base_urls_json = serde_json::to_string(&ch.base_urls)?;
    let custom_header_json = serde_json::to_string(&ch.custom_header)?;
    let request_rewrite_json = ch.request_rewrite.as_ref().map(|r| serde_json::to_string(r)).transpose()?;

    let result = sqlx::query(
        "INSERT INTO channels (name, group_id, type, enabled, base_urls, model, custom_model,
         proxy_mode, proxy_config_id, proxy, auto_sync, auto_group, skip_model_test,
         key_selection_strategy, custom_header, param_override, channel_proxy,
         request_rewrite, match_regex)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&ch.name)
    .bind(ch.group_id)
    .bind(ch.r#type.as_str())
    .bind(ch.enabled as i32)
    .bind(&base_urls_json)
    .bind(&ch.model)
    .bind(&ch.custom_model)
    .bind(serde_json::to_string(&ch.proxy_mode)?.trim_matches('"'))
    .bind(ch.proxy_config_id)
    .bind(ch.proxy as i32)
    .bind(ch.auto_sync as i32)
    .bind(ch.auto_group as i32)
    .bind(ch.skip_model_test as i32)
    .bind(&ch.key_selection_strategy)
    .bind(&custom_header_json)
    .bind(&ch.param_override)
    .bind(&ch.channel_proxy)
    .bind(&request_rewrite_json)
    .bind(&ch.match_regex)
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid() as i32;

    // Insert keys
    for key in &ch.keys {
        insert_channel_key(pool, id, key).await?;
    }

    let created = get_channel(id).await?.unwrap_or_else(|| Channel {
        id,
        ..ch.clone()
    });

    super::get_channel_cache().insert(id, created.clone());
    Ok(created)
}

/// Update a channel
pub async fn update_channel(id: i32, updates: &octopus_core::model::channel::ChannelUpdateRequest) -> Result<(), octopus_core::error::AppError> {
    let pool = get_db();

    if let Some(ref name) = updates.name {
        sqlx::query("UPDATE channels SET name = ? WHERE id = ?").bind(name).bind(id).execute(pool).await?;
    }
    if let Some(group_id) = updates.group_id {
        sqlx::query("UPDATE channels SET group_id = ? WHERE id = ?").bind(group_id).bind(id).execute(pool).await?;
    }
    if let Some(ref type_) = updates.r#type {
        sqlx::query("UPDATE channels SET type = ? WHERE id = ?").bind(type_.as_str()).bind(id).execute(pool).await?;
    }
    if let Some(enabled) = updates.enabled {
        sqlx::query("UPDATE channels SET enabled = ? WHERE id = ?").bind(enabled as i32).bind(id).execute(pool).await?;
    }
    if let Some(ref base_urls) = updates.base_urls {
        let json = serde_json::to_string(base_urls)?;
        sqlx::query("UPDATE channels SET base_urls = ? WHERE id = ?").bind(&json).bind(id).execute(pool).await?;
    }
    if let Some(ref model) = updates.model {
        sqlx::query("UPDATE channels SET model = ? WHERE id = ?").bind(model).bind(id).execute(pool).await?;
    }
    if let Some(ref custom_model) = updates.custom_model {
        sqlx::query("UPDATE channels SET custom_model = ? WHERE id = ?").bind(custom_model).bind(id).execute(pool).await?;
    }
    if let Some(ref proxy_mode) = updates.proxy_mode {
        let s = serde_json::to_string(proxy_mode)?.trim_matches('"').to_string();
        sqlx::query("UPDATE channels SET proxy_mode = ? WHERE id = ?").bind(&s).bind(id).execute(pool).await?;
    }
    if let Some(proxy) = updates.proxy {
        sqlx::query("UPDATE channels SET proxy = ? WHERE id = ?").bind(proxy as i32).bind(id).execute(pool).await?;
    }
    if let Some(auto_sync) = updates.auto_sync {
        sqlx::query("UPDATE channels SET auto_sync = ? WHERE id = ?").bind(auto_sync as i32).bind(id).execute(pool).await?;
    }

    // Handle key changes
    if let Some(ref keys_add) = updates.keys_to_add {
        for key in keys_add {
            let ck = ChannelKey {
                id: 0,
                channel_id: id,
                enabled: key.enabled,
                channel_key: key.channel_key.clone(),
                status_code: 0,
                last_use_time_stamp: 0,
                total_cost: 0.0,
                remark: key.remark.clone(),
            };
            insert_channel_key(pool, id, &ck).await?;
        }
    }
    if let Some(ref keys_update) = updates.keys_to_update {
        for key in keys_update {
            if let Some(enabled) = key.enabled {
                sqlx::query("UPDATE channel_keys SET enabled = ? WHERE id = ? AND channel_id = ?")
                    .bind(enabled as i32).bind(key.id).bind(id).execute(pool).await?;
            }
            if let Some(ref channel_key) = key.channel_key {
                sqlx::query("UPDATE channel_keys SET channel_key = ? WHERE id = ? AND channel_id = ?")
                    .bind(channel_key).bind(key.id).bind(id).execute(pool).await?;
            }
            if let Some(ref remark) = key.remark {
                sqlx::query("UPDATE channel_keys SET remark = ? WHERE id = ? AND channel_id = ?")
                    .bind(remark).bind(key.id).bind(id).execute(pool).await?;
            }
        }
    }
    if let Some(ref keys_delete) = updates.keys_to_delete {
        for key_id in keys_delete {
            sqlx::query("DELETE FROM channel_keys WHERE id = ? AND channel_id = ?")
                .bind(key_id).bind(id).execute(pool).await?;
        }
    }

    // Refresh cache
    if let Some(ch) = get_channel(id).await? {
        super::get_channel_cache().insert(id, ch);
    }

    Ok(())
}

/// Delete a channel
pub async fn delete_channel(id: i32) -> Result<(), octopus_core::error::AppError> {
    let pool = get_db();
    sqlx::query("DELETE FROM channel_keys WHERE channel_id = ?").bind(id).execute(pool).await?;
    sqlx::query("DELETE FROM channels WHERE id = ?").bind(id).execute(pool).await?;
    super::get_channel_cache().remove(&id);
    Ok(())
}

/// Get channel by name
pub async fn get_channel_by_name(name: &str) -> Result<Option<Channel>, octopus_core::error::AppError> {
    let pool = get_db();
    let row = sqlx::query_as::<_, ChannelRow>(
        "SELECT id, name, group_id, type, enabled, base_urls, model, custom_model, 
         proxy_mode, proxy_config_id, proxy, auto_sync, auto_group, skip_model_test,
         key_selection_strategy, custom_header, param_override, channel_proxy, 
         request_rewrite, match_regex FROM channels WHERE name = ?"
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => get_channel(row.id).await,
        None => Ok(None),
    }
}

async fn list_channel_keys(pool: &SqlitePool, channel_id: i32) -> Result<Vec<ChannelKey>, octopus_core::error::AppError> {
    let rows = sqlx::query_as::<_, ChannelKeyRow>(
        "SELECT id, channel_id, enabled, channel_key, status_code, last_use_time_stamp, total_cost, remark
         FROM channel_keys WHERE channel_id = ? ORDER BY id"
    )
    .bind(channel_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| ChannelKey {
        id: r.id,
        channel_id: r.channel_id,
        enabled: r.enabled != 0,
        channel_key: r.channel_key,
        status_code: r.status_code,
        last_use_time_stamp: r.last_use_time_stamp,
        total_cost: r.total_cost,
        remark: r.remark.unwrap_or_default(),
    }).collect())
}

async fn insert_channel_key(pool: &SqlitePool, channel_id: i32, key: &ChannelKey) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "INSERT INTO channel_keys (channel_id, enabled, channel_key, status_code, last_use_time_stamp, total_cost, remark)
         VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(channel_id)
    .bind(key.enabled as i32)
    .bind(&key.channel_key)
    .bind(key.status_code)
    .bind(key.last_use_time_stamp)
    .bind(key.total_cost)
    .bind(&key.remark)
    .execute(pool)
    .await?;
    Ok(())
}

#[derive(sqlx::FromRow)]
struct ChannelRow {
    id: i32,
    name: String,
    group_id: i32,
    #[sqlx(rename = "type")]
    type_: String,
    enabled: i32,
    base_urls: String,
    model: Option<String>,
    custom_model: Option<String>,
    proxy_mode: Option<String>,
    proxy_config_id: Option<i32>,
    proxy: i32,
    auto_sync: i32,
    auto_group: i32,
    skip_model_test: i32,
    key_selection_strategy: Option<String>,
    custom_header: Option<String>,
    param_override: Option<String>,
    channel_proxy: Option<String>,
    request_rewrite: Option<String>,
    match_regex: Option<String>,
}

#[derive(sqlx::FromRow)]
struct ChannelKeyRow {
    id: i32,
    channel_id: i32,
    enabled: i32,
    channel_key: String,
    status_code: i32,
    last_use_time_stamp: i64,
    total_cost: f64,
    remark: Option<String>,
}