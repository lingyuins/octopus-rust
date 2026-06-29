use serde::{Deserialize, Serialize};

/// WebAuthn credential model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAuthnCredential {
    pub id: i32,
    pub user_id: i32,
    pub credential_id: String,
    pub public_key: String,
    pub attestation_type: String,
    pub sign_count: i32,
    pub created_at: String,
}

/// WebAuthn registration request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAuthnRegistrationRequest {
    pub username: String,
    pub display_name: String,
}

/// WebAuthn authentication request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAuthnAuthRequest {
    pub username: String,
}

/// WebAuthn credential response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAuthnCredentialResponse {
    pub id: String,
    pub raw_id: String,
    pub response: WebAuthnResponse,
    pub r#type: String,
}

/// WebAuthn response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAuthnResponse {
    pub client_data_json: String,
    pub attestation_object: Option<String>,
    pub authenticator_data: Option<String>,
    pub signature: Option<String>,
    pub user_handle: Option<String>,
}