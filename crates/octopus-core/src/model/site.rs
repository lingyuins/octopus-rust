use serde::{Deserialize, Serialize};

use crate::types::SitePlatform;

/// Site model (third-party management platform)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    pub id: i32,
    pub name: String,
    pub platform: SitePlatform,
    #[serde(default)]
    pub base_url: String,
    #[serde(default)]
    pub proxy_mode: String,
    #[serde(default)]
    pub proxy_config_id: Option<i32>,
    #[serde(default)]
    pub archived: bool,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// Site account model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteAccount {
    pub id: i32,
    pub site_id: i32,
    pub name: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub access_token: String,
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub sync_enabled: bool,
    #[serde(default)]
    pub checkin_enabled: bool,
    #[serde(default)]
    pub proxy_mode: String,
    #[serde(default)]
    pub proxy_config_id: Option<i32>,
    #[serde(default)]
    pub last_sync_at: Option<String>,
    #[serde(default)]
    pub last_checkin_at: Option<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// Site token (API key from upstream)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteToken {
    pub id: i32,
    pub site_id: i32,
    pub account_id: i32,
    pub name: String,
    pub key: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub balance: f64,
    #[serde(default)]
    pub created_at: String,
}

/// Site user group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteUserGroup {
    pub id: i32,
    pub site_id: i32,
    pub account_id: i32,
    pub name: String,
    pub key: String,
    #[serde(default)]
    pub created_at: String,
}

/// Site model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteModel {
    pub id: i32,
    pub site_id: i32,
    pub account_id: i32,
    pub model_name: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub created_at: String,
}

/// Site channel binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteChannelBinding {
    pub id: i32,
    pub site_id: i32,
    pub account_id: i32,
    pub channel_id: i32,
    pub token_id: i32,
    #[serde(default)]
    pub model_name: String,
    #[serde(default)]
    pub created_at: String,
}

/// Site create/update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteRequest {
    pub name: Option<String>,
    pub platform: Option<SitePlatform>,
    pub base_url: Option<String>,
    pub proxy_mode: Option<String>,
    pub proxy_config_id: Option<i32>,
}

/// Site account create/update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteAccountRequest {
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub access_token: Option<String>,
    pub api_key: Option<String>,
    pub sync_enabled: Option<bool>,
    pub checkin_enabled: Option<bool>,
    pub proxy_mode: Option<String>,
    pub proxy_config_id: Option<i32>,
}

/// Site import request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteImportRequest {
    pub url: String,
    pub name: Option<String>,
}

/// Site discovery result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteDiscoveryResult {
    pub platform: SitePlatform,
    pub name: String,
    pub base_url: String,
    pub accounts: Vec<SiteDiscoveryAccount>,
}

/// Site discovery account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteDiscoveryAccount {
    pub name: String,
    pub key: String,
}