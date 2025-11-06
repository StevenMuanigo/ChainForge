use serde::{Deserialize, Serialize};

// LLM Requests/Responses
#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub prompt: String,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub text: String,
    pub model: String,
    pub tokens_used: usize,
    pub latency_ms: u64,
    pub cost: f64,
}

// Chain Requests/Responses
#[derive(Debug, Deserialize)]
pub struct ExecuteChainRequest {
    pub variables: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ExecuteChainResponse {
    pub result: serde_json::Value,
    pub execution_time_ms: u64,
    pub total_tokens: usize,
    pub total_cost: f64,
}

// RAG Requests/Responses
#[derive(Debug, Deserialize)]
pub struct IndexDocumentRequest {
    pub content: String,
    pub source: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct IndexDocumentResponse {
    pub document_id: String,
    pub chunks_created: usize,
}

#[derive(Debug, Deserialize)]
pub struct RAGQueryRequest {
    pub query: String,
    pub top_k: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct RAGQueryResponse {
    pub answer: String,
    pub context: Vec<String>,
    pub sources: Vec<String>,
}

// Agent Requests/Responses
#[derive(Debug, Deserialize)]
pub struct AgentExecuteRequest {
    pub task: String,
    pub tools: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct AgentExecuteResponse {
    pub final_answer: String,
    pub steps: Vec<serde_json::Value>,
    pub total_iterations: usize,
}

// Memory Requests/Responses
#[derive(Debug, Deserialize)]
pub struct AddMessageRequest {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub session_id: String,
    pub messages: Vec<serde_json::Value>,
}

// Status Response
#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub total_requests: u64,
    pub active_chains: usize,
}
