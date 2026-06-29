use octopus_core::model::group::{Group, GroupItem};
use sqlx::SqlitePool;

use crate::get_db;

/// List all groups
pub async fn list_groups() -> Result<Vec<Group>, octopus_core::error::AppError> {
    let pool = get_db();
    let rows = sqlx::query_as::<_, GroupRow>(
        "SELECT id, name, endpoint_type, endpoint_provider, outbound_format, mode,
         match_regex, first_token_time_out, session_keep_time, condition,
         last_test_passed, last_test_at FROM groups_table ORDER BY id"
    )
    .fetch_all(pool)
    .await?;

    let mut groups = Vec::new();
    for row in rows {
        let items = list_group_items(pool, row.id).await?;
        groups.push(Group {
            id: row.id,
            name: row.name,
            endpoint_type: row.endpoint_type,
            endpoint_provider: row.endpoint_provider.unwrap_or_default(),
            outbound_format: row.outbound_format.unwrap_or_default(),
            mode: model::GroupMode::from_int(row.mode),
            match_regex: row.match_regex.unwrap_or_default(),
            first_token_time_out: row.first_token_time_out,
            session_keep_time: row.session_keep_time,
            condition: row.condition,
            items,
            last_test_passed: row.last_test_passed.map(|v| v != 0),
            last_test_at: row.last_test_at,
        });
    }

    Ok(groups)
}

/// Get group by ID
pub async fn get_group(id: i32) -> Result<Option<Group>, octopus_core::error::AppError> {
    let pool = get_db();
    let row = sqlx::query_as::<_, GroupRow>(
        "SELECT id, name, endpoint_type, endpoint_provider, outbound_format, mode,
         match_regex, first_token_time_out, session_keep_time, condition,
         last_test_passed, last_test_at FROM groups_table WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => {
            let items = list_group_items(pool, row.id).await?;
            Ok(Some(Group {
                id: row.id,
                name: row.name,
                endpoint_type: row.endpoint_type,
                endpoint_provider: row.endpoint_provider.unwrap_or_default(),
                outbound_format: row.outbound_format.unwrap_or_default(),
            mode: model::GroupMode::from_int(row.mode),
            match_regex: row.match_regex.unwrap_or_default(),
            first_token_time_out: row.first_token_time_out,
            session_keep_time: row.session_keep_time,
            condition: row.condition,
            items,
            last_test_passed: row.last_test_passed.map(|v| v != 0),
            last_test_at: row.last_test_at,
        }))
        }
        None => Ok(None),
    }
}

/// Create a new group
pub async fn create_group(group: &Group) -> Result<Group, octopus_core::error::AppError> {
    let pool = get_db();
    let result = sqlx::query(
        "INSERT INTO groups_table (name, endpoint_type, endpoint_provider, outbound_format, mode,
         match_regex, first_token_time_out, session_keep_time, condition)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&group.name)
    .bind(&group.endpoint_type)
    .bind(&group.endpoint_provider)
    .bind(&group.outbound_format)
    .bind(group.mode.as_int())
    .bind(&group.match_regex)
    .bind(group.first_token_time_out)
    .bind(group.session_keep_time)
    .bind(&group.condition)
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid() as i32;

    // Insert items
    for item in &group.items {
        insert_group_item(pool, id, item).await?;
    }

    let created = get_group(id).await?.unwrap_or_else(|| Group {
        id,
        ..group.clone()
    });

    super::get_group_cache().insert(id, created.clone());
    Ok(created)
}

