use super::Chain;
use anyhow::Result;
use dashmap::DashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

pub struct ChainManager {
    chains: Arc<DashMap<String, Arc<dyn Chain>>>,
}

impl ChainManager {
    pub fn new() -> Self {
        Self {
            chains: Arc::new(DashMap::new()),
        }
    }
    
    pub fn register_chain(&self, id: impl Into<String>, chain: Arc<dyn Chain>) {
        self.chains.insert(id.into(), chain);
        tracing::info!("Registered chain: {}", chain.name());
    }
    
    pub fn get_chain(&self, id: &str) -> Option<Arc<dyn Chain>> {
        self.chains.get(id).map(|entry| entry.value().clone())
    }
    
    pub fn list_chains(&self) -> Vec<ChainInfo> {
        self.chains
            .iter()
            .map(|entry| ChainInfo {
                id: entry.key().clone(),
                name: entry.value().name().to_string(),
                description: entry.value().description().to_string(),
            })
            .collect()
    }
    
    pub fn remove_chain(&self, id: &str) -> Option<Arc<dyn Chain>> {
        self.chains.remove(id).map(|(_, chain)| chain)
    }
    
    pub fn has_chain(&self, id: &str) -> bool {
        self.chains.contains_key(id)
    }
}

impl Default for ChainManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainInfo {
    pub id: String,
    pub name: String,
    pub description: String,
}
