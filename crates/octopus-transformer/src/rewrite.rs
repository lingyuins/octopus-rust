use crate::InternalLLMRequest;
use regex::Regex;

/// Rewrite an internal request (e.g., model name, headers)
pub fn rewrite_request(req: &mut InternalLLMRequest, rewrite_config: Option<&serde_json::Value>) {
    if let Some(config) = rewrite_config {
        if let Some(profile) = config.get("profile").and_then(|v| v.as_str()) {
            match profile {
                "preserve" => {}
                "openai_chat_compat" => rewrite_openai_compat(req),
                _ => {}
            }
        }
    }
}

fn rewrite_openai_compat(req: &mut InternalLLMRequest) {
    // Normalize role names
    for msg in &mut req.messages {
        if msg.role == "model" {
            msg.role = "assistant".to_string();
        }
    }
}

/// Apply model name mapping
pub fn map_model_name(model: &str, mappings: &[(String, String)]) -> String {
    for (pattern, replacement) in mappings {
        if let Ok(re) = Regex::new(pattern) {
            if re.is_match(model) {
                return re.replace(model, replacement.as_str()).to_string();
            }
        } else if pattern.contains('*') {
            let escaped = regex::escape(pattern).replace("\\*", ".*");
            if let Ok(re) = Regex::new(&format!("^{}$", escaped)) {
                if re.is_match(model) {
                    return replacement.clone();
                }
            }
        } else if pattern == model {
            return replacement.clone();
        }
    }
    model.to_string()
}