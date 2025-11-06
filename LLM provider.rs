use super::{LLMProvider, LLMRequest, LLMResponse, LLMStream, TokenUsage};
use async_trait::async_trait;
use anyhow::{Result, Context};
use async_openai::{Client, types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessage, Role}};

pub struct OpenAIProvider {
    client: Client<async_openai::config::OpenAIConfig>,
    default_model: String,
    default_temperature: f32,
    default_max_tokens: usize,
}

impl OpenAIProvider {
    pub fn new(api_key: String, default_model: String, temperature: f32, max_tokens: usize) -> Self {
        let config = async_openai::config::OpenAIConfig::new().with_api_key(api_key);
        let client = Client::with_config(config);
        
        Self {
            client,
            default_model,
            default_temperature: temperature,
            default_max_tokens: max_tokens,
        }
    }
}

#[async_trait]
impl LLMProvider for OpenAIProvider {
    async fn generate(&self, request: &LLMRequest) -> Result<LLMResponse> {
        let start = std::time::Instant::now();
        
        let model = request.model.as_ref().unwrap_or(&self.default_model).clone();
        
        let mut messages = vec![];
        
        if let Some(system_msg) = &request.system_message {
            messages.push(ChatCompletionRequestMessage::System(
                async_openai::types::ChatCompletionRequestSystemMessage {
                    content: async_openai::types::ChatCompletionRequestSystemMessageContent::Text(system_msg.clone()),
                    role: Role::System,
                    name: None,
                }
            ));
        }
        
        messages.push(ChatCompletionRequestMessage::User(
            async_openai::types::ChatCompletionRequestUserMessage {
                content: async_openai::types::ChatCompletionRequestUserMessageContent::Text(request.prompt.clone()),
                role: Role::User,
                name: None,
            }
        ));
        
        let chat_request = CreateChatCompletionRequestArgs::default()
            .model(&model)
            .messages(messages)
            .temperature(request.temperature.unwrap_or(self.default_temperature))
            .max_tokens(request.max_tokens.unwrap_or(self.default_max_tokens) as u16)
            .build()
            .context("Failed to build chat completion request")?;
        
        let response = self.client
            .chat()
            .create(chat_request)
            .await
            .context("Failed to call OpenAI API")?;
        
        let text = response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_default();
        
        let usage = response.usage.unwrap_or_default();
        
        Ok(LLMResponse {
            text,
            model,
            tokens_used: TokenUsage {
                prompt_tokens: usage.prompt_tokens as usize,
                completion_tokens: usage.completion_tokens as usize,
                total_tokens: usage.total_tokens as usize,
            },
            finish_reason: response
                .choices
                .first()
                .map(|c| format!("{:?}", c.finish_reason))
                .unwrap_or_default(),
            latency_ms: start.elapsed().as_millis() as u64,
        })
    }
    
    async fn stream_generate(&self, _request: &LLMRequest) -> Result<LLMStream> {
        // Streaming implementation placeholder
        unimplemented!("Streaming not yet implemented for OpenAI")
    }
    
    fn count_tokens(&self, text: &str) -> Result<usize> {
        // Simplified token counting (approximate)
        Ok(text.split_whitespace().count())
    }
}
