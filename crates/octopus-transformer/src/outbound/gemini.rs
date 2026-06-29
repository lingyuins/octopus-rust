use crate::InternalLLMRequest;
pub fn to_gemini(req: &InternalLLMRequest, _model: &str) -> serde_json::Value {
    let contents: Vec<serde_json::Value> = req.messages.iter().map(|m| {
        serde_json::json!({
            "role": if m.role == "assistant" { "model" } else { "user" },
            "parts": [{"text": format!("{:?}", m.content)}]
        })
    }).collect();
    serde_json::json!({
        "contents": contents,
        "generationConfig": {
            "temperature": req.temperature,
            "topP": req.top_p,
            "maxOutputTokens": req.max_tokens
        }
    })
}