use anyhow::Result;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

pub mod simple;
pub mod sequential;
pub mod pipeline;
pub mod manager;

#[async_trait]
pub trait Chain: Send + Sync {
    async fn execute(&self, input: ChainInput) -> Result<ChainOutput>;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainInput {
    pub variables: serde_json::Map<String, serde_json::Value>,
}

impl ChainInput {
    pub fn new() -> Self {
        Self {
            variables: serde_json::Map::new(),
        }
    }
    
    pub fn with_variable(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.variables.insert(key.into(), value);
        self
    }
    
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.variables.get(key)?.as_str().map(|s| s.to_string())
    }
}

impl Default for ChainInput {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainOutput {
    pub result: serde_json::Value,
    pub metadata: ChainMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainMetadata {
    pub chain_name: String,
    pub execution_time_ms: u64,
    pub steps: Vec<StepInfo>,
    pub total_tokens: usize,
    pub total_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepInfo {
    pub name: String,
    pub duration_ms: u64,
    pub input: String,
    pub output: String,
}
