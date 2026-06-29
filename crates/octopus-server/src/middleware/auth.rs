use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use octopus_core::AppConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: Option<String>,
    pub user_id: Option<i32>,
    pub role: Option<String>,
    pub exp: usize,
    pub iat: Option<usize>,
    pub iss: Option<String>,
}

/// JWT authentication middleware
pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, Response> {
    let config = AppConfig::load(None).unwrap_or_default();
    let secret = &config.auth.jwt_secret;

    if secret.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "JWT secret not configured",
        )
            .into_response());
    }

    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));

    let token = match auth_header {
        Some(t) => t,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Missing authorization header",
            )
                .into_response());
        }
    };

    let validation = Validation::new(Algorithm::HS256);
    let token_data = match decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    ) {
        Ok(data) => data,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token",
            )
                .into_response());
        }
    };

    // Store user info in request extensions
    let claims = token_data.claims;
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

/// Extract JWT claims from request extensions
pub fn get_claims(req: &Request) -> Option<&JwtClaims> {
    req.extensions().get::<JwtClaims>()
}

/// Generate JWT token
pub fn generate_token(
    user_id: i32,
    role: &str,
    expire_minutes: i32,
    secret: &str,
) -> Result<(String, String), octopus_core::error::AppError> {
    let now = chrono::Utc::now();
    let exp = if expire_minutes == 0 {
        now + chrono::Duration::minutes(15) // default
    } else if expire_minutes == -1 {
        now + chrono::Duration::days(30) // remember me
    } else {
        now + chrono::Duration::minutes(expire_minutes as i64)
    };

    let claims = JwtClaims {
        sub: Some(user_id.to_string()),
        user_id: Some(user_id),
        role: Some(role.to_string()),
        exp: exp.timestamp() as usize,
        iat: Some(now.timestamp() as usize),
        iss: Some("octopus".to_string()),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok((token, exp.to_rfc3339()))
}

/// Generate API key
pub fn generate_api_key() -> String {
    use rand::Rng;
    const CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = rand::thread_rng();
    let key: String = (0..48).map(|_| CHARS[rng.gen_range(0..CHARS.len())] as char).collect();
    format!("sk-octopus-{}", key)
}