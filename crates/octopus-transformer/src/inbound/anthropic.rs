use crate::InternalLLMRequest;

/// Parse Anthropic messages request into InternalLLMRequest
pub fn parse_anthropic_messages(body: &[u8]) -> Result<InternalLLMRequest, serde_json::Error> {
    let anthropic: serde_json::Value = serde_json::from_slice(body)?;
    let mut req = InternalLLMRequest {
        model: anthropic.get("model").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        messages: anthropic.get("messages").map(|msgs| {
            serde_json::from_value(msgs.clone()).unwrap_or_default()
        }).unwrap_or_default(),
        max_tokens: anthropic.get("max_tokens").and_then(|v| v.as_i64()).map(|v| v as i32),
        temperature: anthropic.get("temperature").and_then(|v| v.as_f64()),
        stream: anthropic.get("stream").and_then(|v| v.as_bool()).unwrap_or(false),
        top_p: anthropic.get("top_p").and_then(|v| v.as_f64()),
        stop: anthropic.get("stop_sequences").and_then(|v| {
            v.as_array().map(|a| a.iter().filter_map(|s| s.as_str().map(String::from)).collect())
        }),
        ..Default::default()
    };
    req.raw_format = Some("anthropic".into());
    req.raw_body = Some(body.to_vec());
    Ok(req)
}

impl Default for InternalLLMRequest {
    fn default() -> Self {
        Self {
            model: String::new(),
            messages: vec![],
            temperature: None,
            top_p: None,
            max_tokens: None,
            stream: false,
            tools: None,
            tool_choice: None,
            response_format: None,
            stop: None,
            frequency_penalty: None,
            presence_penalty: None,
            user: None,
            input: None,
            raw_body: None,
            raw_format: None,
            extra_body: None,
        }
    }
}