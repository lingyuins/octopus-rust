use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::constants::*;

/// Server configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub trusted_proxies: String,
}

fn default_host() -> String {
    DEFAULT_SERVER_HOST.to_string()
}

fn default_port() -> u16 {
    DEFAULT_SERVER_PORT
}

/// Log configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
}

fn default_log_level() -> String {
    DEFAULT_LOG_LEVEL.to_string()
}

/// SQLite specific configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SQLiteConfig {
    #[serde(default = "default_sqlite_cache_size")]
    pub cache_size: i32,
    #[serde(default = "default_sqlite_mmap_size")]
    pub mmap_size: i64,
}

fn default_sqlite_cache_size() -> i32 {
    DEFAULT_SQLITE_CACHE_SIZE
}

fn default_sqlite_mmap_size() -> i64 {
    DEFAULT_SQLITE_MMAP_SIZE
}

/// Database configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatabaseConfig {
    #[serde(default = "default_db_type")]
    pub r#type: String,
    #[serde(default = "default_db_path")]
    pub path: String,
    #[serde(default)]
    pub log_type: String,
    #[serde(default)]
    pub log_path: String,
    #[serde(default)]
    pub sqlite: SQLiteConfig,
}

fn default_db_type() -> String {
    DEFAULT_DB_TYPE.to_string()
}

fn default_db_path() -> String {
    format!("{}/{}", DEFAULT_DATA_DIR, DEFAULT_DB_FILE)
}

/// Auth configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthConfig {
    #[serde(default)]
    pub jwt_secret: String,
}

/// Relay configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RelayConfig {
    #[serde(default = "default_max_json_body")]
    pub max_json_body_bytes: i64,
    #[serde(default = "default_max_multipart_body")]
    pub max_multipart_body_bytes: i64,
}

fn default_max_json_body() -> i64 {
    DEFAULT_MAX_JSON_BODY_BYTES
}

fn default_max_multipart_body() -> i64 {
    DEFAULT_MAX_MULTIPART_BODY_BYTES
}

/// External service configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExternalConfig {
    #[serde(default = "default_llm_price_url")]
    pub llm_price_url: String,
    #[serde(default = "default_update_url")]
    pub update_url: String,
    #[serde(default = "default_update_api_url")]
    pub update_api_url: String,
}

fn default_llm_price_url() -> String {
    DEFAULT_LLM_PRICE_URL.to_string()
}

fn default_update_url() -> String {
    DEFAULT_UPDATE_URL.to_string()
}

fn default_update_api_url() -> String {
    DEFAULT_UPDATE_API_URL.to_string()
}

/// Security configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityConfig {
    #[serde(default)]
    pub encryption_key: String,
}

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub log: LogConfig,
    #[serde(default)]
    pub database: DatabaseConfig,
    #[serde(default)]
    pub auth: AuthConfig,
    #[serde(default)]
    pub relay: RelayConfig,
    #[serde(default)]
    pub external: ExternalConfig,
    #[serde(default)]
    pub security: SecurityConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: default_host(),
                port: default_port(),
                trusted_proxies: String::new(),
            },
            log: LogConfig {
                level: default_log_level(),
            },
            database: DatabaseConfig {
                r#type: default_db_type(),
                path: default_db_path(),
                log_type: String::new(),
                log_path: String::new(),
                sqlite: SQLiteConfig {
                    cache_size: default_sqlite_cache_size(),
                    mmap_size: default_sqlite_mmap_size(),
                },
            },
            auth: AuthConfig {
                jwt_secret: String::new(),
            },
            relay: RelayConfig {
                max_json_body_bytes: default_max_json_body(),
                max_multipart_body_bytes: default_max_multipart_body(),
            },
            external: ExternalConfig {
                llm_price_url: default_llm_price_url(),
                update_url: default_update_url(),
                update_api_url: default_update_api_url(),
            },
            security: SecurityConfig {
                encryption_key: String::new(),
            },
        }
    }
}

impl AppConfig {
    /// Load configuration from file and environment variables
    pub fn load(config_path: Option<&str>) -> Result<Self, crate::error::AppError> {
        use figment::{
            providers::{Env, Format, Json},
            Figment,
        };

        let data_dir = std::env::var("OCTOPUS_DATA_DIR")
            .unwrap_or_else(|_| DEFAULT_DATA_DIR.to_string());

        let default_config_path = PathBuf::from(&data_dir).join(DEFAULT_CONFIG_FILE);

        let config_path = config_path
            .map(PathBuf::from)
            .unwrap_or(default_config_path);

        let figment = Figment::new()
            .merge(Json::file(&config_path))
            .merge(Env::prefixed("OCTOPUS_").split("_"));

        // If config file doesn't exist, create one with freshly generated secrets
        if !config_path.exists() {
            tracing::info!("Config file not found, creating default config at {:?}", config_path);

            if let Some(parent) = config_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    crate::error::AppError::Config(format!(
                        "failed to create config directory: {}",
                        e
                    ))
                })?;
            }

            let mut default_config = AppConfig::default();
            default_config.security.encryption_key = generate_random_hex(32);
            default_config.auth.jwt_secret = generate_random_hex(48);
            let json = serde_json::to_string_pretty(&default_config)?;
            std::fs::write(&config_path, json).map_err(|e| {
                crate::error::AppError::Config(format!(
                    "failed to write default config: {}",
                    e
                ))
            })?;

            tracing::info!("Generated security.encryption_key and auth.jwt_secret in config file");
            return Ok(default_config);
        }

        let config: AppConfig = figment.extract().map_err(|e| {
            crate::error::AppError::Config(format!("failed to parse config: {}", e))
        })?;

        tracing::info!("Using config file: {:?}", config_path);

        Ok(config)
    }

    pub fn is_debug(&self) -> bool {
        self.log.level.to_lowercase() == "debug"
    }

    pub fn data_dir() -> String {
        std::env::var("OCTOPUS_DATA_DIR")
            .unwrap_or_else(|_| DEFAULT_DATA_DIR.to_string())
    }

    pub fn default_config_path() -> PathBuf {
        PathBuf::from(Self::data_dir()).join(DEFAULT_CONFIG_FILE)
    }

    pub fn default_database_path() -> PathBuf {
        PathBuf::from(Self::data_dir()).join(DEFAULT_DB_FILE)
    }
}

/// Generate `n` random bytes and return them as a lowercase hex string.
///
/// Uses `ring::rand` for cryptographic randomness. Used to auto-generate the
/// encryption key and JWT secret on first run so the server can boot without
/// manual configuration (the values are persisted to `config.json` and can be
/// overridden via `OCTOPUS_*` env vars).
fn generate_random_hex(n: usize) -> String {
    use ring::rand::SecureRandom;
    let rng = ring::rand::SystemRandom::new();
    let mut bytes = vec![0u8; n];
    // SystemRandom almost never fails; panic is acceptable here.
    rng.fill(&mut bytes).expect("ring SystemRandom failed");
    hex::encode(bytes)
}
