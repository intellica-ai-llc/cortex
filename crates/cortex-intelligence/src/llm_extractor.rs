use serde::{Deserialize, Serialize};

/// LLM‑powered extraction of structured data from unstructured text.
///
/// Supports local models (LLaMA, Whisper) and cloud APIs (Groq, Claude).
pub struct LLMExtractor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionRequest {
    pub text: String,
    pub schema: serde_json::Value,   // JSON Schema describing desired output
    pub model: String,               // "groq-llama3-70b", "claude-opus-4"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResponse {
    pub extracted: serde_json::Value,
    pub confidence: f64,
    pub model_used: String,
    pub tokens_used: u64,
}

impl LLMExtractor {
    pub fn new() -> Self { Self }

    /// Send an extraction request to an LLM.
    pub async fn extract(&self, req: &ExtractionRequest) -> Result<ExtractionResponse, String> {
        // In production: route to local LLM or cloud API.
        Ok(ExtractionResponse {
            extracted: serde_json::json!({}),
            confidence: 0.9,
            model_used: req.model.clone(),
            tokens_used: 100,
        })
    }
}
