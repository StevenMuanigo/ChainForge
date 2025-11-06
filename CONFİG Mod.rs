use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::PathBuf;

pub mod settings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub llm: LlmConfig,
    pub embeddings: EmbeddingsConfig,
    pub memory: MemoryConfig,
    pub rag: RagConfig,
    pub chains: ChainsConfig,
    pub agents: AgentsConfig,
    pub monitoring: MonitoringConfig,
    pub database: DatabaseConfig,
    pub plugins: PluginsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub default_provider: String,
    pub providers: LlmProviders,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmProviders {
    pub openai: OpenAIConfig,
    pub ollama: OllamaConfig,
    pub huggingface: HuggingFaceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    pub api_key_env: String,
    pub default_model: String,
    pub temperature: f32,
    pub max_tokens: usize,
    pub top_p: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub base_url: String,
    pub default_model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuggingFaceConfig {
    pub api_key_env: String,
    pub default_model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingsConfig {
    pub provider: String,
    pub model: String,
    pub batch_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub session_backend: String,
    pub redis: RedisConfig,
    pub vector_store: String,
    pub qdrant: QdrantConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub ttl_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QdrantConfig {
    pub url: String,
    pub collection_name: String,
    pub vector_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagConfig {
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub retrieval_top_k: usize,
    pub similarity_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainsConfig {
    pub max_iterations: usize,
    pub timeout_seconds: u64,
    pub enable_graph_view: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsConfig {
    pub max_tool_calls: usize,
    pub tool_timeout_seconds: u64,
    pub enable_reasoning_logs: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enable_metrics: bool,
    pub metrics_port: u16,
    pub log_level: String,
    pub log_format: String,
    pub enable_token_tracking: bool,
    pub enable_cost_tracking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginsConfig {
    pub tools_directory: PathBuf,
    pub enable_hot_reload: bool,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        dotenv::dotenv().ok();
        
        let config = config::Config::builder()
            .add_source(config::File::with_name("config"))
            .add_source(config::Environment::with_prefix("CHAINFORGE"))
            .build()?;
        
        let app_config: AppConfig = config.try_deserialize()?;
        
        Ok(app_config)
    }
    
    pub fn get_openai_api_key(&self) -> Result<String> {
        std::env::var(&self.llm.providers.openai.api_key_env)
            .map_err(|_| anyhow::anyhow!("OpenAI API key not found"))
    }
    
    pub fn get_hf_api_key(&self) -> Result<String> {
        std::env::var(&self.llm.providers.huggingface.api_key_env)
            .map_err(|_| anyhow::anyhow!("Hugging Face API key not found"))
    }
}
