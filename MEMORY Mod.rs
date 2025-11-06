use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod session;
pub mod vector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

impl Message {
    pub fn new(role: MessageRole, content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role,
            content: content.into(),
            timestamp: chrono::Utc::now(),
        }
    }
}

#[async_trait]
pub trait SessionMemory: Send + Sync {
    async fn add_message(&self, session_id: &str, message: Message) -> Result<()>;
    async fn get_messages(&self, session_id: &str, limit: Option<usize>) -> Result<Vec<Message>>;
    async fn clear_session(&self, session_id: &str) -> Result<()>;
    async fn get_context(&self, session_id: &str) -> Result<String>;
}

#[async_trait]
pub trait VectorMemory: Send + Sync {
    async fn store(&self, id: &str, text: &str, embedding: Vec<f32>, metadata: serde_json::Value) -> Result<()>;
    async fn search(&self, query_embedding: Vec<f32>, top_k: usize, threshold: f32) -> Result<Vec<SearchResult>>;
    async fn delete(&self, id: &str) -> Result<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub text: String,
    pub score: f32,
    pub metadata: serde_json::Value,
}
