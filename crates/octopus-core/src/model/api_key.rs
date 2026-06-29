use serde::{Deserialize, Serialize};

/// API key model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: i32,
    pub key: String,
    pub name: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub allowed_models: Option<String>,
    #[serde(default)]
    pub expiry: Option<String>,
    #[serde(default)]
    pub max_cost: Option<f64>,
    #[serde(default)]
    pub rpm: Option<i32>,
    #[serde(default)]
    pub tpm: Option<i32>,
    #[serde(default)]
    pub per_model_quotas: Option<String>,
    #[serde(default)]
    pub ip_allowlist: Option<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub last_used_at: Option<String>,
    #[serde(default)]
    pub total_cost: f64,
    #[serde(default)]
    pub remark: String,
}

fn default_true() -> bool {
    true
}

/// API key create request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyCreateRequest {
    pub name: String,
    #[serde(default)]
    pub allowed_models: Option<String>,
    #[serde(default)]
    pub expiry: Option<String>,
    #[serde(default)]
    pub max_cost: Option<f64>,
    #[serde(default)]
    pub rpm: Option<i32>,
    #[serde(default)]
    pub tpm: Option<i32>,
    #[serde(default)]
    pub per_model_quotas: Option<String>,
    #[serde(default)]
    pub ip_allowlist: Option<String>,
    #[serde(default)]
    pub remark: String,
}

/// API key update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyUpdateRequest {
    pub id: i32,
    pub name: Option<String>,
    pub enabled: Option<bool>,
    pub allowed_models: Option<String>,
    pub expiry: Option<String>,
    pub max_cost: Option<f64>,
    pub rpm: Option<i32>,
    pub tpm: Option<i32>,
    pub per_model_quotas: Option<String>,
    pub ip_allowlist: Option<String>,
    pub remark: Option<String>,
}