use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use std::path::Path;

/// Create a SQLite connection pool.
///
/// `database_url` may be a bare file path (e.g. `data/data.db` or an
/// absolute Windows path like `F:\app\data.db`), a `sqlite:`/`file:` URL,
/// or `:memory:`. Bare and absolute file paths are handled via
/// `SqliteConnectOptions::from_filename` so that Windows drive letters
/// (`F:`) are not misread as a URI scheme by sqlx's URL parser.
pub async fn create_pool(database_url: &str) -> Result<SqlitePool, octopus_core::error::AppError> {
    let options = parse_connect_options(database_url);

    // Ensure the parent directory exists (only for file-based databases)
    if database_url != ":memory:" && !database_url.starts_with("file::memory:") {
        let file_path = extract_file_path(database_url);
        if let Some(path) = file_path {
            if let Some(parent) = path.parent() {
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
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(1) // SQLite: single connection to serialize writes
        .connect_with(options)
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

/// Build `SqliteConnectOptions` from a database URL/path.
///
/// - `:memory:` → in-memory database.
/// - `sqlite:`/`file:` URLs → parsed by sqlx directly.
/// - Bare file paths (relative or absolute) → `from_filename`, which avoids
///   the URL parser misreading Windows drive letters as a scheme.
fn parse_connect_options(database_url: &str) -> SqliteConnectOptions {
    if database_url == ":memory:" {
        return SqliteConnectOptions::new()
            .in_memory(true)
            .create_if_missing(true);
    }

    if database_url.starts_with("sqlite:") || database_url.starts_with("file:") {
        // sqlx can parse these URL-style connection strings directly.
        return database_url
            .parse::<SqliteConnectOptions>()
            .unwrap_or_else(|_| SqliteConnectOptions::new())
            .create_if_missing(true);
    }

    // Bare file path — use filename to sidestep URL parsing entirely.
    SqliteConnectOptions::new()
        .filename(database_url)
        .create_if_missing(true)
}

/// Extract the filesystem path from a database URL (for directory creation).
///
/// For `sqlite:`/`file:` URLs the path portion after the scheme is returned.
/// For bare paths and `:memory:` the input is returned as-is (or `None` for
/// memory databases).
fn extract_file_path(database_url: &str) -> Option<&Path> {
    if database_url == ":memory:" || database_url.starts_with("file::memory:") {
        return None;
    }

    let path_str = if let Some(rest) = database_url.strip_prefix("sqlite://") {
        rest
    } else if let Some(rest) = database_url.strip_prefix("sqlite:") {
        rest
    } else if let Some(rest) = database_url.strip_prefix("file:") {
        // file: URI may be percent-encoded; for our use the path is plain.
        rest.strip_prefix("//").unwrap_or(rest)
    } else {
        database_url
    };

    Some(Path::new(path_str))
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
