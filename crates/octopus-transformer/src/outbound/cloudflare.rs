use crate::InternalLLMRequest;
pub fn to_cloudflare(req: &InternalLLMRequest, _model: &str) -> serde_json::Value {
    serde_json::json!({
        "messages": req.messages,
        "stream": req.stream,
        "max_tokens": req.max_tokens
    })
}
pub fn to_volcengine(req: &InternalLLMRequest, model: &str) -> serde_json::Value {
    serde_json::json!({
        "model": model,
        "messages": req.messages,
        "temperature": req.temperature,
        "top_p": req.top_p,
        "max_tokens": req.max_tokens,
        "stream": req.stream
    })
}
pub fn to_mimo(req: &InternalLLMRequest, model: &str) -> serde_json::Value {
    serde_json::json!({
        "model": model,
        "messages": req.messages,
        "temperature": req.temperature,
        "max_tokens": req.max_tokens,
        "stream": req.stream
    })
}