use super::{LLMProvider, LLMRequest, LLMResponse, LLMStream, TokenUsage};
use async_trait::async_trait;
use anyhow::Result;

pub struct HuggingFaceProvider {
    api_key: String,
    default_model: String,
}

impl HuggingFaceProvider {
    pub fn new(api_key: String, default_model: String) -> Self {
        Self {
            api_key,
            default_model,
        }
    }
}

#[async_trait]
impl LLMProvider for HuggingFaceProvider {
    async fn generate(&self, request: &LLMRequest) -> Result<LLMResponse> {
        let start = std::time::Instant::now();
        
        let model = request.model.as_ref().unwrap_or(&self.default_model);
        
        let client = reqwest::Client::new();
        let url = format!("https://api-inference.huggingface.co/models/{}", model);
        
        let payload = serde_json::json!({
            "inputs": request.prompt,
            "parameters": {
                "temperature": request.temperature.unwrap_or(0.7),
                "max_new_tokens": request.max_tokens.unwrap_or(2048),
                "top_p": request.top_p.unwrap_or(1.0),
            }
        });
        
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        
        let text = if let Some(arr) = response.as_array() {
            arr.first()
                .and_then(|v| v["generated_text"].as_str())
                .unwrap_or_default()
                .to_string()
        } else {
            response["generated_text"]
                .as_str()
                .unwrap_or_default()
                .to_string()
        };
        
        Ok(LLMResponse {
            text,
            model: model.clone(),
            tokens_used: TokenUsage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
            finish_reason: "stop".to_string(),
            latency_ms: start.elapsed().as_millis() as u64,
        })
    }
    
    async fn stream_generate(&self, _request: &LLMRequest) -> Result<LLMStream> {
        unimplemented!("Streaming not yet implemented for Hugging Face")
    }
    
    fn count_tokens(&self, text: &str) -> Result<usize> {
        Ok(text.split_whitespace().count())
    }
}
