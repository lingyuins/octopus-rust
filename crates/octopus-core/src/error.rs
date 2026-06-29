use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Unified error type for the entire application
#[derive(Error, Debug)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("forbidden: {0}")]
    Forbidden(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("internal error: {0}")]
    Internal(String),

    #[error("service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("too many requests: {0}")]
    TooManyRequests(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("config error: {0}")]
    Config(String),

    #[error("encryption error: {0}")]
    Encryption(String),

    #[error("upstream error: {0}")]
    Upstream(String),

    #[error("{0}")]
    Other(String),
}

/// API response error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    Unknown,
    InternalServerError,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    TooManyRequests,
    ServiceUnavailable,
    ValidationFailed,
    InvalidCredentials,
    TokenExpired,
    ChannelDisabled,
    KeyDisabled,
    KeyExhausted,
    GroupDisabled,
    ModelNotFound,
    RateLimitExceeded,
    QuotaExceeded,
    CircuitBreakerOpen,
    UpstreamTimeout,
    UpstreamError,
    InvalidAPIKey,
    JWTSecretMissing,
    JWTSecretEphemeral,
    EncryptionKeyMissing,
    SettingNotFound,
}

impl AppError {
    pub fn status_code(&self) -> u16 {
        match self {
            AppError::NotFound(_) => 404,
            AppError::BadRequest(_) => 400,
            AppError::Unauthorized(_) => 401,
            AppError::Forbidden(_) => 403,
            AppError::Conflict(_) => 409,
            AppError::Internal(_) => 500,
            AppError::ServiceUnavailable(_) => 503,
            AppError::TooManyRequests(_) => 429,
            AppError::Validation(_) => 422,
            AppError::Database(_) => 500,
            AppError::Http(_) => 502,
            AppError::Json(_) => 400,
            AppError::Io(_) => 500,
            AppError::Jwt(_) => 401,
            AppError::Config(_) => 500,
            AppError::Encryption(_) => 500,
            AppError::Upstream(_) => 502,
            AppError::Other(_) => 500,
        }
    }

    pub fn error_code(&self) -> ErrorCode {
        match self {
            AppError::NotFound(_) => ErrorCode::NotFound,
            AppError::BadRequest(_) => ErrorCode::BadRequest,
            AppError::Unauthorized(_) => ErrorCode::Unauthorized,
            AppError::Forbidden(_) => ErrorCode::Forbidden,
            AppError::Conflict(_) => ErrorCode::Conflict,
            AppError::Internal(_) => ErrorCode::InternalServerError,
            AppError::ServiceUnavailable(_) => ErrorCode::ServiceUnavailable,
            AppError::TooManyRequests(_) => ErrorCode::TooManyRequests,
            AppError::Validation(_) => ErrorCode::ValidationFailed,
            AppError::Database(_) => ErrorCode::InternalServerError,
            AppError::Http(_) => ErrorCode::UpstreamError,
            AppError::Json(_) => ErrorCode::BadRequest,
            AppError::Io(_) => ErrorCode::InternalServerError,
            AppError::Jwt(_) => ErrorCode::InvalidCredentials,
            AppError::Config(_) => ErrorCode::InternalServerError,
            AppError::Encryption(_) => ErrorCode::InternalServerError,
            AppError::Upstream(_) => ErrorCode::UpstreamError,
            AppError::Other(_) => ErrorCode::Unknown,
        }
    }
}