use super::AppState;
use super::routes::*;
use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::llm::LLMRequest;
use crate::chains::ChainInput;

// Health Check
pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "ChainForge",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// Status
pub async fn status(State(state): State<AppState>) -> impl IntoResponse {
    let stats = state.metrics.get_stats();
    
    Json(StatusResponse {
        status: "running".to_string(),
        version: "1.0.0".to_string(),
        uptime_seconds: 0, // TODO: Track actual uptime
        total_requests: stats.total_requests,
        active_chains: state.chain_manager.list_chains().len(),
    })
}

// LLM Generate
pub async fn llm_generate(
    State(state): State<AppState>,
    Json(req): Json<GenerateRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    state.metrics.record_request();
    
    let provider = state.provider_manager
        .get_provider(req.provider.as_deref())
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    let llm_request = LLMRequest::new(req.prompt)
        .with_temperature(req.temperature.unwrap_or(0.7))
        .with_max_tokens(req.max_tokens.unwrap_or(2048));
    
    let response = provider
        .generate(&llm_request)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    state.metrics.record_llm_latency(response.latency_ms);
    state.metrics.record_token_usage(response.tokens_used.total_tokens);
    
    Ok(Json(GenerateResponse {
        text: response.text,
        model: response.model.clone(),
        tokens_used: response.tokens_used.total_tokens,
        latency_ms: response.latency_ms,
        cost: response.tokens_used.estimate_cost(&response.model),
    }))
}

// List Providers
pub async fn list_providers(State(state): State<AppState>) -> impl IntoResponse {
    let providers = state.provider_manager.list_providers();
    Json(serde_json::json!({
        "providers": providers
    }))
}

// List Chains
pub async fn list_chains(State(state): State<AppState>) -> impl IntoResponse {
    let chains = state.chain_manager.list_chains();
    Json(serde_json::json!({
        "chains": chains
    }))
}

// Execute Chain
pub async fn execute_chain(
    State(state): State<AppState>,
    Path(chain_id): Path<String>,
    Json(req): Json<ExecuteChainRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    state.metrics.record_request();
    state.metrics.record_chain_execution();
    
    let chain = state.chain_manager
        .get_chain(&chain_id)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Chain not found".to_string()))?;
    
    let input = ChainInput {
        variables: req.variables,
    };
    
    let output = chain
        .execute(input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    state.metrics.record_token_usage(output.metadata.total_tokens);
    
    Ok(Json(ExecuteChainResponse {
        result: output.result,
        execution_time_ms: output.metadata.execution_time_ms,
        total_tokens: output.metadata.total_tokens,
        total_cost: output.metadata.total_cost,
    }))
}

// Index Document (RAG)
pub async fn index_document(
    State(_state): State<AppState>,
    Json(req): Json<IndexDocumentRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // TODO: Implement document indexing
    Ok(Json(IndexDocumentResponse {
        document_id: uuid::Uuid::new_v4().to_string(),
        chunks_created: 10, // Mock value
    }))
}

// RAG Query
pub async fn rag_query(
    State(_state): State<AppState>,
    Json(req): Json<RAGQueryRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // TODO: Implement RAG query
    Ok(Json(RAGQueryResponse {
        answer: format!("Answer to: {}", req.query),
        context: vec!["Context 1".to_string(), "Context 2".to_string()],
        sources: vec!["doc1".to_string(), "doc2".to_string()],
    }))
}

// Agent Execute
pub async fn agent_execute(
    State(_state): State<AppState>,
    Json(req): Json<AgentExecuteRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // TODO: Implement agent execution
    Ok(Json(AgentExecuteResponse {
        final_answer: format!("Completed task: {}", req.task),
        steps: vec![],
        total_iterations: 1,
    }))
}

// Get Session
pub async fn get_session(
    State(_state): State<AppState>,
    Path(session_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // TODO: Implement session retrieval
    Ok(Json(SessionResponse {
        session_id,
        messages: vec![],
    }))
}

// Add to Session
pub async fn add_to_session(
    State(_state): State<AppState>,
    Path(session_id): Path<String>,
    Json(_req): Json<AddMessageRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // TODO: Implement add message
    Ok(Json(serde_json::json!({
        "session_id": session_id,
        "success": true
    })))
}

// Clear Session
pub async fn clear_session(
    State(_state): State<AppState>,
    Path(session_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // TODO: Implement clear session
    Ok(Json(serde_json::json!({
        "session_id": session_id,
        "success": true
    })))
}

// Metrics
pub async fn metrics(State(state): State<AppState>) -> impl IntoResponse {
    state.metrics.get_metrics()
}
