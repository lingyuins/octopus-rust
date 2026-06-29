use crate::InternalLLMRequest;
/// Convert internal request to OpenAI chat format
pub fn to_openai_chat(req: &InternalLLMRequest, model: &str) -> serde_json::Value {
    serde_json::json!({
        "model": model,
        "messages": req.messages,
        "temperature": req.temperature,
        "top_p": req.top_p,
        "max_tokens": req.max_tokens,
        "stream": req.stream,
        "tools": req.tools,
        "tool_choice": req.tool_choice,
        "response_format": req.response_format,
        "stop": req.stop,
        "frequency_penalty": req.frequency_penalty,
        "presence_penalty": req.presence_penalty,
        "user": req.user
    })
}
pub fn to_openai_response(req: &InternalLLMRequest, model: &str) -> serde_json::Value {
    serde_json::json!({
        "model": model,
        "input": req.messages.last().map(|m| m.content.clone()),
        "stream": req.stream
    })
}
pub fn to_openai_embedding(req: &InternalLLMRequest, model: &str) -> serde_json::Value {
    serde_json::json!({
        "model": model,
        "input": req.input
    })
}