use crate::InternalLLMRequest;

/// Parse OpenAI chat completion request into InternalLLMRequest
pub fn parse_openai_chat(body: &[u8]) -> Result<InternalLLMRequest, serde_json::Error> {
    let mut req: InternalLLMRequest = serde_json::from_slice(body)?;
    req.raw_format = Some("openai_chat".into());
    req.raw_body = Some(body.to_vec());
    Ok(req)
}

/// Parse OpenAI response request
pub fn parse_openai_response(body: &[u8]) -> Result<InternalLLMRequest, serde_json::Error> {
    let mut req: InternalLLMRequest = serde_json::from_slice(body)?;
    req.raw_format = Some("openai_response".into());
    req.raw_body = Some(body.to_vec());
    Ok(req)
}

/// Parse OpenAI embedding request
pub fn parse_openai_embedding(body: &[u8]) -> Result<InternalLLMRequest, serde_json::Error> {
    let mut req: InternalLLMRequest = serde_json::from_slice(body)?;
    req.raw_format = Some("openai_embedding".into());
    req.raw_body = Some(body.to_vec());
    Ok(req)
}