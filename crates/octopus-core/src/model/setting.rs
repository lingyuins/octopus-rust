use serde::{Deserialize, Serialize};

/// Setting model (key-value store)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    pub value: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub updated_at: String,
}

/// Setting update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingUpdateRequest {
    pub key: String,
    pub value: String,
}

/// Setting batch update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingBatchUpdateRequest {
    pub settings: Vec<SettingUpdateRequest>,
}