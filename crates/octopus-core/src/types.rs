use serde::{Deserialize, Serialize};

/// Outbound channel type (provider)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutboundType {
    OpenAI,
    OpenAIChat,
    OpenAIResponse,
    OpenAIEmbedding,
    Anthropic,
    Gemini,
    Cloudflare,
    Volcengine,
    MiMo,
    #[serde(other)]
    Unknown,
}

impl OutboundType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OutboundType::OpenAI => "openai",
            OutboundType::OpenAIChat => "openai_chat",
            OutboundType::OpenAIResponse => "openai_response",
            OutboundType::OpenAIEmbedding => "openai_embedding",
            OutboundType::Anthropic => "anthropic",
            OutboundType::Gemini => "gemini",
            OutboundType::Cloudflare => "cloudflare",
            OutboundType::Volcengine => "volcengine",
            OutboundType::MiMo => "mimo",
            OutboundType::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for OutboundType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for OutboundType {
    fn default() -> Self {
        OutboundType::Unknown
    }
}

/// Inbound protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InboundType {
    OpenAIChat,
    OpenAIResponse,
    OpenAIEmbedding,
    Anthropic,
}

/// Endpoint type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointType {
    Chat,
    Responses,
    Messages,
    Embeddings,
    Images,
    Audio,
    Video,
    Search,
    Rerank,
    Moderation,
}

/// Group mode (load balancing strategy)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupMode {
    RoundRobin,
    Random,
    Failover,
    Weighted,
    Auto,
}

impl Default for GroupMode {
    fn default() -> Self {
        GroupMode::RoundRobin
    }
}

/// Auto group type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutoGroupType {
    None = 0,
    Fuzzy = 1,
    Exact = 2,
    Regex = 3,
}

impl Default for AutoGroupType {
    fn default() -> Self {
        AutoGroupType::None
    }
}

impl AutoGroupType {
    pub fn is_valid(&self) -> bool {
        matches!(
            self,
            AutoGroupType::None | AutoGroupType::Fuzzy | AutoGroupType::Exact | AutoGroupType::Regex
        )
    }
}

/// Request rewrite profile
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestRewriteProfile {
    #[default]
    Preserve,
    OpenAIChatCompat,
    Codex,
}

/// Tool role strategy
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolRoleStrategy {
    Keep,
    StringifyToUser,
}

/// System message strategy
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemMessageStrategy {
    Keep,
    Merge,
}

/// Proxy usage mode
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyUsageMode {
    Direct,
    System,
    Pool,
    Inherit,
}

impl Default for ProxyUsageMode {
    fn default() -> Self {
        ProxyUsageMode::Direct
    }
}

/// User role
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Admin,
    Editor,
    Viewer,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Admin
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::Editor => write!(f, "editor"),
            UserRole::Viewer => write!(f, "viewer"),
        }
    }
}

/// Managed channel source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedChannelSource {
    pub site_id: u32,
    pub site_name: String,
    pub account_id: u32,
    pub account_name: String,
}

/// Site platform type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SitePlatform {
    NewAPI,
    OneAPI,
    OneHub,
    DoneHub,
    Sub2API,
    AnyRouter,
    OpenAI,
    Claude,
    Gemini,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for SitePlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SitePlatform::NewAPI => write!(f, "new-api"),
            SitePlatform::OneAPI => write!(f, "one-api"),
            SitePlatform::OneHub => write!(f, "one-hub"),
            SitePlatform::DoneHub => write!(f, "done-hub"),
            SitePlatform::Sub2API => write!(f, "sub2api"),
            SitePlatform::AnyRouter => write!(f, "anyrouter"),
            SitePlatform::OpenAI => write!(f, "openai"),
            SitePlatform::Claude => write!(f, "claude"),
            SitePlatform::Gemini => write!(f, "gemini"),
            SitePlatform::Unknown => write!(f, "unknown"),
        }
    }
}

/// Alert condition type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertConditionType {
    ErrorRate,
    CostThreshold,
    QuotaExceeded,
    ChannelDown,
}

/// Notification channel type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotifChannelType {
    Webhook,
    Gotify,
    Email,
    Telegram,
    Feishu,
    DingTalk,
    WeCom,
    Ntfy,
}

