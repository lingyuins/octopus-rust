use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use octopus_db::ops::api_key;

/// API key info extracted from the request and stored in extensions for
/// downstream relay handlers. Defined at module scope so handlers can
/// retrieve it via `get_api_key_info` / `req.extensions().get::<ApiKeyInfo>()`.
#[derive(Debug, Clone)]
pub struct ApiKeyInfo {
    pub id: i32,
    pub key: String,
    pub name: String,
    pub allowed_models: Option<String>,
    pub max_cost: Option<f64>,
    pub rpm: Option<i32>,
    pub tpm: Option<i32>,
    pub ip_allowlist: Option<String>,
}

/// API Key authentication middleware for relay endpoints
pub async fn api_key_middleware(mut req: Request, next: Next) -> Result<Response, Response> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));

    let key = match auth_header {
        Some(k) => k,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Missing API key",
            ).into_response());
        }
    };

    // Look up the API key
    let api_key_record = match api_key::get_api_key_by_key(key).await {
        Ok(Some(k)) => k,
        Ok(None) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Invalid API key",
            ).into_response());
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to verify API key",
            ).into_response());
        }
    };

    if !api_key_record.enabled {
        return Err((
            StatusCode::FORBIDDEN,
            "API key is disabled",
        ).into_response());
    }

    // Check expiry
    if let Some(ref expiry) = api_key_record.expiry {
        if let Ok(expiry_time) = chrono::DateTime::parse_from_rfc3339(expiry) {
            if chrono::Utc::now() > expiry_time {
                return Err((
                    StatusCode::FORBIDDEN,
                    "API key has expired",
                ).into_response());
            }
        }
    }

    // Store API key info in request extensions
    req.extensions_mut().insert(ApiKeyInfo {
        id: api_key_record.id,
        key: api_key_record.key,
        name: api_key_record.name,
        allowed_models: api_key_record.allowed_models,
        max_cost: api_key_record.max_cost,
        rpm: api_key_record.rpm,
        tpm: api_key_record.tpm,
        ip_allowlist: api_key_record.ip_allowlist,
    });

    Ok(next.run(req).await)
}

/// Extract API key info from request extensions
pub fn get_api_key_info(req: &Request) -> Option<&ApiKeyInfo> {
    req.extensions().get::<ApiKeyInfo>()
}