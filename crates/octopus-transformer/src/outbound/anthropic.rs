use crate::InternalLLMRequest;
pub fn to_anthropic(req: &InternalLLMRequest, model: &str) -> serde_json::Value {
    serde_json::json!({
        "model": model,
        "messages": req.messages,
        "max_tokens": req.max_tokens.unwrap_or(4096),
        "temperature": req.temperature,
        "stream": req.stream,
        "system": req.messages.iter().find(|m| m.role == "system").map(|m| &m.content)
    })
}