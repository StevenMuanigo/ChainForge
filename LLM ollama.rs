use super::{LLMProvider, LLMRequest, LLMResponse, LLMStream, TokenUsage};
use async_trait::async_trait;
use anyhow::Result;

pub struct OllamaProvider {
    base_url: String,
    default_model: String,
}

impl OllamaProvider {
    pub fn new(base_url: String, default_model: String) -> Self {
        Self {
            base_url,
            default_model,
        }
    }
}

#[async_trait]
impl LLMProvider for OllamaProvider {
    async fn generate(&self, request: &LLMRequest) -> Result<LLMResponse> {
        let start = std::time::Instant::now();
        
        let model = request.model.as_ref().unwrap_or(&self.default_model);
        
        let client = reqwest::Client::new();
        let url = format!("{}/api/generate", self.base_url);
        
        let payload = serde_json::json!({
            "model": model,
            "prompt": request.prompt,
            "stream": false,
            "options": {
                "temperature": request.temperature.unwrap_or(0.7),
                "num_predict": request.max_tokens.unwrap_or(2048),
            }
        });
        
        let response = client
            .post(&url)
            .json(&payload)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        
        let text = response["response"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        
        Ok(LLMResponse {
            text,
            model: model.clone(),
            tokens_used: TokenUsage {
                prompt_tokens: 0, // Ollama doesn't provide token counts
                completion_tokens: 0,
                total_tokens: 0,
            },
            finish_reason: "stop".to_string(),
            latency_ms: start.elapsed().as_millis() as u64,
        })
    }
    
    async fn stream_generate(&self, _request: &LLMRequest) -> Result<LLMStream> {
        unimplemented!("Streaming not yet implemented for Ollama")
    }
    
    fn count_tokens(&self, text: &str) -> Result<usize> {
        Ok(text.split_whitespace().count())
    }
}
