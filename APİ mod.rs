use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub mod routes;
pub mod handlers;

use crate::config::AppConfig;
use crate::llm::provider::ProviderManager;
use crate::chains::manager::ChainManager;
use crate::agents::executor::AgentExecutor;
use crate::monitoring::MetricsCollector;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub provider_manager: Arc<ProviderManager>,
    pub chain_manager: Arc<ChainManager>,
    pub metrics: Arc<MetricsCollector>,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health & Status
        .route("/health", get(handlers::health_check))
        .route("/status", get(handlers::status))
        
        // LLM Endpoints
        .route("/llm/generate", post(handlers::llm_generate))
        .route("/llm/providers", get(handlers::list_providers))
        
        // Chain Endpoints
        .route("/chains", get(handlers::list_chains))
        .route("/chains/:id/execute", post(handlers::execute_chain))
        
        // RAG Endpoints
        .route("/rag/index", post(handlers::index_document))
        .route("/rag/query", post(handlers::rag_query))
        
        // Agent Endpoints
        .route("/agent/execute", post(handlers::agent_execute))
        
        // Memory Endpoints
        .route("/memory/session/:id", get(handlers::get_session))
        .route("/memory/session/:id", post(handlers::add_to_session))
        .route("/memory/session/:id/clear", post(handlers::clear_session))
        
        // Monitoring
        .route("/metrics", get(handlers::metrics))
        
        .with_state(state)
}