/// Update a group
pub async fn update_group(
    id: i32,
    updates: &octopus_core::model::group::GroupUpdateRequest,
) -> Result<(), octopus_core::error::AppError> {
    let pool = get_db();

    if let Some(ref name) = updates.name {
        sqlx::query("UPDATE groups_table SET name = ? WHERE id = ?").bind(name).bind(id).execute(pool).await?;
    }
    if let Some(ref endpoint_type) = updates.endpoint_type {
        sqlx::query("UPDATE groups_table SET endpoint_type = ? WHERE id = ?").bind(endpoint_type).bind(id).execute(pool).await?;
    }
    if let Some(ref condition) = updates.condition {
        sqlx::query("UPDATE groups_table SET condition = ? WHERE id = ?").bind(condition).bind(id).execute(pool).await?;
    }
    if let Some(mode) = updates.mode {
        sqlx::query("UPDATE groups_table SET mode = ? WHERE id = ?").bind(mode.as_int()).bind(id).execute(pool).await?;
    }
    if let Some(first_token_time_out) = updates.first_token_time_out {
        sqlx::query("UPDATE groups_table SET first_token_time_out = ? WHERE id = ?").bind(first_token_time_out).bind(id).execute(pool).await?;
    }

    // Handle item changes
    if let Some(ref items_add) = updates.items_to_add {
        for item in items_add {
            let gi = GroupItem {
                id: 0,
                group_id: id,
                channel_id: item.channel_id,
                model_name: item.model_name.clone(),
                priority: item.priority,
                weight: item.weight,
            };
            insert_group_item(pool, id, &gi).await?;
        }
    }
    if let Some(ref items_update) = updates.items_to_update {
        for item in items_update {
            if item.priority > 0 {
                sqlx::query("UPDATE group_items SET priority = ? WHERE id = ? AND group_id = ?")
                    .bind(item.priority).bind(item.id).bind(id).execute(pool).await?;
            }
            if item.weight > 0 {
                sqlx::query("UPDATE group_items SET weight = ? WHERE id = ? AND group_id = ?")
                    .bind(item.weight).bind(item.id).bind(id).execute(pool).await?;
            }
        }
    }
    if let Some(ref items_delete) = updates.items_to_delete {
        for item_id in items_delete {
            sqlx::query("DELETE FROM group_items WHERE id = ? AND group_id = ?")
                .bind(item_id).bind(id).execute(pool).await?;
        }
    }

    // Refresh cache
    if let Some(g) = get_group(id).await? {
        super::get_group_cache().insert(id, g);
    }

    Ok(())
}

/// Delete a group
pub async fn delete_group(id: i32) -> Result<(), octopus_core::error::AppError> {
    let pool = get_db();
    sqlx::query("DELETE FROM group_items WHERE group_id = ?").bind(id).execute(pool).await?;
    sqlx::query("DELETE FROM groups_table WHERE id = ?").bind(id).execute(pool).await?;
    super::get_group_cache().remove(&id);
    Ok(())
}

async fn list_group_items(pool: &SqlitePool, group_id: i32) -> Result<Vec<GroupItem>, octopus_core::error::AppError> {
    let rows = sqlx::query_as::<_, GroupItemRow>(
        "SELECT id, group_id, channel_id, model_name, priority, weight
         FROM group_items WHERE group_id = ? ORDER BY priority, id"
    )
    .bind(group_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| GroupItem {
        id: r.id,
        group_id: r.group_id,
        channel_id: r.channel_id,
        model_name: r.model_name,
        priority: r.priority,
        weight: r.weight,
    }).collect())
}

async fn insert_group_item(pool: &SqlitePool, group_id: i32, item: &GroupItem) -> Result<(), octopus_core::error::AppError> {
    sqlx::query(
        "INSERT INTO group_items (group_id, channel_id, model_name, priority, weight)
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(group_id)
    .bind(item.channel_id)
    .bind(&item.model_name)
    .bind(item.priority)
    .bind(item.weight)
    .execute(pool)
    .await?;
    Ok(())
}

#[derive(sqlx::FromRow)]
struct GroupRow {
    id: i32,
    name: String,
    endpoint_type: String,
    endpoint_provider: Option<String>,
    outbound_format: Option<String>,
    mode: i32,
    match_regex: Option<String>,
    first_token_time_out: i32,
    session_keep_time: i32,
    condition: Option<String>,
    last_test_passed: Option<i32>,
    last_test_at: i64,
}

#[derive(sqlx::FromRow)]
struct GroupItemRow {
    id: i32,
    group_id: i32,
    channel_id: i32,
    model_name: String,
    priority: i32,
    weight: i32,
}

// Re-export GroupMode for convenience. `GroupMode` is defined in
// `octopus_core::types` (the re-export through `model::group` is private), so
// import it from there.
mod model {
    pub use octopus_core::types::GroupMode;
}