/// Setting key enum
///
/// `Serialize` is derived. `Deserialize` is implemented by hand below so that
/// any unrecognized key is captured as `Other(String)` — serde's
/// `#[serde(other)]` only works on unit variants, so it cannot fill
/// `Other(String)` directly.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum SettingKey {
    #[serde(rename = "trusted_proxies")]
    TrustedProxies,
    #[serde(rename = "nav_order")]
    NavOrder,
    #[serde(rename = "nav_visible")]
    NavVisible,
    #[serde(rename = "jwt_default_expiry_minutes")]
    JWTDefaultExpiryMinutes,
    #[serde(rename = "jwt_remember_me_expiry_days")]
    JWTRememberMeExpiryDays,
    #[serde(rename = "ratelimit_cooldown")]
    RatelimitCooldown,
    #[serde(rename = "key_selection_strategy")]
    KeySelectionStrategy,
    #[serde(rename = "auto_strategy_window_size")]
    AutoStrategyWindowSize,
    #[serde(rename = "auto_strategy_min_samples")]
    AutoStrategyMinSamples,
    #[serde(rename = "circuit_breaker_threshold")]
    CircuitBreakerThreshold,
    #[serde(rename = "circuit_breaker_cooldown")]
    CircuitBreakerCooldown,
    #[serde(rename = "relay_timeout_seconds")]
    RelayTimeoutSeconds,
    #[serde(rename = "max_retry_attempts")]
    MaxRetryAttempts,
    #[serde(rename = "response_filter_keywords")]
    ResponseFilterKeywords,
    #[serde(rename = "normalize_models")]
    NormalizeModels,
    #[serde(rename = "semantic_cache_enabled")]
    SemanticCacheEnabled,
    #[serde(rename = "webdav_url")]
    WebDAVUrl,
    #[serde(rename = "webdav_username")]
    WebDAVUsername,
    #[serde(rename = "webdav_password")]
    WebDAVPassword,
    #[serde(rename = "backup_schedule")]
    BackupSchedule,
    #[serde(rename = "llm_sync_enabled")]
    LLMSyncEnabled,
    #[serde(rename = "auto_checkin_enabled")]
    AutoCheckinEnabled,
    #[serde(rename = "site_sync_enabled")]
    SiteSyncEnabled,
    #[serde(rename = "webauthn_rp_id")]
    WebAuthnRPID,
    #[serde(rename = "webauthn_rp_name")]
    WebAuthnRPName,
    #[serde(rename = "webauthn_rp_origin")]
    WebAuthnRPOrigin,
    // Unknown keys are captured into `Other(String)` via the custom `Deserialize`
    // impl below. (`#[serde(other)]` cannot be used because it requires a unit
    // variant, but we want to preserve the original key string.)
    Other(String),
}

impl<'de> Deserialize<'de> for SettingKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "trusted_proxies" => SettingKey::TrustedProxies,
            "nav_order" => SettingKey::NavOrder,
            "nav_visible" => SettingKey::NavVisible,
            "jwt_default_expiry_minutes" => SettingKey::JWTDefaultExpiryMinutes,
            "jwt_remember_me_expiry_days" => SettingKey::JWTRememberMeExpiryDays,
            "ratelimit_cooldown" => SettingKey::RatelimitCooldown,
            "key_selection_strategy" => SettingKey::KeySelectionStrategy,
            "auto_strategy_window_size" => SettingKey::AutoStrategyWindowSize,
            "auto_strategy_min_samples" => SettingKey::AutoStrategyMinSamples,
            "circuit_breaker_threshold" => SettingKey::CircuitBreakerThreshold,
            "circuit_breaker_cooldown" => SettingKey::CircuitBreakerCooldown,
            "relay_timeout_seconds" => SettingKey::RelayTimeoutSeconds,
            "max_retry_attempts" => SettingKey::MaxRetryAttempts,
            "response_filter_keywords" => SettingKey::ResponseFilterKeywords,
            "normalize_models" => SettingKey::NormalizeModels,
            "semantic_cache_enabled" => SettingKey::SemanticCacheEnabled,
            "webdav_url" => SettingKey::WebDAVUrl,
            "webdav_username" => SettingKey::WebDAVUsername,
            "webdav_password" => SettingKey::WebDAVPassword,
            "backup_schedule" => SettingKey::BackupSchedule,
            "llm_sync_enabled" => SettingKey::LLMSyncEnabled,
            "auto_checkin_enabled" => SettingKey::AutoCheckinEnabled,
            "site_sync_enabled" => SettingKey::SiteSyncEnabled,
            "webauthn_rp_id" => SettingKey::WebAuthnRPID,
            "webauthn_rp_name" => SettingKey::WebAuthnRPName,
            "webauthn_rp_origin" => SettingKey::WebAuthnRPOrigin,
            _ => SettingKey::Other(s),
        })
    }
}

impl std::fmt::Display for SettingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingKey::Other(s) => write!(f, "{}", s),
            _ => {
                let s = serde_json::to_string(self).unwrap_or_default();
                write!(f, "{}", s.trim_matches('"'))
            }
        }
    }
}