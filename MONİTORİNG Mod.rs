use prometheus::{IntCounter, Histogram, Registry, Encoder, TextEncoder};
use lazy_static::lazy_static;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

pub mod logger;

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    
    pub static ref TOTAL_REQUESTS: IntCounter = IntCounter::new(
        "chainforge_total_requests",
        "Total number of requests"
    ).unwrap();
    
    pub static ref LLM_LATENCY: Histogram = Histogram::new(
        "chainforge_llm_latency_ms",
        "LLM request latency in milliseconds"
    ).unwrap();
    
    pub static ref CHAIN_EXECUTIONS: IntCounter = IntCounter::new(
        "chainforge_chain_executions",
        "Total chain executions"
    ).unwrap();
    
    pub static ref TOKEN_USAGE: IntCounter = IntCounter::new(
        "chainforge_tokens_used",
        "Total tokens used"
    ).unwrap();
}

pub struct MetricsCollector {
    pub total_requests: Arc<IntCounter>,
    pub llm_latency: Arc<Histogram>,
    pub chain_executions: Arc<IntCounter>,
    pub token_usage: Arc<IntCounter>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        REGISTRY.register(Box::new(TOTAL_REQUESTS.clone())).ok();
        REGISTRY.register(Box::new(LLM_LATENCY.clone())).ok();
        REGISTRY.register(Box::new(CHAIN_EXECUTIONS.clone())).ok();
        REGISTRY.register(Box::new(TOKEN_USAGE.clone())).ok();
        
        Self {
            total_requests: Arc::new(TOTAL_REQUESTS.clone()),
            llm_latency: Arc::new(LLM_LATENCY.clone()),
            chain_executions: Arc::new(CHAIN_EXECUTIONS.clone()),
            token_usage: Arc::new(TOKEN_USAGE.clone()),
        }
    }
    
    pub fn record_request(&self) {
        self.total_requests.inc();
    }
    
    pub fn record_llm_latency(&self, latency_ms: u64) {
        self.llm_latency.observe(latency_ms as f64);
    }
    
    pub fn record_chain_execution(&self) {
        self.chain_executions.inc();
    }
    
    pub fn record_token_usage(&self, tokens: usize) {
        self.token_usage.inc_by(tokens as u64);
    }
    
    pub fn get_metrics(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = REGISTRY.gather();
        let mut buffer = vec![];
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
    
    pub fn get_stats(&self) -> MetricsStats {
        MetricsStats {
            total_requests: self.total_requests.get(),
            total_chain_executions: self.chain_executions.get(),
            total_tokens_used: self.token_usage.get(),
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsStats {
    pub total_requests: u64,
    pub total_chain_executions: u64,
    pub total_tokens_used: u64,
}
