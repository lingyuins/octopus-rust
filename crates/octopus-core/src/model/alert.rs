use serde::{Deserialize, Serialize};

use crate::types::{AlertConditionType, NotifChannelType};

/// Alert rule model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub id: i32,
    pub name: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub condition_type: AlertConditionType,
    pub threshold: f64,
    #[serde(default)]
    pub scope: String,
    #[serde(default)]
    pub notif_channel_id: i32,
    #[serde(default)]
    pub cooldown_minutes: i32,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

fn default_true() -> bool {
    true
}

/// Alert notification channel model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertNotifChannel {
    pub id: i32,
    pub name: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub channel_type: NotifChannelType,
    pub config: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// Alert state record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertStateRecord {
    pub rule_id: i32,
    pub is_firing: bool,
    pub last_fired_at: Option<String>,
    pub last_resolved_at: Option<String>,
    pub fire_count: i32,
}

/// Alert history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertHistory {
    pub id: i32,
    pub rule_id: i32,
    pub rule_name: String,
    pub alert_type: AlertConditionType,
    pub message: String,
    pub notif_channel: String,
    pub created_at: String,
}

/// Alert rule create/update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRuleRequest {
    pub name: Option<String>,
    pub enabled: Option<bool>,
    pub condition_type: Option<AlertConditionType>,
    pub threshold: Option<f64>,
    pub scope: Option<String>,
    pub notif_channel_id: Option<i32>,
    pub cooldown_minutes: Option<i32>,
}

/// Alert notification channel create/update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertNotifChannelRequest {
    pub name: Option<String>,
    pub enabled: Option<bool>,
    pub channel_type: Option<NotifChannelType>,
    pub config: Option<String>,
}