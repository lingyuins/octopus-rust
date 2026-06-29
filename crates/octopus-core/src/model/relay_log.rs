use serde::{Deserialize, Serialize};

/// Relay log model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayLog {
    pub id: i64,
    pub request_id: String,
    pub api_key_id: i32,
    pub group_id: i32,
    pub channel_id: i32,
    pub key_id: i32,
    pub model_name: String,
    pub upstream_model: Option<String>,
    pub request_body: Option<String>,
    pub response_body: Option<String>,
    pub status_code: i32,
    pub latency_ms: i64,
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
    pub cost: f64,
    pub error_message: Option<String>,
    pub client_ip: String,
    pub created_at: String,
}

/// Relay log attempt (for retry tracking)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayLogAttempt {
    pub id: i64,
    pub relay_log_id: i64,
    pub attempt: i32,
    pub channel_id: i32,
    pub key_id: i32,
    pub status_code: i32,
    pub latency_ms: i64,
    pub error_message: Option<String>,
    pub created_at: String,
}

/// Relay log query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayLogQuery {
    #[serde(default)]
    pub page: i32,
    #[serde(default = "default_page_size")]
    pub page_size: i32,
    pub api_key_id: Option<i32>,
    pub channel_id: Option<i32>,
    pub group_id: Option<i32>,
    pub model_name: Option<String>,
    pub status_code: Option<i32>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub request_id: Option<String>,
}

fn default_page_size() -> i32 {
    20
}