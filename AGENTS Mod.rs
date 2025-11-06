use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod tools;
pub mod executor;

#[async_trait]
pub trait Tool: Send + Sync {
    async fn execute(&self, input: &str) -> Result<ToolOutput>;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> ToolParameters;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameters {
    pub required: Vec<String>,
    pub optional: Vec<String>,
    pub schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    pub result: String,
    pub success: bool,
    pub metadata: serde_json::Value,
}

impl ToolOutput {
    pub fn success(result: impl Into<String>) -> Self {
        Self {
            result: result.into(),
            success: true,
            metadata: serde_json::json!({}),
        }
    }
    
    pub fn error(error: impl Into<String>) -> Self {
        Self {
            result: error.into(),
            success: false,
            metadata: serde_json::json!({}),
        }
    }
}
