use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

/// Create a SQLite connection pool
pub async fn create_pool(database_url: &str) -> Result<SqlitePool, octopus_core::error::AppError> {
    // Ensure the parent directory exists
    if database_url != ":memory:" && !database_url.starts_with("file::memory:") {
        if let Some(parent) = std::path::Path::new(database_url).parent() {
            if !parent.as_os_str().is_empty() && parent.to_str() != Some(".") {
                std::fs::create_dir_all(parent).map_err(|e| {
                    octopus_core::error::AppError::Config(format!(
                        "failed to create database directory: {}",
                        e
                    ))
                })?;
            }
        }
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(1) // SQLite: single connection to serialize writes
        .connect(database_url)
        .await?;

    // Apply PRAGMA settings for SQLite
    let pragmas = [
        "PRAGMA journal_mode=WAL",
        "PRAGMA synchronous=NORMAL",
        "PRAGMA cache_size=-20000",
        "PRAGMA mmap_size=0",
        "PRAGMA busy_timeout=5000",
        "PRAGMA foreign_keys=ON",
        "PRAGMA auto_vacuum=INCREMENTAL",
    ];

    for pragma in &pragmas {
        sqlx::query(pragma).execute(&pool).await?;
    }

    tracing::info!("SQLite database pool created: {}", database_url);

    Ok(pool)
}

/// Create a pool for MySQL
pub async fn create_mysql_pool(database_url: &str) -> Result<SqlitePool, octopus_core::error::AppError> {
    // MySQL support placeholder - would use sqlx::MySqlPool
    // For now, use SQLite everywhere
    create_pool(database_url).await
}

/// Create a pool for PostgreSQL
pub async fn create_postgres_pool(database_url: &str) -> Result<SqlitePool, octopus_core::error::AppError> {
    // PostgreSQL support placeholder - would use sqlx::PgPool
    create_pool(database_url).await
}