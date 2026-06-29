/// Application name
pub const APP_NAME: &str = "octopus";

/// Application description
pub const APP_DESC: &str = "A Simple, Beautiful, and Elegant LLM API Aggregation & Load Balancing Service";

/// Default data directory
pub const DEFAULT_DATA_DIR: &str = "data";

/// Default config file name
pub const DEFAULT_CONFIG_FILE: &str = "config.json";

/// Default database file name
pub const DEFAULT_DB_FILE: &str = "data.db";

/// Default server host
pub const DEFAULT_SERVER_HOST: &str = "0.0.0.0";

/// Default server port
pub const DEFAULT_SERVER_PORT: u16 = 8080;

/// Default database type
pub const DEFAULT_DB_TYPE: &str = "sqlite";

/// Default log level
pub const DEFAULT_LOG_LEVEL: &str = "info";

/// Default SQLite cache size in KB (negative = KB, ~20MB)
pub const DEFAULT_SQLITE_CACHE_SIZE: i32 = -20000;

/// Default SQLite mmap size (0 = disabled)
pub const DEFAULT_SQLITE_MMAP_SIZE: i64 = 0;

/// Default max JSON body bytes (64MB)
pub const DEFAULT_MAX_JSON_BODY_BYTES: i64 = 64 * 1024 * 1024;

/// Default max multipart body bytes (64MB)
pub const DEFAULT_MAX_MULTIPART_BODY_BYTES: i64 = 64 * 1024 * 1024;

/// Default LLM price URL
pub const DEFAULT_LLM_PRICE_URL: &str = "https://models.dev/api.json";

/// Default update URL
pub const DEFAULT_UPDATE_URL: &str = "https://github.com/lingyuins/octopus/releases/latest/download";

/// Default update API URL
pub const DEFAULT_UPDATE_API_URL: &str = "https://api.github.com/repos/lingyuins/octopus/releases/latest";

/// Known placeholder JWT secrets
pub const KNOWN_PLACEHOLDER_SECRETS: &[&str] = &["change-this-to-a-long-random-secret"];