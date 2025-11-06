use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod chunker;
pub mod retriever;
pub mod loader;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub content: String,
    pub metadata: serde_json::Value,
    pub source: String,
}

impl Document {
    pub fn new(content: String, source: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            content,
            metadata: serde_json::json!({}),
            source,
        }
    }
    
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: String,
    pub document_id: String,
    pub content: String,
    pub chunk_index: usize,
    pub metadata: serde_json::Value,
}
