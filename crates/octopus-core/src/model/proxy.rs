use serde::{Deserialize, Serialize};

/// Proxy configuration model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfiguration {
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub proxy_type: String,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: i32,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub created_at: String,
}

/// Remote site model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteSite {
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub created_at: String,
}

/// Balance snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSnapshot {
    pub id: i32,
    pub site_id: i32,
    pub account_id: i32,
    pub balance: f64,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub captured_at: String,
}

/// Check-in record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInRecord {
    pub id: i32,
    pub site_id: i32,
    pub account_id: i32,
    pub result: String,
    #[serde(default)]
    pub points: f64,
    #[serde(default)]
    pub created_at: String,
}

/// API credential profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCredentialProfile {
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub base_url: String,
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub created_at: String,
}

/// Model mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMapping {
    pub id: i32,
    pub pattern: String,
    pub replacement: String,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub match_type: String,
    #[serde(default)]
    pub group_id: Option<i32>,
    #[serde(default)]
    pub enabled: bool,
}

/// Redemption record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedemptionRecord {
    pub id: i32,
    pub site_id: i32,
    pub account_id: i32,
    pub code: String,
    pub result: String,
    #[serde(default)]
    pub points: f64,
    #[serde(default)]
    pub created_at: String,
}

/// Remote usage record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteUsageRecord {
    pub id: i32,
    pub site_id: i32,
    pub account_id: i32,
    pub model_name: String,
    pub tokens: i64,
    #[serde(default)]
    pub cost: f64,
    #[serde(default)]
    pub recorded_at: String,
}

/// Site announcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteAnnouncement {
    pub id: i32,
    pub site_id: i32,
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub published_at: String,
    #[serde(default)]
    pub created_at: String,
}

/// Remote site token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteSiteToken {
    pub id: i32,
    pub site_id: i32,
    pub token: String,
    #[serde(default)]
    pub created_at: String,
}

/// WS response affinity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSResponseAffinity {
    pub id: i32,
    pub request_id: String,
    pub channel_id: i32,
    pub key_id: i32,
    #[serde(default)]
    pub created_at: String,
}

/// Runtime state for auto strategy and circuit breaker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeState {
    pub key: String,
    pub value: String,
    #[serde(default)]
    pub updated_at: String,
}

/// AI route model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRoute {
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub prompt: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub created_at: String,
}

/// AI route task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRouteTask {
    pub id: i32,
    pub route_id: i32,
    pub status: String,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub error_message: Option<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// Backup info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub filename: String,
    pub size: i64,
    pub created_at: String,
}