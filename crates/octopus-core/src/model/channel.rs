use serde::{Deserialize, Serialize};

use crate::types::{AutoGroupType, OutboundType, ProxyUsageMode, RequestRewriteProfile};

/// Base URL with latency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseUrl {
    pub url: String,
    #[serde(default)]
    pub delay: i32,
    #[serde(default)]
    pub suffix_mode: String,
}

/// Custom header for channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomHeader {
    pub header_key: String,
    pub header_value: String,
}

/// Channel key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelKey {
    pub id: i32,
    pub channel_id: i32,
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub channel_key: String,
    #[serde(default)]
    pub status_code: i32,
    #[serde(default)]
    pub last_use_time_stamp: i64,
    #[serde(default)]
    pub total_cost: f64,
    #[serde(default)]
    pub remark: String,
}

fn default_true() -> bool {
    true
}

/// Request rewrite configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestRewriteConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub profile: RequestRewriteProfile,
    #[serde(default)]
    pub tool_role_strategy: Option<String>,
    #[serde(default)]
    pub system_message_strategy: Option<String>,
    #[serde(default)]
    pub header_profile: Option<String>,
}

/// Channel model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub group_id: i32,
    pub r#type: OutboundType,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub base_urls: Vec<BaseUrl>,
    #[serde(default)]
    pub keys: Vec<ChannelKey>,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub custom_model: String,
    #[serde(default)]
    pub proxy_mode: ProxyUsageMode,
    #[serde(default)]
    pub proxy_config_id: Option<i32>,
    #[serde(default)]
    pub proxy: bool,
    #[serde(default)]
    pub auto_sync: bool,
    #[serde(default)]
    pub auto_group: AutoGroupType,
    #[serde(default)]
    pub skip_model_test: bool,
    #[serde(default)]
    pub key_selection_strategy: String,
    #[serde(default)]
    pub custom_header: Vec<CustomHeader>,
    #[serde(default)]
    pub param_override: Option<String>,
    #[serde(default)]
    pub channel_proxy: Option<String>,
    #[serde(default)]
    pub request_rewrite: Option<RequestRewriteConfig>,
    #[serde(default)]
    pub match_regex: Option<String>,
    // Non-persisted fields
    #[serde(default)]
    pub managed: bool,
    #[serde(default)]
    pub managed_source: Option<crate::types::ManagedChannelSource>,
    #[serde(default)]
    pub stats: Option<Box<crate::model::stats::StatsChannel>>,
}

/// Channel update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelUpdateRequest {
    pub id: i32,
    pub name: Option<String>,
    pub group_id: Option<i32>,
    pub r#type: Option<OutboundType>,
    pub enabled: Option<bool>,
    pub base_urls: Option<Vec<BaseUrl>>,
    pub model: Option<String>,
    pub custom_model: Option<String>,
    pub proxy_mode: Option<ProxyUsageMode>,
    pub proxy_config_id: Option<i32>,
    pub proxy: Option<bool>,
    pub auto_sync: Option<bool>,
    pub skip_model_test: Option<bool>,
    pub key_selection_strategy: Option<String>,
    pub auto_group: Option<AutoGroupType>,
    pub custom_header: Option<Vec<CustomHeader>>,
    pub channel_proxy: Option<String>,
    pub param_override: Option<String>,
    pub request_rewrite: Option<RequestRewriteConfig>,
    pub match_regex: Option<String>,
    pub keys_to_add: Option<Vec<ChannelKeyAddRequest>>,
    pub keys_to_update: Option<Vec<ChannelKeyUpdateRequest>>,
    pub keys_to_delete: Option<Vec<i32>>,
}

/// Channel key add request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelKeyAddRequest {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub channel_key: String,
    #[serde(default)]
    pub remark: String,
}

/// Channel key update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelKeyUpdateRequest {
    pub id: i32,
    pub enabled: Option<bool>,
    pub channel_key: Option<String>,
    pub remark: Option<String>,
}

/// Channel fetch model request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelFetchModelRequest {
    pub r#type: OutboundType,
    pub base_url: String,
    pub key: String,
    #[serde(default)]
    pub proxy: bool,
}

