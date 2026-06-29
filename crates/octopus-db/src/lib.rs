pub mod pool;
pub mod migrate;
pub mod ops;
pub mod cache;

use once_cell::sync::OnceCell;
use sqlx::SqlitePool;

static DB_POOL: OnceCell<SqlitePool> = OnceCell::new();
static LOG_DB_POOL: OnceCell<SqlitePool> = OnceCell::new();

/// Initialize the main database pool
pub async fn init_db(database_url: &str) -> Result<(), octopus_core::error::AppError> {
    let pool = pool::create_pool(database_url).await?;
    DB_POOL.set(pool).map_err(|_| {
        octopus_core::error::AppError::Internal("database pool already initialized".to_string())
    })?;

    // Run migrations
    migrate::run_migrations(get_db()).await?;

    // Initialize caches
    ops::init_caches().await?;

    Ok(())
}

/// Initialize the log database pool (separate from main DB)
pub async fn init_log_db(database_url: &str) -> Result<(), octopus_core::error::AppError> {
    let pool = pool::create_pool(database_url).await?;
    LOG_DB_POOL.set(pool).map_err(|_| {
        octopus_core::error::AppError::Internal("log database pool already initialized".to_string())
    })?;

    // Run log-specific migrations
    migrate::run_log_migrations(get_log_db()).await?;

    Ok(())
}

/// Get the main database pool
pub fn get_db() -> &'static SqlitePool {
    DB_POOL.get().expect("database pool not initialized")
}

/// Get the log database pool (falls back to main DB if not separately initialized)
pub fn get_log_db() -> &'static SqlitePool {
    LOG_DB_POOL.get().unwrap_or_else(|| get_db())
}

/// Check if the main database is SQLite
pub fn is_sqlite() -> bool {
    // For now, we only support SQLite. Will be extended for MySQL/Postgres.
    true
}

/// Check if log database is separate
pub fn is_log_db_separate() -> bool {
    LOG_DB_POOL.get().is_some()
}

/// Close all database connections
pub async fn close() -> Result<(), octopus_core::error::AppError> {
    if let Some(pool) = LOG_DB_POOL.get() {
        pool.close().await;
    }
    if let Some(pool) = DB_POOL.get() {
        pool.close().await;
    }
    Ok(())
}