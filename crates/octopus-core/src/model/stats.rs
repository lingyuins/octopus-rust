use serde::{Deserialize, Serialize};

/// Stats total (aggregated)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsTotal {
    pub id: i32,
    pub total_requests: i64,
    pub total_tokens: i64,
    pub total_cost: f64,
    pub success_count: i64,
    pub fail_count: i64,
    pub updated_at: String,
}

/// Stats daily
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsDaily {
    pub id: i32,
    pub date: String,
    pub requests: i64,
    pub tokens: i64,
    pub cost: f64,
    pub success_count: i64,
    pub fail_count: i64,
}

/// Stats hourly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsHourly {
    pub id: i32,
    pub hour: String,
    pub requests: i64,
    pub tokens: i64,
    pub cost: f64,
    pub success_count: i64,
    pub fail_count: i64,
}

/// Stats per channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsChannel {
    pub channel_id: i32,
    pub requests: i64,
    pub tokens: i64,
    pub cost: f64,
    pub success_count: i64,
    pub fail_count: i64,
    #[serde(default)]
    pub avg_latency: f64,
}

/// Stats per model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsModel {
    pub model_name: String,
    pub requests: i64,
    pub tokens: i64,
    pub cost: f64,
    pub success_count: i64,
    pub fail_count: i64,
    #[serde(default)]
    pub avg_latency: f64,
}

/// Stats per API key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsApiKey {
    pub api_key_id: i32,
    pub requests: i64,
    pub tokens: i64,
    pub cost: f64,
    pub success_count: i64,
    pub fail_count: i64,
}

/// Stats site model hourly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsSiteModelHourly {
    pub id: i32,
    pub site_id: i32,
    pub model_name: String,
    pub hour: String,
    pub requests: i64,
    pub tokens: i64,
    pub cost: f64,
    pub success_count: i64,
    pub fail_count: i64,
}

/// Analytics overview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsOverview {
    pub total_requests: i64,
    pub total_tokens: i64,
    pub total_cost: f64,
    pub success_rate: f64,
    pub avg_latency: f64,
    pub active_channels: i32,
    pub active_keys: i32,
    pub today_requests: i64,
    pub today_cost: f64,
}

/// Analytics latency distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyDistribution {
    pub bucket: String,
    pub count: i64,
    pub percentage: f64,
}

/// Activity heatmap entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEntry {
    pub date: String,
    pub count: i64,
    pub level: u8,
}