impl Channel {
    /// Get the best base URL (lowest latency)
    pub fn get_base_url(&self) -> String {
        if self.base_urls.is_empty() {
            return String::new();
        }

        let mut best_url = String::new();
        let mut best_delay = 0;
        let mut best_set = false;

        for bu in &self.base_urls {
            if bu.url.is_empty() {
                continue;
            }
            if !best_set || bu.delay < best_delay {
                best_url = bu.url.clone();
                best_delay = bu.delay;
                best_set = true;
            }
        }

        best_url
    }

    /// Get normalized base URL with proper path suffix
    pub fn get_normalized_base_url(&self) -> String {
        let raw_url = self.get_base_url();
        normalize_channel_base_url(&raw_url, self.r#type, "")
    }

    /// Get an available channel key (lowest cost)
    pub fn get_channel_key(&self) -> Option<&ChannelKey> {
        self.get_channel_key_excluding(&[])
    }

    /// Get an available channel key excluding certain key IDs
    pub fn get_channel_key_excluding(&self, exclude_ids: &[i32]) -> Option<&ChannelKey> {
        let candidates: Vec<&ChannelKey> = self
            .keys
            .iter()
            .filter(|k| k.enabled && !k.channel_key.is_empty())
            .filter(|k| !exclude_ids.contains(&k.id))
            .collect();

        if candidates.is_empty() {
            return None;
        }

        // Select key with lowest total cost
        candidates
            .iter()
            .min_by(|a, b| a.total_cost.partial_cmp(&b.total_cost).unwrap_or(std::cmp::Ordering::Equal))
            .copied()
    }

    /// Count enabled keys with non-empty channel_key
    pub fn enabled_key_count(&self) -> usize {
        self.keys
            .iter()
            .filter(|k| k.enabled && !k.channel_key.is_empty())
            .count()
    }
}

/// Normalize channel base URL by appending appropriate path based on type
pub fn normalize_channel_base_url(raw_url: &str, channel_type: OutboundType, suffix_mode: &str) -> String {
    let trimmed = raw_url.trim_end_matches('/');
    if trimmed.is_empty() {
        return String::new();
    }

    match suffix_mode.to_lowercase().as_str() {
        "" => append_base_url_path_by_channel(trimmed, channel_type),
        "custom" => normalize_custom_base_url(trimmed, channel_type),
        "openai_compat" | "openai" => append_path_if_missing(trimmed, "/v1"),
        "anthropic" => append_path_if_missing(trimmed, "/v1"),
        "gemini" => append_path_if_missing(trimmed, "/v1beta"),
        "volcengine" => append_path_if_missing(trimmed, "/api/v3"),
        _ => append_base_url_path_by_channel(trimmed, channel_type),
    }
}

fn append_base_url_path_by_channel(raw_url: &str, channel_type: OutboundType) -> String {
    match channel_type {
        OutboundType::Anthropic => append_path_if_missing(raw_url, "/v1"),
        OutboundType::Gemini => append_path_if_missing(raw_url, "/v1beta"),
        OutboundType::Volcengine => append_path_if_missing(raw_url, "/api/v3"),
        OutboundType::Cloudflare => raw_url.to_string(),
        _ => append_path_if_missing(raw_url, "/v1"),
    }
}

fn normalize_custom_base_url(raw_url: &str, channel_type: OutboundType) -> String {
    match channel_type {
        OutboundType::OpenAIChat | OutboundType::OpenAIResponse | OutboundType::OpenAIEmbedding | OutboundType::MiMo => {
            trim_known_openai_endpoint_path(raw_url)
        }
        _ => raw_url.to_string(),
    }
}

fn trim_known_openai_endpoint_path(raw_url: &str) -> String {
    let lower = raw_url.to_lowercase();
    let suffixes = &[
        "/v1/chat/completions",
        "/chat/completions",
        "/v1/responses",
        "/responses",
        "/v1/embeddings",
        "/embeddings",
    ];

    for suffix in suffixes {
        if lower.ends_with(suffix) {
            return raw_url[..raw_url.len() - suffix.len()].trim_end_matches('/').to_string();
        }
    }

    raw_url.to_string()
}

fn append_path_if_missing(raw_url: &str, suffix: &str) -> String {
    let lower = raw_url.to_lowercase();
    if lower.ends_with(&suffix.to_lowercase()) {
        raw_url.to_string()
    } else {
        format!("{}{}", raw_url, suffix)
    }
}