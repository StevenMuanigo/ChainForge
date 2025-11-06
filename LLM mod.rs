use async_trait::async_trait;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod openai;
pub mod ollama;
pub mod huggingface;
pub mod provider;

#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn generate(&self, request: &LLMRequest) -> Result<LLMResponse>;
    async fn stream_generate(&self, request: &LLMRequest) -> Result<LLMStream>;
    fn count_tokens(&self, text: &str) -> Result<usize>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMRequest {
    pub prompt: String,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<usize>,
    pub top_p: Option<f32>,
    pub stop_sequences: Option<Vec<String>>,
    pub system_message: Option<String>,
}

impl LLMRequest {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            model: None,
            temperature: None,
            max_tokens: None,
            top_p: None,
            stop_sequences: None,
            system_message: None,
        }
    }
    
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }
    
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }
    
    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }
    
    pub fn with_system_message(mut self, message: impl Into<String>) -> Self {
        self.system_message = Some(message.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub text: String,
    pub model: String,
    pub tokens_used: TokenUsage,
    pub finish_reason: String,
    pub latency_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

impl TokenUsage {
    pub fn estimate_cost(&self, model: &str) -> f64 {
        // Simplified cost estimation (USD)
        match model {
            m if m.contains("gpt-4") => {
                (self.prompt_tokens as f64 * 0.00003) + (self.completion_tokens as f64 * 0.00006)
            }
            m if m.contains("gpt-3.5") => {
                (self.prompt_tokens as f64 * 0.0000015) + (self.completion_tokens as f64 * 0.000002)
            }
            _ => 0.0,
        }
    }
}

pub type LLMStream = Box<dyn futures::Stream<Item = Result<String>> + Send + Unpin>;
