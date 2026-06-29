use serde::{Deserialize, Serialize};

use crate::types::GroupMode;

/// Group model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub endpoint_type: String,
    #[serde(default)]
    pub endpoint_provider: String,
    #[serde(default)]
    pub outbound_format: String,
    pub mode: GroupMode,
    #[serde(default)]
    pub match_regex: String,
    #[serde(default)]
    pub first_token_time_out: i32,
    #[serde(default)]
    pub session_keep_time: i32,
    #[serde(default)]
    pub condition: Option<String>,
    #[serde(default)]
    pub items: Vec<GroupItem>,
    #[serde(default)]
    pub last_test_passed: Option<bool>,
    #[serde(default)]
    pub last_test_at: i64,
}

/// Group item (channel binding)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupItem {
    pub id: i32,
    pub group_id: i32,
    pub channel_id: i32,
    pub model_name: String,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub weight: i32,
}

/// Group update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupUpdateRequest {
    pub id: i32,
    pub name: Option<String>,
    pub endpoint_type: Option<String>,
    pub endpoint_provider: Option<String>,
    pub outbound_format: Option<String>,
    pub mode: Option<GroupMode>,
    pub match_regex: Option<String>,
    pub condition: Option<String>,
    pub first_token_time_out: Option<i32>,
    pub session_keep_time: Option<i32>,
    pub items_to_add: Option<Vec<GroupItemAddRequest>>,
    pub items_to_update: Option<Vec<GroupItemUpdateRequest>>,
    pub items_to_delete: Option<Vec<i32>>,
}

/// Group item add request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupItemAddRequest {
    pub channel_id: i32,
    pub model_name: String,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub weight: i32,
}

/// Group item update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupItemUpdateRequest {
    pub id: i32,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub weight: i32,
}

/// Group ID and LLM name DTO
#[derive(Debug, Clone)]
pub struct GroupIdAndLLMName {
    pub channel_id: i32,
    pub model_name: String,
}

// Group mode constants
impl GroupMode {
    pub fn as_int(&self) -> i32 {
        match self {
            GroupMode::RoundRobin => 1,
            GroupMode::Random => 2,
            GroupMode::Failover => 3,
            GroupMode::Weighted => 4,
            GroupMode::Auto => 5,
        }
    }

    pub fn from_int(v: i32) -> Self {
        match v {
            1 => GroupMode::RoundRobin,
            2 => GroupMode::Random,
            3 => GroupMode::Failover,
            4 => GroupMode::Weighted,
            5 => GroupMode::Auto,
            _ => GroupMode::RoundRobin,
        }
    }
}