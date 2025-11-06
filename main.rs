use anyhow::Result;
use std::sync::Arc;
use tracing::info;

mod config;
mod llm;
mod embeddings;
mod memory;
mod rag;
mod chains;
mod agents;
mod monitoring;
mod api;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = config::AppConfig::load()?;
    let config = Arc::new(config);
    
    // Initialize logger
    monitoring::logger::init_logger(&config);
    info!("🦀 ChainForge - Enterprise LangChain in Rust");
    info!("======================================================");
    
    // Initialize metrics
    let metrics = Arc::new(monitoring::MetricsCollector::new());
    info!(" Metrics collector initialized");
    
    // Initialize LLM providers
    let provider_manager = Arc::new(llm::provider::ProviderManager::new(&config).await?);
    info!(" LLM providers initialized: {:?}", provider_manager.list_providers());
    
    // Initialize chain manager
    let chain_manager = Arc::new(chains::manager::ChainManager::new());
    info!(" Chain manager initialized");
    
    // Setup default chains
    setup_default_chains(&config, &provider_manager, &chain_manager).await?;
    
    // Create API state
    let app_state = api::AppState {
        config: config.clone(),
        provider_manager,
        chain_manager,
        metrics,
    };
    
    // Create router
    let app = api::create_router(app_state);
    
    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    info!("======================================================");
    info!("🚀 Server running on http://{}", addr);
    info!("📊 Metrics available at http://{}:{}/metrics", config.server.host, config.server.port);
    info!("======================================================");
    info!("");
    info!(" Available endpoints:");
    info!("  GET  /health               - Health check");
    info!("  GET  /status               - System status");
    info!("  POST /llm/generate         - Generate text");
    info!("  GET  /llm/providers        - List providers");
    info!("  GET  /chains               - List chains");
    info!("  POST /chains/:id/execute   - Execute chain");
    info!("  POST /rag/index            - Index document");
    info!("  POST /rag/query            - Query with RAG");
    info!("  POST /agent/execute        - Execute agent");
    info!("  GET  /metrics              - Prometheus metrics");
    info!("");
    info!("Ready to process requests!");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn setup_default_chains(
    config: &config::AppConfig,
    provider_manager: &llm::provider::ProviderManager,
    chain_manager: &chains::manager::ChainManager,
) -> Result<()> {
    // Create a simple Q&A chain
    let llm = provider_manager.get_provider(None)?;
    
    let qa_chain = chains::simple::SimpleChain::new(
        "qa_chain",
        "Simple question-answering chain",
        llm.clone(),
        "Answer the following question: {question}",
    );
    
    chain_manager.register_chain("qa", Arc::new(qa_chain));
    
    // Create a summarization chain
    let summarize_chain = chains::simple::SimpleChain::new(
        "summarize_chain",
        "Text summarization chain",
        llm.clone(),
        "Summarize the following text in 2-3 sentences:\n\n{text}",
    );
    
    chain_manager.register_chain("summarize", Arc::new(summarize_chain));
    
    info!("✅ Default chains registered: qa, summarize");
    
    Ok(())
}
