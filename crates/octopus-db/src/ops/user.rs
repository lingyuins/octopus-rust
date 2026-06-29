use octopus_core::model::user::User;

use crate::get_db;

/// List all users
pub async fn list_users() -> Result<Vec<User>, octopus_core::error::AppError> {
    let pool = get_db();
    let rows = sqlx::query_as::<_, UserRow>(
        "SELECT id, username, password, role FROM users ORDER BY id"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| User {
        id: r.id,
        username: r.username,
        password: r.password,
        role: r.role,
    }).collect())
}

/// Get user by ID
pub async fn get_user(id: i32) -> Result<Option<User>, octopus_core::error::AppError> {
    let pool = get_db();
    let row = sqlx::query_as::<_, UserRow>(
        "SELECT id, username, password, role FROM users WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| User {
        id: r.id,
        username: r.username,
        password: r.password,
        role: r.role,
    }))
}

/// Get user by username
pub async fn get_user_by_username(username: &str) -> Result<Option<User>, octopus_core::error::AppError> {
    let pool = get_db();
    let row = sqlx::query_as::<_, UserRow>(
        "SELECT id, username, password, role FROM users WHERE username = ?"
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| User {
        id: r.id,
        username: r.username,
        password: r.password,
        role: r.role,
    }))
}

/// Create a new user
pub async fn create_user(user: &User) -> Result<User, octopus_core::error::AppError> {
    let pool = get_db();
    let result = sqlx::query(
        "INSERT INTO users (username, password, role) VALUES (?, ?, ?)"
    )
    .bind(&user.username)
    .bind(&user.password)
    .bind(&user.role)
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid() as i32;

    let new_user = User {
        id,
        username: user.username.clone(),
        password: user.password.clone(),
        role: user.role.clone(),
    };

    // Update cache
    super::get_user_cache().insert(id, new_user.clone());

    Ok(new_user)
}

/// Update a user
pub async fn update_user(id: i32, username: Option<&str>, password: Option<&str>, role: Option<&str>) -> Result<(), octopus_core::error::AppError> {
    let pool = get_db();

    if let Some(username) = username {
        sqlx::query("UPDATE users SET username = ? WHERE id = ?")
            .bind(username)
            .bind(id)
            .execute(pool)
            .await?;
    }
    if let Some(password) = password {
        sqlx::query("UPDATE users SET password = ? WHERE id = ?")
            .bind(password)
            .bind(id)
            .execute(pool)
            .await?;
    }
    if let Some(role) = role {
        sqlx::query("UPDATE users SET role = ? WHERE id = ?")
            .bind(role)
            .bind(id)
            .execute(pool)
            .await?;
    }

    // Refresh cache
    if let Some(user) = get_user(id).await? {
        super::get_user_cache().insert(id, user);
    }

    Ok(())
}

/// Delete a user
pub async fn delete_user(id: i32) -> Result<(), octopus_core::error::AppError> {
    let pool = get_db();
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    super::get_user_cache().remove(&id);
    Ok(())
}

/// Count total users
pub async fn count_users() -> Result<i64, octopus_core::error::AppError> {
    let pool = get_db();
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    Ok(row.0)
}

/// Ensure at least one admin user exists (bootstrap)
pub async fn ensure_admin_exists() -> Result<bool, octopus_core::error::AppError> {
    let count = count_users().await?;
    Ok(count > 0)
}

#[derive(sqlx::FromRow)]
struct UserRow {
    id: i32,
    username: String,
    password: String,
    role: String,
}
