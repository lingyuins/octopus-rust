use serde::{Deserialize, Serialize};

/// LLM pricing info model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMInfo {
    pub id: i32,
    pub model_name: String,
    pub display_name: String,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub input_price: f64,
    #[serde(default)]
    pub output_price: f64,
    #[serde(default)]
    pub cache_input_price: f64,
    #[serde(default)]
    pub unit: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// LLM price update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMPriceUpdateRequest {
    pub id: i32,
    pub input_price: Option<f64>,
    pub output_price: Option<f64>,
    pub cache_input_price: Option<f64>,
    pub enabled: Option<bool>,
}

/// Model capability info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCapability {
    pub model_name: String,
    pub supports_chat: bool,
    pub supports_responses: bool,
    pub supports_embeddings: bool,
    pub supports_vision: bool,
    pub supports_tools: bool,
    pub supports_streaming: bool,
    pub max_tokens: i32,
    pub max_input_tokens: i32,
    pub max_output_tokens: i32,
}