use serde::{Deserialize, Serialize};

/// Audit log model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: i32,
    pub user_id: i32,
    pub username: String,
    pub action: String,
    pub resource: String,
    pub resource_id: Option<String>,
    pub details: Option<String>,
    pub ip_address: String,
    pub user_agent: String,
    pub created_at: String,
}

/// Audit log request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogQuery {
    #[serde(default)]
    pub page: i32,
    #[serde(default = "default_page_size")]
    pub page_size: i32,
    pub user_id: Option<i32>,
    pub action: Option<String>,
    pub resource: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

fn default_page_size() -> i32 {
    20
}